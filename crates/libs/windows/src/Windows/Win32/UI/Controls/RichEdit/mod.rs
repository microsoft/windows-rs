#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct IRichEditOle(::windows_core::IUnknown);
impl IRichEditOle {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetClientSite(&self) -> ::windows_core::Result<super::super::super::System::Ole::IOleClientSite> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClientSite)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObjectCount(&self) -> i32 {
        (::windows_core::Interface::vtable(self).GetObjectCount)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetLinkCount(&self) -> i32 {
        (::windows_core::Interface::vtable(self).GetLinkCount)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn GetObject(&self, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObject)(::windows_core::Interface::as_raw(self), iob, lpreobject, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn InsertObject(&self, lpreobject: *mut REOBJECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertObject)(::windows_core::Interface::as_raw(self), lpreobject).ok()
    }
    pub unsafe fn ConvertObject<P0>(&self, iob: i32, rclsidnew: *const ::windows_core::GUID, lpstrusertypenew: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).ConvertObject)(::windows_core::Interface::as_raw(self), iob, rclsidnew, lpstrusertypenew.into_param().abi()).ok()
    }
    pub unsafe fn ActivateAs(&self, rclsid: *const ::windows_core::GUID, rclsidas: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ActivateAs)(::windows_core::Interface::as_raw(self), rclsid, rclsidas).ok()
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, lpstrcontainerapp: P0, lpstrcontainerobj: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    {
        (::windows_core::Interface::vtable(self).SetHostNames)(::windows_core::Interface::as_raw(self), lpstrcontainerapp.into_param().abi(), lpstrcontainerobj.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLinkAvailable<P0>(&self, iob: i32, favailable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetLinkAvailable)(::windows_core::Interface::as_raw(self), iob, favailable.into_param().abi()).ok()
    }
    pub unsafe fn SetDvaspect(&self, iob: i32, dvaspect: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDvaspect)(::windows_core::Interface::as_raw(self), iob, dvaspect).ok()
    }
    pub unsafe fn HandsOffStorage(&self, iob: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HandsOffStorage)(::windows_core::Interface::as_raw(self), iob).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveCompleted<P0>(&self, iob: i32, lpstg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IStorage>,
    {
        (::windows_core::Interface::vtable(self).SaveCompleted)(::windows_core::Interface::as_raw(self), iob, lpstg.into_param().abi()).ok()
    }
    pub unsafe fn InPlaceDeactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InPlaceDeactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ContextSensitiveHelp)(::windows_core::Interface::as_raw(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClipboardData)(::windows_core::Interface::as_raw(self), lpchrg, reco, ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ImportDataObject<P0, P1>(&self, lpdataobj: P0, cf: u16, hmetapict: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IDataObject>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    {
        (::windows_core::Interface::vtable(self).ImportDataObject)(::windows_core::Interface::as_raw(self), lpdataobj.into_param().abi(), cf, hmetapict.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IRichEditOle, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IRichEditOle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOle {}
impl ::core::fmt::Debug for IRichEditOle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRichEditOle {
    type Vtable = IRichEditOle_Vtbl;
}
impl ::core::clone::Clone for IRichEditOle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRichEditOle {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020d00_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOle_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetClientSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpolesite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetClientSite: usize,
    pub GetObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
    pub GetLinkCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, lpreobject: *mut REOBJECT, dwflags: RICH_EDIT_GET_OBJECT_FLAGS) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    GetObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub InsertObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpreobject: *mut REOBJECT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole")))]
    InsertObject: usize,
    pub ConvertObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, rclsidnew: *const ::windows_core::GUID, lpstrusertypenew: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    pub ActivateAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rclsidas: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpstrcontainerapp: ::windows_core::PCSTR, lpstrcontainerobj: ::windows_core::PCSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLinkAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, favailable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLinkAvailable: usize,
    pub SetDvaspect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, dvaspect: u32) -> ::windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iob: i32, lpstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveCompleted: usize,
    pub InPlaceDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ContextSensitiveHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContextSensitiveHelp: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ImportDataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobj: *mut ::core::ffi::c_void, cf: u16, hmetapict: super::super::super::Foundation::HGLOBAL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ImportDataObject: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct IRichEditOleCallback(::windows_core::IUnknown);
impl IRichEditOleCallback {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetNewStorage(&self) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::IStorage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNewStorage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetInPlaceContext(&self, lplpframe: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceFrame>, lplpdoc: *mut ::core::option::Option<super::super::super::System::Ole::IOleInPlaceUIWindow>, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInPlaceContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lplpframe), ::core::mem::transmute(lplpdoc), lpframeinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowContainerUI<P0>(&self, fshow: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ShowContainerUI)(::windows_core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn QueryInsertObject<P0>(&self, lpclsid: *mut ::windows_core::GUID, lpstg: P0, cp: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::StructuredStorage::IStorage>,
    {
        (::windows_core::Interface::vtable(self).QueryInsertObject)(::windows_core::Interface::as_raw(self), lpclsid, lpstg.into_param().abi(), cp).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn DeleteObject<P0>(&self, lpoleobj: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Ole::IOleObject>,
    {
        (::windows_core::Interface::vtable(self).DeleteObject)(::windows_core::Interface::as_raw(self), lpoleobj.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn QueryAcceptData<P0, P1, P2>(&self, lpdataobj: P0, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: P1, hmetapict: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Com::IDataObject>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::super::Foundation::HGLOBAL>,
    {
        (::windows_core::Interface::vtable(self).QueryAcceptData)(::windows_core::Interface::as_raw(self), lpdataobj.into_param().abi(), lpcfformat, reco, freally.into_param().abi(), hmetapict.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ContextSensitiveHelp)(::windows_core::Interface::as_raw(self), fentermode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClipboardData)(::windows_core::Interface::as_raw(self), lpchrg, reco, ::core::mem::transmute(lplpdataobj)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_SystemServices\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices"))]
    pub unsafe fn GetDragDropEffect<P0>(&self, fdrag: P0, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).GetDragDropEffect)(::windows_core::Interface::as_raw(self), fdrag.into_param().abi(), grfkeystate, pdweffect).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetContextMenu<P0>(&self, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: P0, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::System::Ole::IOleObject>,
    {
        (::windows_core::Interface::vtable(self).GetContextMenu)(::windows_core::Interface::as_raw(self), seltype, lpoleobj.into_param().abi(), lpchrg, lphmenu).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IRichEditOleCallback, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IRichEditOleCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichEditOleCallback {}
impl ::core::fmt::Debug for IRichEditOleCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichEditOleCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRichEditOleCallback {
    type Vtable = IRichEditOleCallback_Vtbl;
}
impl ::core::clone::Clone for IRichEditOleCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRichEditOleCallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00020d03_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditOleCallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetNewStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetNewStorage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetInPlaceContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lplpframe: *mut *mut ::core::ffi::c_void, lplpdoc: *mut *mut ::core::ffi::c_void, lpframeinfo: *mut super::super::super::System::Ole::OLEINPLACEFRAMEINFO) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))]
    GetInPlaceContext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowContainerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowContainerUI: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub QueryInsertObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows_core::GUID, lpstg: *mut ::core::ffi::c_void, cp: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    QueryInsertObject: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub DeleteObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpoleobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    DeleteObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub QueryAcceptData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobj: *mut ::core::ffi::c_void, lpcfformat: *mut u16, reco: super::super::super::System::SystemServices::RECO_FLAGS, freally: super::super::super::Foundation::BOOL, hmetapict: super::super::super::Foundation::HGLOBAL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    QueryAcceptData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ContextSensitiveHelp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fentermode: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ContextSensitiveHelp: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, reco: u32, lplpdataobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices"))]
    pub GetDragDropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdrag: super::super::super::Foundation::BOOL, grfkeystate: super::super::super::System::SystemServices::MODIFIERKEYS_FLAGS, pdweffect: *mut super::super::super::System::Ole::DROPEFFECT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_SystemServices")))]
    GetDragDropEffect: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seltype: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE, lpoleobj: *mut ::core::ffi::c_void, lpchrg: *mut CHARRANGE, lphmenu: *mut super::super::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging")))]
    GetContextMenu: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct IRicheditUiaOverrides(::windows_core::IUnknown);
impl IRicheditUiaOverrides {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: i32, pretvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyOverrideValue)(::windows_core::Interface::as_raw(self), propertyid, pretvalue).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IRicheditUiaOverrides, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for IRicheditUiaOverrides {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRicheditUiaOverrides {}
impl ::core::fmt::Debug for IRicheditUiaOverrides {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRicheditUiaOverrides").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRicheditUiaOverrides {
    type Vtable = IRicheditUiaOverrides_Vtbl;
}
impl ::core::clone::Clone for IRicheditUiaOverrides {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRicheditUiaOverrides {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverrides_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPropertyOverrideValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: i32, pretvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPropertyOverrideValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextDisplays(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextDisplays {}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextDisplays, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDisplays {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDisplays {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDisplays {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDisplays").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextDisplays {
    type Vtable = ITextDisplays_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextDisplays {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextDisplays {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5f2_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDisplays_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextDocument(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows_core::Result<ITextSelection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSelection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges(&self) -> ::windows_core::Result<ITextStoryRanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryRanges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSaved(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSaved)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSaved)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultTabStop)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDefaultTabStop)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn New(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).New)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Open)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    pub unsafe fn Freeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Freeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unfreeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Unfreeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Undo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Undo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    pub unsafe fn Redo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Redo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Range)(::windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RangeFromPoint)(::windows_core::Interface::as_raw(self), x, y, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextDocument, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextDocument {
    type Vtable = ITextDocument_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c0_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection: usize,
    pub GetStoryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStoryRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStoryRanges: usize,
    pub GetSaved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSaved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows_core::HRESULT,
    pub GetDefaultTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetDefaultTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub New: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Open: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Save: usize,
    pub Freeze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub Unfreeze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub BeginEditCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndEditCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Undo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub Redo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Range: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Range: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RangeFromPoint: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextDocument2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2 {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows_core::Result<ITextSelection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSelection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges(&self) -> ::windows_core::Result<ITextStoryRanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryRanges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSaved(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSaved)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSaved)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultTabStop)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDefaultTabStop)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn New(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.New)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Save)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    pub unsafe fn Freeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Freeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unfreeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Unfreeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Undo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Undo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    pub unsafe fn Redo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Redo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Range)(::windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RangeFromPoint)(::windows_core::Interface::as_raw(self), x, y, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCaretType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCaretType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCaretType(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCaretType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDisplays(&self) -> ::windows_core::Result<ITextDisplays> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplays)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentFont(&self) -> ::windows_core::Result<ITextFont2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentFont<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont2>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentFont)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentPara(&self) -> ::windows_core::Result<ITextPara2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDocumentPara<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara2>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentPara)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetEastAsianFlags(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEastAsianFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGenerator(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGenerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIMEInProgress(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIMEInProgress)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetNotificationMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNotificationMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotificationMode(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNotificationMode)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection2(&self) -> ::windows_core::Result<ITextSelection2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSelection2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges2(&self) -> ::windows_core::Result<ITextStoryRanges2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryRanges2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTypographyOptions(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTypographyOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWindow(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWindow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AttachMsgFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckTextLimit)(::windows_core::Interface::as_raw(self), cch, pcch).ok()
    }
    pub unsafe fn GetCallManager(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCallManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClientRect(&self, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientRect)(::windows_core::Interface::as_raw(self), r#type, pleft, ptop, pright, pbottom).ok()
    }
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEffectColor)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImmContext(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImmContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut ::windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPreferredFont)(::windows_core::Interface::as_raw(self), cp, charrep, options, curcharrep, curfontsize, ::core::mem::transmute(pbstr), ppitchandfamily, pnewfontsize).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStrings(&self) -> ::windows_core::Result<ITextStrings> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrings)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Notify(&self, notify: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), notify).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range2(&self, cpactive: i32, cpanchor: i32) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Range2)(::windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint2(&self, x: i32, y: i32, r#type: i32) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RangeFromPoint2)(::windows_core::Interface::as_raw(self), x, y, r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).ReleaseCallManager)(::windows_core::Interface::as_raw(self), pvoid.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseImmContext)(::windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn SetEffectColor(&self, index: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEffectColor)(::windows_core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
    pub unsafe fn SetTypographyOptions(&self, options: i32, mask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTypographyOptions)(::windows_core::Interface::as_raw(self), options, mask).ok()
    }
    pub unsafe fn SysBeep(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SysBeep)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn UpdateWindow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateWindow)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMathProperties(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMathProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMathProperties(&self, options: i32, mask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMathProperties)(::windows_core::Interface::as_raw(self), options, mask).ok()
    }
    pub unsafe fn GetActiveStory(&self) -> ::windows_core::Result<ITextStory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetActiveStory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetActiveStory<P0>(&self, pstory: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextStory>,
    {
        (::windows_core::Interface::vtable(self).SetActiveStory)(::windows_core::Interface::as_raw(self), pstory.into_param().abi()).ok()
    }
    pub unsafe fn GetMainStory(&self) -> ::windows_core::Result<ITextStory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMainStory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNewStory(&self) -> ::windows_core::Result<ITextStory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNewStory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStory(&self, index: i32) -> ::windows_core::Result<ITextStory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStory)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextDocument2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextDocument);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextDocument2 {
    type Vtable = ITextDocument2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextDocument2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextDocument2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e0_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub GetCaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDisplays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisplays: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDisplays: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentFont: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentFont: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentPara: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDocumentPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDocumentPara: usize,
    pub GetEastAsianFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut tomConstants) -> ::windows_core::HRESULT,
    pub GetGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetIMEInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStoryRanges2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstories: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStoryRanges2: usize,
    pub GetTypographyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i64) -> ::windows_core::HRESULT,
    pub AttachMsgFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows_core::HRESULT,
    pub GetCallManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: tomConstants, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::HRESULT,
    pub GetEffectColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, options: i32, curcharrep: i32, curfontsize: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStrings: usize,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Range2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Range2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RangeFromPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RangeFromPoint2: usize,
    pub ReleaseCallManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: i64) -> ::windows_core::HRESULT,
    pub SetEffectColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
    pub SetTypographyOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMathProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut i32) -> ::windows_core::HRESULT,
    pub SetMathProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: i32, mask: i32) -> ::windows_core::HRESULT,
    pub GetActiveStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetActiveStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMainStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNewStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppstory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextDocument2Old(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextDocument2Old {
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection(&self) -> ::windows_core::Result<ITextSelection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSelection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStoryRanges(&self) -> ::windows_core::Result<ITextStoryRanges> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryRanges)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSaved(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSaved)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSaved(&self, value: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSaved)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetDefaultTabStop(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDefaultTabStop)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTabStop(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDefaultTabStop)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn New(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.New)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Open(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Open)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save(&self, pvar: *const super::super::super::System::Variant::VARIANT, flags: tomConstants, codepage: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Save)(::windows_core::Interface::as_raw(self), pvar, flags, codepage).ok()
    }
    pub unsafe fn Freeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Freeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unfreeze(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Unfreeze)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BeginEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EndEditCollection(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndEditCollection)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Undo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Undo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    pub unsafe fn Redo(&self, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Redo)(::windows_core::Interface::as_raw(self), count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Range(&self, cpactive: i32, cpanchor: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Range)(::windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RangeFromPoint(&self, x: i32, y: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RangeFromPoint)(::windows_core::Interface::as_raw(self), x, y, &mut result__).from_abi(result__)
    }
    pub unsafe fn AttachMsgFilter<P0>(&self, pfilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).AttachMsgFilter)(::windows_core::Interface::as_raw(self), pfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEffectColor<P0>(&self, index: i32, cr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::COLORREF>,
    {
        (::windows_core::Interface::vtable(self).SetEffectColor)(::windows_core::Interface::as_raw(self), index, cr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectColor(&self, index: i32) -> ::windows_core::Result<super::super::super::Foundation::COLORREF> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEffectColor)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCaretType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCaretType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCaretType(&self, carettype: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCaretType)(::windows_core::Interface::as_raw(self), carettype).ok()
    }
    pub unsafe fn GetImmContext(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImmContext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseImmContext(&self, context: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseImmContext)(::windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn GetPreferredFont(&self, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut ::windows_core::BSTR, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPreferredFont)(::windows_core::Interface::as_raw(self), cp, charrep, option, charrepcur, curfontsize, ::core::mem::transmute(pbstr), ppitchandfamily, pnewfontsize).ok()
    }
    pub unsafe fn GetNotificationMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNotificationMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotificationMode(&self, mode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNotificationMode)(::windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn GetClientRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetClientRect)(::windows_core::Interface::as_raw(self), r#type, pleft, ptop, pright, pbottom).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelection2(&self) -> ::windows_core::Result<ITextSelection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSelection2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWindow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWindow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFEFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFEFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateWindow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateWindow)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CheckTextLimit(&self, cch: i32, pcch: *const i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CheckTextLimit)(::windows_core::Interface::as_raw(self), cch, pcch).ok()
    }
    pub unsafe fn IMEInProgress(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IMEInProgress)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SysBeep(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SysBeep)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Update(&self, mode: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Update)(::windows_core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn Notify(&self, notify: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Notify)(::windows_core::Interface::as_raw(self), notify).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentFont(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentPara(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCallManager(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCallManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseCallManager<P0>(&self, pvoid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).ReleaseCallManager)(::windows_core::Interface::as_raw(self), pvoid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextDocument2Old, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextDocument);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextDocument2Old {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextDocument2Old {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextDocument2Old {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextDocument2Old").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextDocument2Old {
    type Vtable = ITextDocument2Old_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextDocument2Old {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextDocument2Old {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01c25500_4268_11d1_883a_3c8b00c10000);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2Old_Vtbl {
    pub base__: ITextDocument_Vtbl,
    pub AttachMsgFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEffectColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, cr: super::super::super::Foundation::COLORREF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEffectColor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pcr: *mut super::super::super::Foundation::COLORREF) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectColor: usize,
    pub GetCaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcarettype: *mut i32) -> ::windows_core::HRESULT,
    pub SetCaretType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, carettype: i32) -> ::windows_core::HRESULT,
    pub GetImmContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut i64) -> ::windows_core::HRESULT,
    pub ReleaseImmContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: i64) -> ::windows_core::HRESULT,
    pub GetPreferredFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp: i32, charrep: i32, option: i32, charrepcur: i32, curfontsize: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitchandfamily: *mut i32, pnewfontsize: *mut i32) -> ::windows_core::HRESULT,
    pub GetNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmode: *mut i32) -> ::windows_core::HRESULT,
    pub SetNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows_core::HRESULT,
    pub GetClientRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSelection2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSelection2: usize,
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut i32) -> ::windows_core::HRESULT,
    pub GetFEFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT,
    pub UpdateWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CheckTextLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: i32, pcch: *const i32) -> ::windows_core::HRESULT,
    pub IMEInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SysBeep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: i32) -> ::windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notify: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitextfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentFont: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppitextpara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentPara: usize,
    pub GetCallManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvoid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseCallManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoid: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextFont(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextFont {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).SetDuplicate)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CanChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, pfont: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), pfont.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStyle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStyle)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAllCaps(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAllCaps)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllCaps)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAnimation(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAnimation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAnimation)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetBackColor(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackColor)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetBold(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBold)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBold(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBold)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmboss(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEmboss)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEmboss)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetForeColor(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetForeColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetForeColor)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHidden(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHidden)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEngrave(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEngrave)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEngrave)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetItalic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetItalic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetItalic)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKerning(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKerning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKerning)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetLanguageID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguageID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLanguageID)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetOutline(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutline)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetPosition(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPosition)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetProtected(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProtected)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProtected)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetShadow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetShadow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetShadow)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSize(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSize)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSmallCaps(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSmallCaps)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSmallCaps)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpacing(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpacing)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpacing)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStrikeThrough(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrikeThrough)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrikeThrough)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSubscript(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubscript)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSubscript)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSuperscript(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSuperscript)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSuperscript)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetUnderline(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUnderline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnderline)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetWeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWeight)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWeight)(::windows_core::Interface::as_raw(self), value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextFont, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextFont {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextFont {
    type Vtable = ITextFont_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextFont {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c3_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDuplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDuplicate: usize,
    pub CanChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: tomConstants) -> ::windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAllCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAllCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetBackColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetEmboss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetEmboss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetForeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetEngrave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetEngrave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetOutline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetOutline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetShadow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetSmallCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSmallCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetStrikeThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetStrikeThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextFont2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextFont2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDuplicate)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, pfont: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsEqual)(::windows_core::Interface::as_raw(self), pfont.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reset)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStyle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStyle)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAllCaps(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAllCaps)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAllCaps(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllCaps)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAnimation(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAnimation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAnimation(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAnimation)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetBackColor(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBackColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackColor(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBackColor)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetBold(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBold)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBold(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBold)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmboss(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEmboss)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEmboss(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEmboss)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetForeColor(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetForeColor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetForeColor(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetForeColor)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHidden(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHidden)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHidden(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetHidden)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEngrave(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEngrave)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEngrave(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEngrave)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetItalic(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetItalic)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetItalic(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetItalic)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKerning(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetKerning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKerning(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetKerning)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetLanguageID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguageID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguageID(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLanguageID)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetOutline(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOutline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOutline(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOutline)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetPosition(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPosition)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPosition(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPosition)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetProtected(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProtected)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProtected(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProtected)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetShadow(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetShadow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetShadow(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetShadow)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSize(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSize)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSmallCaps(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSmallCaps)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSmallCaps(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSmallCaps)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpacing(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSpacing)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpacing(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSpacing)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStrikeThrough(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStrikeThrough)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrikeThrough(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStrikeThrough)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSubscript(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSubscript)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubscript(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSubscript)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSuperscript(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSuperscript)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSuperscript(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSuperscript)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetUnderline(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetUnderline)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUnderline(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUnderline)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetWeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWeight)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWeight(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWeight)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAutoLigatures(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAutoLigatures)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutoLigatures(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoLigatures)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAutospaceAlpha(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAutospaceAlpha)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutospaceAlpha(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutospaceAlpha)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAutospaceNumeric(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAutospaceNumeric)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutospaceNumeric(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutospaceNumeric)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAutospaceParens(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAutospaceParens)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAutospaceParens(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutospaceParens)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCharRep(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCharRep)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCharRep(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCharRep)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCompressionMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCompressionMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCompressionMode(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCompressionMode)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCookie(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCookie)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCookie(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCookie)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetDoubleStrike(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDoubleStrike)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDoubleStrike(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDoubleStrike)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate2(&self) -> ::windows_core::Result<ITextFont2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate2<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont2>,
    {
        (::windows_core::Interface::vtable(self).SetDuplicate2)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    pub unsafe fn GetLinkType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLinkType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMathZone(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMathZone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMathZone(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMathZone)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetModWidthPairs(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetModWidthPairs)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetModWidthPairs(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModWidthPairs)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetModWidthSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetModWidthSpace)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetModWidthSpace(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModWidthSpace)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetOldNumbers(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOldNumbers)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOldNumbers(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOldNumbers)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetOverlapping(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOverlapping)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOverlapping(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOverlapping)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetPositionSubSuper(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPositionSubSuper)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPositionSubSuper(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPositionSubSuper)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetScaling(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetScaling)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetScaling(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScaling)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpaceExtension(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpaceExtension)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpaceExtension(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpaceExtension)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetUnderlinePositionMode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUnderlinePositionMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUnderlinePositionMode(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUnderlinePositionMode)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffects)(::windows_core::Interface::as_raw(self), pvalue, pmask).ok()
    }
    pub unsafe fn GetEffects2(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffects2)(::windows_core::Interface::as_raw(self), pvalue, pmask).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyInfo(&self, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertyInfo)(::windows_core::Interface::as_raw(self), index, ptype, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual2<P0>(&self, pfont: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextFont2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual2)(::windows_core::Interface::as_raw(self), pfont.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEffects)(::windows_core::Interface::as_raw(self), value, mask).ok()
    }
    pub unsafe fn SetEffects2(&self, value: i32, mask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEffects2)(::windows_core::Interface::as_raw(self), value, mask).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextFont2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextFont);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextFont2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextFont2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextFont2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextFont2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextFont2 {
    type Vtable = ITextFont2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextFont2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextFont2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e3_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextFont2_Vtbl {
    pub base__: ITextFont_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub GetAutoLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAutoLigatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAutospaceAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAutospaceAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAutospaceNumeric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAutospaceNumeric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAutospaceParens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAutospaceParens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCharRep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCharRep: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCompressionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCompressionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetDoubleStrike: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetDoubleStrike: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDuplicate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDuplicate2: usize,
    pub GetLinkType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetMathZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetMathZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetModWidthPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetModWidthPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetModWidthSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetModWidthSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetOldNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetOldNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetOverlapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetOverlapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetPositionSubSuper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetPositionSubSuper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetScaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSpaceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpaceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetUnderlinePositionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetUnderlinePositionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::HRESULT,
    pub GetEffects2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ptype: *mut i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void, pb: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual2: usize,
    pub SetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows_core::HRESULT,
    pub SetEffects2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct ITextHost(::windows_core::IUnknown);
impl ITextHost {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        (::windows_core::Interface::vtable(self).TxGetDC)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC<P0>(&self, hdc: P0) -> i32
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxReleaseDC)(::windows_core::Interface::as_raw(self), hdc.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowScrollBar<P0>(&self, fnbar: i32, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxShowScrollBar)(::windows_core::Interface::as_raw(self), fnbar, fshow.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxEnableScrollBar)(::windows_core::Interface::as_raw(self), fusbflags, fuarrowflags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollRange<P0>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxSetScrollRange)(::windows_core::Interface::as_raw(self), fnbar, nminpos, nmaxpos, fredraw.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollPos<P0>(&self, fnbar: i32, npos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxSetScrollPos)(::windows_core::Interface::as_raw(self), fnbar, npos, fredraw.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxInvalidateRect<P0>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxInvalidateRect)(::windows_core::Interface::as_raw(self), prc, fmode.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxViewChange<P0>(&self, fupdate: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxViewChange)(::windows_core::Interface::as_raw(self), fupdate.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxCreateCaret<P0>(&self, hbmp: P0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows_core::Interface::vtable(self).TxCreateCaret)(::windows_core::Interface::as_raw(self), hbmp.into_param().abi(), xwidth, yheight)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowCaret<P0>(&self, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxShowCaret)(::windows_core::Interface::as_raw(self), fshow.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxSetCaretPos)(::windows_core::Interface::as_raw(self), x, y)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxSetTimer)(::windows_core::Interface::as_raw(self), idtimer, utimeout)
    }
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        (::windows_core::Interface::vtable(self).TxKillTimer)(::windows_core::Interface::as_raw(self), idtimer)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx<P0>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: P0, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS)
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HRGN>,
    {
        (::windows_core::Interface::vtable(self).TxScrollWindowEx)(::windows_core::Interface::as_raw(self), dx, dy, lprcscroll, lprcclip, hrgnupdate.into_param().abi(), lprcupdate, fuscroll)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCapture<P0>(&self, fcapture: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxSetCapture)(::windows_core::Interface::as_raw(self), fcapture.into_param().abi())
    }
    pub unsafe fn TxSetFocus(&self) {
        (::windows_core::Interface::vtable(self).TxSetFocus)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor<P0, P1>(&self, hcur: P0, ftext: P1)
    where
        P0: ::windows_core::IntoParam<super::super::WindowsAndMessaging::HCURSOR>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxSetCursor)(::windows_core::Interface::as_raw(self), hcur.into_param().abi(), ftext.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxScreenToClient)(::windows_core::Interface::as_raw(self), lppt)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxClientToScreen)(::windows_core::Interface::as_raw(self), lppt)
    }
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxActivate)(::windows_core::Interface::as_raw(self), ploldstate).ok()
    }
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxDeactivate)(::windows_core::Interface::as_raw(self), lnewstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetClientRect)(::windows_core::Interface::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetViewInset)(::windows_core::Interface::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetCharFormat)(::windows_core::Interface::as_raw(self), ppcf).ok()
    }
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetParaFormat)(::windows_core::Interface::as_raw(self), pppf).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
        (::windows_core::Interface::vtable(self).TxGetSysColor)(::windows_core::Interface::as_raw(self), nindex)
    }
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetBackStyle)(::windows_core::Interface::as_raw(self), pstyle).ok()
    }
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetMaxLength)(::windows_core::Interface::as_raw(self), plength).ok()
    }
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetScrollBars)(::windows_core::Interface::as_raw(self), pdwscrollbar).ok()
    }
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows_core::Result<i8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TxGetPasswordChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetAcceleratorPos)(::windows_core::Interface::as_raw(self), pcp).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetExtent)(::windows_core::Interface::as_raw(self), lpextent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxCharFormatChange)(::windows_core::Interface::as_raw(self), pcf).ok()
    }
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxParaFormatChange)(::windows_core::Interface::as_raw(self), ppf).ok()
    }
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetPropertyBits)(::windows_core::Interface::as_raw(self), dwmask, pdwbits).ok()
    }
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxNotify)(::windows_core::Interface::as_raw(self), inotify, pv).ok()
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        (::windows_core::Interface::vtable(self).TxImmGetContext)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmReleaseContext<P0>(&self, himc: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).TxImmReleaseContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi())
    }
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetSelectionBarWidth)(::windows_core::Interface::as_raw(self), lselbarwidth).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITextHost, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for ITextHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost {}
impl ::core::fmt::Debug for ITextHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextHost {
    type Vtable = ITextHost_Vtbl;
}
impl ::core::clone::Clone for ITextHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextHost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::super::super::Graphics::Gdi::HDC) -> i32,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxReleaseDC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxShowScrollBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxShowScrollBar: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TxEnableScrollBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TxEnableScrollBar: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSetScrollRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSetScrollRange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSetScrollPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fnbar: i32, npos: i32, fredraw: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSetScrollPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxInvalidateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT, fmode: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    TxInvalidateRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fupdate: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    TxViewChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub TxCreateCaret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbmp: super::super::super::Graphics::Gdi::HBITMAP, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    TxCreateCaret: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxShowCaret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxShowCaret: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSetCaretPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSetCaretPos: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSetTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSetTimer: usize,
    pub TxKillTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idtimer: u32),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TxScrollWindowEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: super::super::super::Graphics::Gdi::HRGN, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    TxScrollWindowEx: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSetCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcapture: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSetCapture: usize,
    pub TxSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TxSetCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, ftext: super::super::super::Foundation::BOOL),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TxSetCursor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxScreenToClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxScreenToClient: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxClientToScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxClientToScreen: usize,
    pub TxActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploldstate: *mut i32) -> ::windows_core::HRESULT,
    pub TxDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnewstate: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetClientRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetClientRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetViewInset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetViewInset: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub TxGetCharFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcf: *const *const CHARFORMATW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    TxGetCharFormat: usize,
    pub TxGetParaFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppf: *const *const PARAFORMAT) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub TxGetSysColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    TxGetSysColor: usize,
    pub TxGetBackStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstyle: *mut TXTBACKSTYLE) -> ::windows_core::HRESULT,
    pub TxGetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plength: *mut u32) -> ::windows_core::HRESULT,
    pub TxGetScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscrollbar: *mut u32) -> ::windows_core::HRESULT,
    pub TxGetPasswordChar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pch: *mut i8) -> ::windows_core::HRESULT,
    pub TxGetAcceleratorPos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcp: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetExtent: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub OnTxCharFormatChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcf: *const CHARFORMATW) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    OnTxCharFormatChange: usize,
    pub OnTxParaFormatChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppf: *const PARAFORMAT) -> ::windows_core::HRESULT,
    pub TxGetPropertyBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, pdwbits: *mut u32) -> ::windows_core::HRESULT,
    pub TxNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Globalization")]
    pub TxImmGetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Globalization::HIMC,
    #[cfg(not(feature = "Win32_Globalization"))]
    TxImmGetContext: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub TxImmReleaseContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC),
    #[cfg(not(feature = "Win32_Globalization"))]
    TxImmReleaseContext: usize,
    pub TxGetSelectionBarWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lselbarwidth: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct ITextHost2(::windows_core::IUnknown);
impl ITextHost2 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetDC(&self) -> super::super::super::Graphics::Gdi::HDC {
        (::windows_core::Interface::vtable(self).base__.TxGetDC)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxReleaseDC<P0>(&self, hdc: P0) -> i32
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.TxReleaseDC)(::windows_core::Interface::as_raw(self), hdc.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowScrollBar<P0>(&self, fnbar: i32, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxShowScrollBar)(::windows_core::Interface::as_raw(self), fnbar, fshow.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxEnableScrollBar(&self, fusbflags: super::super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, fuarrowflags: i32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.TxEnableScrollBar)(::windows_core::Interface::as_raw(self), fusbflags, fuarrowflags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollRange<P0>(&self, fnbar: i32, nminpos: i32, nmaxpos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSetScrollRange)(::windows_core::Interface::as_raw(self), fnbar, nminpos, nmaxpos, fredraw.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetScrollPos<P0>(&self, fnbar: i32, npos: i32, fredraw: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSetScrollPos)(::windows_core::Interface::as_raw(self), fnbar, npos, fredraw.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxInvalidateRect<P0>(&self, prc: *mut super::super::super::Foundation::RECT, fmode: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxInvalidateRect)(::windows_core::Interface::as_raw(self), prc, fmode.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxViewChange<P0>(&self, fupdate: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxViewChange)(::windows_core::Interface::as_raw(self), fupdate.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxCreateCaret<P0>(&self, hbmp: P0, xwidth: i32, yheight: i32) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows_core::Interface::vtable(self).base__.TxCreateCaret)(::windows_core::Interface::as_raw(self), hbmp.into_param().abi(), xwidth, yheight)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxShowCaret<P0>(&self, fshow: P0) -> super::super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxShowCaret)(::windows_core::Interface::as_raw(self), fshow.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCaretPos(&self, x: i32, y: i32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.TxSetCaretPos)(::windows_core::Interface::as_raw(self), x, y)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetTimer(&self, idtimer: u32, utimeout: u32) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.TxSetTimer)(::windows_core::Interface::as_raw(self), idtimer, utimeout)
    }
    pub unsafe fn TxKillTimer(&self, idtimer: u32) {
        (::windows_core::Interface::vtable(self).base__.TxKillTimer)(::windows_core::Interface::as_raw(self), idtimer)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxScrollWindowEx<P0>(&self, dx: i32, dy: i32, lprcscroll: *mut super::super::super::Foundation::RECT, lprcclip: *mut super::super::super::Foundation::RECT, hrgnupdate: P0, lprcupdate: *mut super::super::super::Foundation::RECT, fuscroll: super::super::WindowsAndMessaging::SCROLL_WINDOW_FLAGS)
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HRGN>,
    {
        (::windows_core::Interface::vtable(self).base__.TxScrollWindowEx)(::windows_core::Interface::as_raw(self), dx, dy, lprcscroll, lprcclip, hrgnupdate.into_param().abi(), lprcupdate, fuscroll)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSetCapture<P0>(&self, fcapture: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSetCapture)(::windows_core::Interface::as_raw(self), fcapture.into_param().abi())
    }
    pub unsafe fn TxSetFocus(&self) {
        (::windows_core::Interface::vtable(self).base__.TxSetFocus)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor<P0, P1>(&self, hcur: P0, ftext: P1)
    where
        P0: ::windows_core::IntoParam<super::super::WindowsAndMessaging::HCURSOR>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSetCursor)(::windows_core::Interface::as_raw(self), hcur.into_param().abi(), ftext.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxScreenToClient(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.TxScreenToClient)(::windows_core::Interface::as_raw(self), lppt)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxClientToScreen(&self, lppt: *mut super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).base__.TxClientToScreen)(::windows_core::Interface::as_raw(self), lppt)
    }
    pub unsafe fn TxActivate(&self, ploldstate: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxActivate)(::windows_core::Interface::as_raw(self), ploldstate).ok()
    }
    pub unsafe fn TxDeactivate(&self, lnewstate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxDeactivate)(::windows_core::Interface::as_raw(self), lnewstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetClientRect(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetClientRect)(::windows_core::Interface::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetViewInset(&self, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetViewInset)(::windows_core::Interface::as_raw(self), prc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetCharFormat(&self, ppcf: *const *const CHARFORMATW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetCharFormat)(::windows_core::Interface::as_raw(self), ppcf).ok()
    }
    pub unsafe fn TxGetParaFormat(&self, pppf: *const *const PARAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetParaFormat)(::windows_core::Interface::as_raw(self), pppf).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxGetSysColor(&self, nindex: super::super::super::Graphics::Gdi::SYS_COLOR_INDEX) -> super::super::super::Foundation::COLORREF {
        (::windows_core::Interface::vtable(self).base__.TxGetSysColor)(::windows_core::Interface::as_raw(self), nindex)
    }
    pub unsafe fn TxGetBackStyle(&self, pstyle: *mut TXTBACKSTYLE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetBackStyle)(::windows_core::Interface::as_raw(self), pstyle).ok()
    }
    pub unsafe fn TxGetMaxLength(&self, plength: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetMaxLength)(::windows_core::Interface::as_raw(self), plength).ok()
    }
    pub unsafe fn TxGetScrollBars(&self, pdwscrollbar: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetScrollBars)(::windows_core::Interface::as_raw(self), pdwscrollbar).ok()
    }
    pub unsafe fn TxGetPasswordChar(&self) -> ::windows_core::Result<i8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TxGetPasswordChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TxGetAcceleratorPos(&self, pcp: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetAcceleratorPos)(::windows_core::Interface::as_raw(self), pcp).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetExtent(&self, lpextent: *mut super::super::super::Foundation::SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetExtent)(::windows_core::Interface::as_raw(self), lpextent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn OnTxCharFormatChange(&self, pcf: *const CHARFORMATW) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxCharFormatChange)(::windows_core::Interface::as_raw(self), pcf).ok()
    }
    pub unsafe fn OnTxParaFormatChange(&self, ppf: *const PARAFORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxParaFormatChange)(::windows_core::Interface::as_raw(self), ppf).ok()
    }
    pub unsafe fn TxGetPropertyBits(&self, dwmask: u32, pdwbits: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetPropertyBits)(::windows_core::Interface::as_raw(self), dwmask, pdwbits).ok()
    }
    pub unsafe fn TxNotify(&self, inotify: u32, pv: *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxNotify)(::windows_core::Interface::as_raw(self), inotify, pv).ok()
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmGetContext(&self) -> super::super::super::Globalization::HIMC {
        (::windows_core::Interface::vtable(self).base__.TxImmGetContext)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Globalization\"`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn TxImmReleaseContext<P0>(&self, himc: P0)
    where
        P0: ::windows_core::IntoParam<super::super::super::Globalization::HIMC>,
    {
        (::windows_core::Interface::vtable(self).base__.TxImmReleaseContext)(::windows_core::Interface::as_raw(self), himc.into_param().abi())
    }
    pub unsafe fn TxGetSelectionBarWidth(&self, lselbarwidth: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetSelectionBarWidth)(::windows_core::Interface::as_raw(self), lselbarwidth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxIsDoubleClickPending(&self) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).TxIsDoubleClickPending)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetWindow(&self, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetWindow)(::windows_core::Interface::as_raw(self), phwnd).ok()
    }
    pub unsafe fn TxSetForegroundWindow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxSetForegroundWindow)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn TxGetPalette(&self) -> super::super::super::Graphics::Gdi::HPALETTE {
        (::windows_core::Interface::vtable(self).TxGetPalette)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn TxGetEastAsianFlags(&self, pflags: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetEastAsianFlags)(::windows_core::Interface::as_raw(self), pflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TxSetCursor2<P0, P1>(&self, hcur: P0, btext: P1) -> super::super::WindowsAndMessaging::HCURSOR
    where
        P0: ::windows_core::IntoParam<super::super::WindowsAndMessaging::HCURSOR>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).TxSetCursor2)(::windows_core::Interface::as_raw(self), hcur.into_param().abi(), btext.into_param().abi())
    }
    pub unsafe fn TxFreeTextServicesNotification(&self) {
        (::windows_core::Interface::vtable(self).TxFreeTextServicesNotification)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn TxGetEditStyle(&self, dwitem: u32, pdwdata: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetEditStyle)(::windows_core::Interface::as_raw(self), dwitem, pdwdata).ok()
    }
    pub unsafe fn TxGetWindowStyles(&self, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetWindowStyles)(::windows_core::Interface::as_raw(self), pdwstyle, pdwexstyle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn TxShowDropCaret<P0, P1>(&self, fshow: P0, hdc: P1, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxShowDropCaret)(::windows_core::Interface::as_raw(self), fshow.into_param().abi(), hdc.into_param().abi(), prc).ok()
    }
    pub unsafe fn TxDestroyCaret(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxDestroyCaret)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TxGetHorzExtent(&self, plhorzextent: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetHorzExtent)(::windows_core::Interface::as_raw(self), plhorzextent).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITextHost2, ::windows_core::IUnknown, ITextHost);
impl ::core::cmp::PartialEq for ITextHost2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextHost2 {}
impl ::core::fmt::Debug for ITextHost2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextHost2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextHost2 {
    type Vtable = ITextHost2_Vtbl;
}
impl ::core::clone::Clone for ITextHost2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextHost2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHost2_Vtbl {
    pub base__: ITextHost_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TxIsDoubleClickPending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxIsDoubleClickPending: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetWindow: usize,
    pub TxSetForegroundWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub TxGetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Graphics::Gdi::HPALETTE,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    TxGetPalette: usize,
    pub TxGetEastAsianFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TxSetCursor2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hcur: super::super::WindowsAndMessaging::HCURSOR, btext: super::super::super::Foundation::BOOL) -> super::super::WindowsAndMessaging::HCURSOR,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    TxSetCursor2: usize,
    pub TxFreeTextServicesNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub TxGetEditStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwitem: u32, pdwdata: *mut u32) -> ::windows_core::HRESULT,
    pub TxGetWindowStyles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstyle: *mut u32, pdwexstyle: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub TxShowDropCaret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::super::Foundation::BOOL, hdc: super::super::super::Graphics::Gdi::HDC, prc: *mut super::super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    TxShowDropCaret: usize,
    pub TxDestroyCaret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TxGetHorzExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plhorzextent: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextPara(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextPara {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).SetDuplicate)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CanChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, ppara: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), ppara.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStyle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStyle)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHyphenation(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHyphenation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHyphenation)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFirstLineIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetKeepTogether(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeepTogether)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKeepTogether)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKeepWithNext(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeepWithNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKeepWithNext)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetLeftIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLeftIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineSpacing(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLineSpacing)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLineSpacingRule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetListAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetListAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListLevelIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetListLevelIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListLevelIndex)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetListStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListStart)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListTab(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetListTab)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListTab)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetListType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListType(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetListType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetNoLineNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNoLineNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNoLineNumber)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPageBreakBefore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPageBreakBefore)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetRightIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRightIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRightIndent)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIndents)(::windows_core::Interface::as_raw(self), first, left, right).ok()
    }
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLineSpacing)(::windows_core::Interface::as_raw(self), rule, spacing).ok()
    }
    pub unsafe fn GetSpaceAfter(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpaceAfter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpaceAfter)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpaceBefore(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpaceBefore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpaceBefore)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetWidowControl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWidowControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWidowControl)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetTabCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTabCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddTab)(::windows_core::Interface::as_raw(self), tbpos, tbalign, tbleader).ok()
    }
    pub unsafe fn ClearAllTabs(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearAllTabs)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTab)(::windows_core::Interface::as_raw(self), tbpos).ok()
    }
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTab)(::windows_core::Interface::as_raw(self), itab, ptbpos, ptbalign, ptbleader).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextPara, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextPara {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextPara {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextPara {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextPara {
    type Vtable = ITextPara_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextPara {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextPara {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c4_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDuplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDuplicate: usize,
    pub CanChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetHyphenation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows_core::HRESULT,
    pub SetHyphenation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetFirstLineIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut tomConstants) -> ::windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetLeftIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub GetLineSpacingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetListAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetListAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetListLevelIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetListLevelIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetListStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetListStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetListTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetListTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetListType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetListType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetNoLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetNoLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetPageBreakBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetPageBreakBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetRightIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetRightIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub SetIndents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, first: f32, left: f32, right: f32) -> ::windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: i32, spacing: f32) -> ::windows_core::HRESULT,
    pub GetSpaceAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpaceAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetSpaceBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpaceBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub GetWidowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetWidowControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetTabCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub AddTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows_core::HRESULT,
    pub ClearAllTabs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeleteTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tbpos: f32) -> ::windows_core::HRESULT,
    pub GetTab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextPara2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextPara2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDuplicate)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, ppara: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsEqual)(::windows_core::Interface::as_raw(self), ppara.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Reset)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetStyle(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStyle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStyle(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStyle)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHyphenation(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHyphenation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHyphenation(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetHyphenation)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetFirstLineIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFirstLineIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetKeepTogether(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetKeepTogether)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetKeepTogether)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKeepWithNext(&self) -> ::windows_core::Result<tomConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetKeepWithNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetKeepWithNext)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetLeftIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLeftIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineSpacing(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLineSpacing)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLineSpacingRule(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLineSpacingRule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetListAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetListAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListLevelIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetListLevelIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListLevelIndex(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListLevelIndex)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetListStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListStart(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListStart)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListTab(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetListTab)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListTab(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListTab)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetListType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetListType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListType(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetListType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetNoLineNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNoLineNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNoLineNumber(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNoLineNumber)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetPageBreakBefore(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPageBreakBefore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPageBreakBefore(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPageBreakBefore)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetRightIndent(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRightIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRightIndent(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRightIndent)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetIndents(&self, first: f32, left: f32, right: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIndents)(::windows_core::Interface::as_raw(self), first, left, right).ok()
    }
    pub unsafe fn SetLineSpacing(&self, rule: i32, spacing: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLineSpacing)(::windows_core::Interface::as_raw(self), rule, spacing).ok()
    }
    pub unsafe fn GetSpaceAfter(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSpaceAfter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpaceAfter(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSpaceAfter)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSpaceBefore(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSpaceBefore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpaceBefore(&self, value: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSpaceBefore)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetWidowControl(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWidowControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWidowControl(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWidowControl)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetTabCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTabCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddTab(&self, tbpos: f32, tbalign: i32, tbleader: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddTab)(::windows_core::Interface::as_raw(self), tbpos, tbalign, tbleader).ok()
    }
    pub unsafe fn ClearAllTabs(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ClearAllTabs)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeleteTab(&self, tbpos: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTab)(::windows_core::Interface::as_raw(self), tbpos).ok()
    }
    pub unsafe fn GetTab(&self, itab: i32, ptbpos: *mut f32, ptbalign: *mut i32, ptbleader: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTab)(::windows_core::Interface::as_raw(self), itab, ptbpos, ptbalign, ptbleader).ok()
    }
    pub unsafe fn GetBorders(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBorders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate2(&self) -> ::windows_core::Result<ITextPara2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDuplicate2<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara2>,
    {
        (::windows_core::Interface::vtable(self).SetDuplicate2)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetFontAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFontAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFontAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFontAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHangingPunctuation(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHangingPunctuation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHangingPunctuation(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHangingPunctuation)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetSnapToGrid(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSnapToGrid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSnapToGrid(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSnapToGrid)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetTrimPunctuationAtStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTrimPunctuationAtStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTrimPunctuationAtStart(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrimPunctuationAtStart)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEffects(&self, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetEffects)(::windows_core::Interface::as_raw(self), pvalue, pmask).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual2<P0>(&self, ppara: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextPara2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual2)(::windows_core::Interface::as_raw(self), ppara.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEffects(&self, value: i32, mask: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEffects)(::windows_core::Interface::as_raw(self), value, mask).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextPara2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextPara);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextPara2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextPara2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextPara2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextPara2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextPara2 {
    type Vtable = ITextPara2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextPara2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextPara2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e4_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextPara2_Vtbl {
    pub base__: ITextPara_Vtbl,
    pub GetBorders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppborders: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDuplicate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDuplicate2: usize,
    pub GetFontAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetFontAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetHangingPunctuation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetHangingPunctuation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetSnapToGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetSnapToGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetTrimPunctuationAtStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetTrimPunctuationAtStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32, pmask: *mut i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void, pb: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual2: usize,
    pub SetEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, mask: i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextRange(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextRange {
    pub unsafe fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetChar)(::windows_core::Interface::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFormattedText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        (::windows_core::Interface::vtable(self).SetFormattedText)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStart)(::windows_core::Interface::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnd)(::windows_core::Interface::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).SetFont)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).SetPara)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Collapse)(::windows_core::Interface::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Expand)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIndex)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIndex)(::windows_core::Interface::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InRange)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InStory)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Select)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StartOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Move)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveStart)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveEnd)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveStartWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveEndWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveStartUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveEndUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindText<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextStart<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindTextStart)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextEnd<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).FindTextEnd)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Cut(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Cut)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Paste)(::windows_core::Interface::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CanPaste)(::windows_core::Interface::as_raw(self), pvar, format, &mut result__).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CanEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangeCase)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPoint)(::windows_core::Interface::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPoint)(::windows_core::Interface::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScrollIntoView)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEmbeddedObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextRange, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRange {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextRange {
    type Vtable = ITextRange_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextRange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c2_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetChar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32) -> ::windows_core::HRESULT,
    pub SetChar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, char: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormattedText: usize,
    pub GetStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpfirst: *mut i32) -> ::windows_core::HRESULT,
    pub SetStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32) -> ::windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcplim: *mut i32) -> ::windows_core::HRESULT,
    pub SetEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cplim: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFont: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFont: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPara: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPara: usize,
    pub GetStoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub GetStoryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstart: i32) -> ::windows_core::HRESULT,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, pindex: *mut i32) -> ::windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, index: i32, extend: i32) -> ::windows_core::HRESULT,
    pub SetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InStory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InStory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual: usize,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub EndOf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub MoveStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub MoveEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveWhile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveWhile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveStartWhile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveStartWhile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveEndWhile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveEndWhile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveUntil: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveStartUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveStartUntil: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub MoveEndUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cset: *const super::super::super::System::Variant::VARIANT, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    MoveEndUntil: usize,
    pub FindText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> ::windows_core::HRESULT,
    pub FindTextStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> ::windows_core::HRESULT,
    pub FindTextEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, count: i32, flags: tomConstants, plength: *mut i32) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Cut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Cut: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Copy: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Paste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Paste: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CanPaste: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::System::Variant::VARIANT, format: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CanPaste: usize,
    pub CanEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub ChangeCase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: tomConstants) -> ::windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetEmbeddedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextRange2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextRange2 {
    pub unsafe fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetChar)(::windows_core::Interface::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFormattedText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetFormattedText)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetStart)(::windows_core::Interface::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetEnd)(::windows_core::Interface::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetFont)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPara)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetStoryType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Collapse)(::windows_core::Interface::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Expand)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetIndex)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIndex)(::windows_core::Interface::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InRange)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InStory)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsEqual)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Select)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.StartOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EndOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Move)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveStart)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveEnd)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveStartWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveEndWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveStartUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveEndUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindText<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FindText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextStart<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FindTextStart)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextEnd<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.FindTextEnd)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Cut(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Cut)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Copy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Paste)(::windows_core::Interface::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanPaste)(::windows_core::Interface::as_raw(self), pvar, format, &mut result__).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ChangeCase)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPoint)(::windows_core::Interface::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetPoint)(::windows_core::Interface::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ScrollIntoView)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetEmbeddedObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveLeft)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveRight)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveUp)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveDown)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.HomeKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn TypeText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.TypeText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetCch(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCch)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCells(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCells)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColumn)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate2(&self) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDuplicate2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont2(&self) -> ::windows_core::Result<ITextFont2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFont2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont2<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont2>,
    {
        (::windows_core::Interface::vtable(self).SetFont2)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText2(&self) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFormattedText2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText2<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).SetFormattedText2)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetGravity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGravity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGravity)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara2(&self) -> ::windows_core::Result<ITextPara2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPara2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara2<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara2>,
    {
        (::windows_core::Interface::vtable(self).SetPara2)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRow(&self) -> ::windows_core::Result<ITextRow> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStartPara(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStartPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTable(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetURL)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddSubrange)(::windows_core::Interface::as_raw(self), cp1, cp2, activate).ok()
    }
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BuildUpMath)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteSubrange)(::windows_core::Interface::as_raw(self), cpfirst, cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Find<P0>(&self, prange: P0, count: i32, flags: i32) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Find)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetChar2)(::windows_core::Interface::as_raw(self), pchar, offset).ok()
    }
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDropCap)(::windows_core::Interface::as_raw(self), pcline, pposition).ok()
    }
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInlineObject)(::windows_core::Interface::as_raw(self), ptype, palign, pchar, pchar1, pchar2, pcount, ptexstyle, pccol, plevel).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRect)(::windows_core::Interface::as_raw(self), r#type, pleft, ptop, pright, pbottom, phit).ok()
    }
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSubrange)(::windows_core::Interface::as_raw(self), isubrange, pcpfirst, pcplim).ok()
    }
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetText2)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn HexToUnicode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).HexToUnicode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertTable)(::windows_core::Interface::as_raw(self), ccol, crow, autofit).ok()
    }
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Linearize)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActiveSubrange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDropCap)(::windows_core::Interface::as_raw(self), cline, position).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
    pub unsafe fn SetText2<P0>(&self, flags: i32, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetText2)(::windows_core::Interface::as_raw(self), flags, bstr.into_param().abi()).ok()
    }
    pub unsafe fn UnicodeToHex(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnicodeToHex)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInlineObject)(::windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol).ok()
    }
    pub unsafe fn GetMathFunctionType<P0>(&self, bstr: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMathFunctionType)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertImage<P0, P1>(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: P0, pstream: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).InsertImage)(::windows_core::Interface::as_raw(self), width, height, ascent, r#type, bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextRange2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange, ITextSelection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRange2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRange2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextRange2 {
    type Vtable = ITextRange2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextRange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextRange2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e2_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange2_Vtbl {
    pub base__: ITextSelection_Vtbl,
    pub GetCch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcch: *mut i32) -> ::windows_core::HRESULT,
    pub GetCells: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcells: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumn: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDuplicate2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDuplicate2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFont2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFont2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFont2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfont: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFont2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormattedText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormattedText2: usize,
    pub GetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPara2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppara: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPara2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPara2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppara: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPara2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprow: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRow: usize,
    pub GetStartPara: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddSubrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cp1: i32, cp2: i32, activate: i32) -> ::windows_core::HRESULT,
    pub BuildUpMath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub DeleteSubrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpfirst: i32, cplim: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Find: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, count: i32, flags: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Find: usize,
    pub GetChar2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchar: *mut i32, offset: i32) -> ::windows_core::HRESULT,
    pub GetDropCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcline: *mut i32, pposition: *mut i32) -> ::windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows_core::HRESULT,
    pub GetSubrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows_core::HRESULT,
    pub GetText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub HexToUnicode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccol: i32, crow: i32, autofit: i32) -> ::windows_core::HRESULT,
    pub Linearize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub SetActiveSubrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpanchor: i32, cpactive: i32) -> ::windows_core::HRESULT,
    pub SetDropCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cline: i32, position: i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
    pub SetText2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub UnicodeToHex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows_core::HRESULT,
    pub GetMathFunctionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: ::std::mem::MaybeUninit<::windows_core::BSTR>, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertImage: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextRow(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextRow {
    pub unsafe fn GetAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellCount(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellCount)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellCountCache(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellCountCache)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellCountCache(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellCountCache)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellIndex(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellIndex)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellMargin(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellMargin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellMargin(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellMargin)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetHeight(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHeight)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetHeight(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetHeight)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetIndent(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIndent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndent(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIndent)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKeepTogether(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeepTogether)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepTogether(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKeepTogether)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetKeepWithNext(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeepWithNext)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeepWithNext(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetKeepWithNext)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetNestLevel(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNestLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRTL(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRTL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRTL(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRTL)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellAlignment(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellAlignment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellAlignment(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellAlignment)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellColorBack(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellColorBack)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellColorBack(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellColorBack)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellColorFore(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellColorFore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellColorFore(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellColorFore)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellMergeFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellMergeFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellMergeFlags(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellMergeFlags)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellShading(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellShading)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellShading(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellShading)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellVerticalText(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellVerticalText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellVerticalText(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellVerticalText)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellWidth(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCellWidth)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCellWidth(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellWidth)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetCellBorderColors(&self, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCellBorderColors)(::windows_core::Interface::as_raw(self), pcrleft, pcrtop, pcrright, pcrbottom).ok()
    }
    pub unsafe fn GetCellBorderWidths(&self, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCellBorderWidths)(::windows_core::Interface::as_raw(self), pduleft, pdutop, pduright, pdubottom).ok()
    }
    pub unsafe fn SetCellBorderColors(&self, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellBorderColors)(::windows_core::Interface::as_raw(self), crleft, crtop, crright, crbottom).ok()
    }
    pub unsafe fn SetCellBorderWidths(&self, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCellBorderWidths)(::windows_core::Interface::as_raw(self), duleft, dutop, duright, dubottom).ok()
    }
    pub unsafe fn Apply(&self, crow: i32, flags: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Apply)(::windows_core::Interface::as_raw(self), crow, flags).ok()
    }
    pub unsafe fn CanChange(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CanChange)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn Insert(&self, crow: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Insert)(::windows_core::Interface::as_raw(self), crow).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prow: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRow>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsEqual)(::windows_core::Interface::as_raw(self), prow.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextRow, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextRow {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextRow {
    type Vtable = ITextRow_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextRow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextRow {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5ef_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextRow_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub GetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellCountCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellCountCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetIndent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetKeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetKeepTogether: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetKeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetKeepWithNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetNestLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetRTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetRTL: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellColorBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellColorBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellColorFore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellColorFore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellMergeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellMergeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellShading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellShading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellVerticalText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellVerticalText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetCellBorderColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcrleft: *mut i32, pcrtop: *mut i32, pcrright: *mut i32, pcrbottom: *mut i32) -> ::windows_core::HRESULT,
    pub GetCellBorderWidths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduleft: *mut i32, pdutop: *mut i32, pduright: *mut i32, pdubottom: *mut i32) -> ::windows_core::HRESULT,
    pub SetCellBorderColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crleft: i32, crtop: i32, crright: i32, crbottom: i32) -> ::windows_core::HRESULT,
    pub SetCellBorderWidths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duleft: i32, dutop: i32, duright: i32, dubottom: i32) -> ::windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crow: i32, flags: tomConstants) -> ::windows_core::HRESULT,
    pub CanChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crow: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prow: *mut ::core::ffi::c_void, pb: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsEqual: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextSelection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection {
    pub unsafe fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetChar)(::windows_core::Interface::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFormattedText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFormattedText)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetStart)(::windows_core::Interface::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetEnd)(::windows_core::Interface::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFont)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPara)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStoryType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Collapse)(::windows_core::Interface::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Expand)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIndex)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIndex)(::windows_core::Interface::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InRange)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.InStory)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsEqual)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Select)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.StartOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Move)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveStart)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveEnd)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveStartWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveEndWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveStartUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoveEndUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindText<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FindText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextStart<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FindTextStart)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextEnd<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.FindTextEnd)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Cut(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Cut)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Copy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Paste)(::windows_core::Interface::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanPaste)(::windows_core::Interface::as_raw(self), pvar, format, &mut result__).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ChangeCase)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPoint)(::windows_core::Interface::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPoint)(::windows_core::Interface::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ScrollIntoView)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetEmbeddedObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveLeft)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveRight)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveUp)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MoveDown)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HomeKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EndKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn TypeText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).TypeText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextSelection, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextSelection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextSelection {
    type Vtable = ITextSelection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextSelection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c1_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_Vtbl {
    pub base__: ITextRange_Vtbl,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut i32) -> ::windows_core::HRESULT,
    pub MoveLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub MoveRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub MoveUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub MoveDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, count: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub HomeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: tomConstants, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub EndKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: i32, extend: i32, pdelta: *mut i32) -> ::windows_core::HRESULT,
    pub TypeText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextSelection2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextSelection2 {
    pub unsafe fn GetText(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetChar(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetChar)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChar(&self, char: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetChar)(::windows_core::Interface::as_raw(self), char).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetDuplicate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFormattedText)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetFormattedText)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStart)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStart(&self, cpfirst: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetStart)(::windows_core::Interface::as_raw(self), cpfirst).ok()
    }
    pub unsafe fn GetEnd(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetEnd)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEnd(&self, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetEnd)(::windows_core::Interface::as_raw(self), cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont(&self) -> ::windows_core::Result<ITextFont> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetFont)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetFont)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara(&self) -> ::windows_core::Result<ITextPara> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPara)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    pub unsafe fn GetStoryLength(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStoryLength)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStoryType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetStoryType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, bstart: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Collapse)(::windows_core::Interface::as_raw(self), bstart).ok()
    }
    pub unsafe fn Expand(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Expand)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self, unit: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetIndex)(::windows_core::Interface::as_raw(self), unit, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndex(&self, unit: i32, index: i32, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetIndex)(::windows_core::Interface::as_raw(self), unit, index, extend).ok()
    }
    pub unsafe fn SetRange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetRange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InRange<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.InRange)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InStory<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.InStory)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsEqual<P0>(&self, prange: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsEqual)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Select(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Select)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn StartOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.StartOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndOf(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EndOf)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn Move(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Move)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveStart(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveStart)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveEnd(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveEnd)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveStartWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndWhile(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveEndWhile)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveStartUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveStartUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn MoveEndUntil(&self, cset: *const super::super::super::System::Variant::VARIANT, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoveEndUntil)(::windows_core::Interface::as_raw(self), cset, count, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindText<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.FindText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextStart<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.FindTextStart)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindTextEnd<P0>(&self, bstr: P0, count: i32, flags: tomConstants) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.FindTextEnd)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self, unit: i32, count: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Delete)(::windows_core::Interface::as_raw(self), unit, count, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Cut(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Cut)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Copy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Paste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.Paste)(::windows_core::Interface::as_raw(self), pvar, format).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CanPaste(&self, pvar: *const super::super::super::System::Variant::VARIANT, format: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CanPaste)(::windows_core::Interface::as_raw(self), pvar, format, &mut result__).from_abi(result__)
    }
    pub unsafe fn CanEdit(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CanEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCase(&self, r#type: tomConstants) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ChangeCase)(::windows_core::Interface::as_raw(self), r#type).ok()
    }
    pub unsafe fn GetPoint(&self, r#type: tomConstants, px: *mut i32, py: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPoint)(::windows_core::Interface::as_raw(self), r#type, px, py).ok()
    }
    pub unsafe fn SetPoint(&self, x: i32, y: i32, r#type: tomConstants, extend: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPoint)(::windows_core::Interface::as_raw(self), x, y, r#type, extend).ok()
    }
    pub unsafe fn ScrollIntoView(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.ScrollIntoView)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetEmbeddedObject(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetEmbeddedObject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveLeft(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveLeft)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveRight(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveRight)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveUp(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveUp)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn MoveDown(&self, unit: i32, count: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoveDown)(::windows_core::Interface::as_raw(self), unit, count, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn HomeKey(&self, unit: tomConstants, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.HomeKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn EndKey(&self, unit: i32, extend: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EndKey)(::windows_core::Interface::as_raw(self), unit, extend, &mut result__).from_abi(result__)
    }
    pub unsafe fn TypeText<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.TypeText)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn GetCch(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCch)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCells(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCells)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColumn)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDuplicate2(&self) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDuplicate2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFont2(&self) -> ::windows_core::Result<ITextFont2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFont2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFont2<P0>(&self, pfont: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextFont2>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFont2)(::windows_core::Interface::as_raw(self), pfont.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText2(&self) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFormattedText2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText2<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).base__.SetFormattedText2)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    pub unsafe fn GetGravity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGravity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGravity(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGravity)(::windows_core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPara2(&self) -> ::windows_core::Result<ITextPara2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPara2)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPara2<P0>(&self, ppara: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextPara2>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPara2)(::windows_core::Interface::as_raw(self), ppara.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRow(&self) -> ::windows_core::Result<ITextRow> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRow)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStartPara(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStartPara)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTable(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetURL(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetURL)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetURL<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetURL)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn AddSubrange(&self, cp1: i32, cp2: i32, activate: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddSubrange)(::windows_core::Interface::as_raw(self), cp1, cp2, activate).ok()
    }
    pub unsafe fn BuildUpMath(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BuildUpMath)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn DeleteSubrange(&self, cpfirst: i32, cplim: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteSubrange)(::windows_core::Interface::as_raw(self), cpfirst, cplim).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Find<P0>(&self, prange: P0, count: i32, flags: i32) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Find)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), count, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChar2(&self, pchar: *mut i32, offset: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetChar2)(::windows_core::Interface::as_raw(self), pchar, offset).ok()
    }
    pub unsafe fn GetDropCap(&self, pcline: *mut i32, pposition: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDropCap)(::windows_core::Interface::as_raw(self), pcline, pposition).ok()
    }
    pub unsafe fn GetInlineObject(&self, ptype: *mut i32, palign: *mut i32, pchar: *mut i32, pchar1: *mut i32, pchar2: *mut i32, pcount: *mut i32, ptexstyle: *mut i32, pccol: *mut i32, plevel: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetInlineObject)(::windows_core::Interface::as_raw(self), ptype, palign, pchar, pchar1, pchar2, pcount, ptexstyle, pccol, plevel).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRect(&self, r#type: i32, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32, phit: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRect)(::windows_core::Interface::as_raw(self), r#type, pleft, ptop, pright, pbottom, phit).ok()
    }
    pub unsafe fn GetSubrange(&self, isubrange: i32, pcpfirst: *mut i32, pcplim: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetSubrange)(::windows_core::Interface::as_raw(self), isubrange, pcpfirst, pcplim).ok()
    }
    pub unsafe fn GetText2(&self, flags: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetText2)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn HexToUnicode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.HexToUnicode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InsertTable(&self, ccol: i32, crow: i32, autofit: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InsertTable)(::windows_core::Interface::as_raw(self), ccol, crow, autofit).ok()
    }
    pub unsafe fn Linearize(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Linearize)(::windows_core::Interface::as_raw(self), flags).ok()
    }
    pub unsafe fn SetActiveSubrange(&self, cpanchor: i32, cpactive: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetActiveSubrange)(::windows_core::Interface::as_raw(self), cpanchor, cpactive).ok()
    }
    pub unsafe fn SetDropCap(&self, cline: i32, position: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDropCap)(::windows_core::Interface::as_raw(self), cline, position).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
    pub unsafe fn SetText2<P0>(&self, flags: i32, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetText2)(::windows_core::Interface::as_raw(self), flags, bstr.into_param().abi()).ok()
    }
    pub unsafe fn UnicodeToHex(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnicodeToHex)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInlineObject(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetInlineObject)(::windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol).ok()
    }
    pub unsafe fn GetMathFunctionType<P0>(&self, bstr: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMathFunctionType)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertImage<P0, P1>(&self, width: i32, height: i32, ascent: i32, r#type: i32, bstralttext: P0, pstream: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).base__.InsertImage)(::windows_core::Interface::as_raw(self), width, height, ascent, r#type, bstralttext.into_param().abi(), pstream.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextSelection2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextRange, ITextSelection, ITextRange2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextSelection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextSelection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextSelection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextSelection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextSelection2 {
    type Vtable = ITextSelection2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextSelection2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextSelection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e1_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection2_Vtbl {
    pub base__: ITextRange2_Vtbl,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct ITextServices(::windows_core::IUnknown);
impl ITextServices {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSendMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).TxSendMessage)(::windows_core::Interface::as_raw(self), msg, wparam.into_param().abi(), lparam.into_param().abi(), plresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxDraw)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcbounds, lprcwbounds, lprcupdate, pfncontinue, dwcontinue, lviewid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetHScroll)(::windows_core::Interface::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetVScroll)(::windows_core::Interface::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).OnTxSetCursor)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcclient, x, y).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxQueryHitPoint)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcclient, x, y, phitresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxInPlaceActivate)(::windows_core::Interface::as_raw(self), prcclient).ok()
    }
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxInPlaceDeactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIActivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxUIActivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxUIDeactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TxGetText(&self, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    pub unsafe fn TxSetText<P0>(&self, psztext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).TxSetText)(::windows_core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetCurTargetX)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetBaseLinePos)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize<P0, P1>(&self, dwaspect: u32, hdcdraw: P0, hictargetdev: P1, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxGetNaturalSize)(::windows_core::Interface::as_raw(self), dwaspect, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ptd, dwmode, psizelextent, pwidth, pheight).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows_core::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TxGetDropTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnTxPropertyBitsChange)(::windows_core::Interface::as_raw(self), dwmask, dwbits).ok()
    }
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).TxGetCachedSize)(::windows_core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITextServices, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for ITextServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices {}
impl ::core::fmt::Debug for ITextServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextServices {
    type Vtable = ITextServices_Vtbl;
}
impl ::core::clone::Clone for ITextServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextServices {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub TxSendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxSendMessage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxDraw: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetHScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetHScroll: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TxGetVScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TxGetVScroll: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub OnTxSetCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    OnTxSetCursor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxQueryHitPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxQueryHitPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTxInPlaceActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTxInPlaceActivate: usize,
    pub OnTxInPlaceDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnTxUIActivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnTxUIDeactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TxGetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TxSetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub TxGetCurTargetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows_core::HRESULT,
    pub TxGetBaseLinePos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxGetNaturalSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxGetNaturalSize: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub TxGetDropTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdroptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    TxGetDropTarget: usize,
    pub OnTxPropertyBitsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32, dwbits: u32) -> ::windows_core::HRESULT,
    pub TxGetCachedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct ITextServices2(::windows_core::IUnknown);
impl ITextServices2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxSendMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::WPARAM>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSendMessage)(::windows_core::Interface::as_raw(self), msg, wparam.into_param().abi(), lparam.into_param().abi(), plresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxDraw<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcwbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, pfncontinue: isize, dwcontinue: u32, lviewid: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.TxDraw)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcbounds, lprcwbounds, lprcupdate, pfncontinue, dwcontinue, lviewid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetHScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetHScroll)(::windows_core::Interface::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TxGetVScroll(&self, plmin: *mut i32, plmax: *mut i32, plpos: *mut i32, plpage: *mut i32, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetVScroll)(::windows_core::Interface::as_raw(self), plmin, plmax, plpos, plpage, pfenabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn OnTxSetCursor<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.OnTxSetCursor)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcclient, x, y).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxQueryHitPoint<P0, P1>(&self, dwdrawaspect: super::super::super::System::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, hdcdraw: P0, hictargetdev: P1, lprcclient: *mut super::super::super::Foundation::RECT, x: i32, y: i32, phitresult: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.TxQueryHitPoint)(::windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, ptd, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), lprcclient, x, y, phitresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTxInPlaceActivate(&self, prcclient: *mut super::super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxInPlaceActivate)(::windows_core::Interface::as_raw(self), prcclient).ok()
    }
    pub unsafe fn OnTxInPlaceDeactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxInPlaceDeactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIActivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxUIActivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnTxUIDeactivate(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxUIDeactivate)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TxGetText(&self, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrtext)).ok()
    }
    pub unsafe fn TxSetText<P0>(&self, psztext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.TxSetText)(::windows_core::Interface::as_raw(self), psztext.into_param().abi()).ok()
    }
    pub unsafe fn TxGetCurTargetX(&self, param0: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetCurTargetX)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn TxGetBaseLinePos(&self, param0: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetBaseLinePos)(::windows_core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize<P0, P1>(&self, dwaspect: u32, hdcdraw: P0, hictargetdev: P1, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).base__.TxGetNaturalSize)(::windows_core::Interface::as_raw(self), dwaspect, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ptd, dwmode, psizelextent, pwidth, pheight).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn TxGetDropTarget(&self) -> ::windows_core::Result<super::super::super::System::Ole::IDropTarget> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.TxGetDropTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnTxPropertyBitsChange(&self, dwmask: u32, dwbits: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnTxPropertyBitsChange)(::windows_core::Interface::as_raw(self), dwmask, dwbits).ok()
    }
    pub unsafe fn TxGetCachedSize(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.TxGetCachedSize)(::windows_core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn TxGetNaturalSize2<P0, P1>(&self, dwaspect: u32, hdcdraw: P0, hictargetdev: P1, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
        P1: ::windows_core::IntoParam<super::super::super::Graphics::Gdi::HDC>,
    {
        (::windows_core::Interface::vtable(self).TxGetNaturalSize2)(::windows_core::Interface::as_raw(self), dwaspect, hdcdraw.into_param().abi(), hictargetdev.into_param().abi(), ptd, dwmode, psizelextent, pwidth, pheight, pascent).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
    pub unsafe fn TxDrawD2D<P0>(&self, prendertarget: P0, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Direct2D::ID2D1RenderTarget>,
    {
        (::windows_core::Interface::vtable(self).TxDrawD2D)(::windows_core::Interface::as_raw(self), prendertarget.into_param().abi(), lprcbounds, lprcupdate, lviewid).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITextServices2, ::windows_core::IUnknown, ITextServices);
impl ::core::cmp::PartialEq for ITextServices2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextServices2 {}
impl ::core::fmt::Debug for ITextServices2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextServices2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextServices2 {
    type Vtable = ITextServices2_Vtbl;
}
impl ::core::clone::Clone for ITextServices2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextServices2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextServices2_Vtbl {
    pub base__: ITextServices_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub TxGetNaturalSize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, hdcdraw: super::super::super::Graphics::Gdi::HDC, hictargetdev: super::super::super::Graphics::Gdi::HDC, ptd: *mut super::super::super::System::Com::DVTARGETDEVICE, dwmode: u32, psizelextent: *const super::super::super::Foundation::SIZE, pwidth: *mut i32, pheight: *mut i32, pascent: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    TxGetNaturalSize2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
    pub TxDrawD2D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendertarget: *mut ::core::ffi::c_void, lprcbounds: *mut super::super::super::Foundation::RECTL, lprcupdate: *mut super::super::super::Foundation::RECT, lviewid: i32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D")))]
    TxDrawD2D: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
pub struct ITextStory(::windows_core::IUnknown);
impl ITextStory {
    pub unsafe fn GetActive(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetActive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetActive(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetActive)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetDisplay(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplay)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetProperty(&self, r#type: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRange(&self, cpactive: i32, cpanchor: i32) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRange)(::windows_core::Interface::as_raw(self), cpactive, cpanchor, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetText(&self, flags: i32) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetText)(::windows_core::Interface::as_raw(self), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFormattedText<P0>(&self, punk: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).SetFormattedText)(::windows_core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn SetProperty(&self, r#type: i32, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), r#type, value).ok()
    }
    pub unsafe fn SetText<P0>(&self, flags: i32, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetText)(::windows_core::Interface::as_raw(self), flags, bstr.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ITextStory, ::windows_core::IUnknown);
impl ::core::cmp::PartialEq for ITextStory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStory {}
impl ::core::fmt::Debug for ITextStory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITextStory {
    type Vtable = ITextStory_Vtbl;
}
impl ::core::clone::Clone for ITextStory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITextStory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5f3_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdisplay: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut i32) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, pvalue: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cpactive: i32, cpanchor: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRange: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, value: i32) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextStoryRanges(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextStoryRanges, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStoryRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStoryRanges {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStoryRanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextStoryRanges {
    type Vtable = ITextStoryRanges_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextStoryRanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextStoryRanges {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cc497c5_a1df_11ce_8098_00aa0047be5d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextStoryRanges2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextStoryRanges2 {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<ITextRange> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item2(&self, index: i32) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item2)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextStoryRanges2, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch, ITextStoryRanges);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStoryRanges2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStoryRanges2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStoryRanges2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoryRanges2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextStoryRanges2 {
    type Vtable = ITextStoryRanges2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextStoryRanges2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextStoryRanges2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e5_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoryRanges2_Vtbl {
    pub base__: ITextStoryRanges_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item2: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITextStrings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITextStrings {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows_core::Result<ITextRange2> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append<P0>(&self, prange: P0, istring: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), prange.into_param().abi(), istring).ok()
    }
    pub unsafe fn Cat2(&self, istring: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cat2)(::windows_core::Interface::as_raw(self), istring).ok()
    }
    pub unsafe fn CatTop2<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).CatTop2)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteRange<P0>(&self, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).DeleteRange)(::windows_core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncodeFunction<P0>(&self, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).EncodeFunction)(::windows_core::Interface::as_raw(self), r#type, align, char, char1, char2, count, texstyle, ccol, prange.into_param().abi()).ok()
    }
    pub unsafe fn GetCch(&self, istring: i32) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCch)(::windows_core::Interface::as_raw(self), istring, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertNullStr(&self, istring: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertNullStr)(::windows_core::Interface::as_raw(self), istring).ok()
    }
    pub unsafe fn MoveBoundary(&self, istring: i32, cch: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveBoundary)(::windows_core::Interface::as_raw(self), istring, cch).ok()
    }
    pub unsafe fn PrefixTop<P0>(&self, bstr: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PrefixTop)(::windows_core::Interface::as_raw(self), bstr.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, istring: i32, cstring: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), istring, cstring).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormattedText<P0, P1>(&self, pranged: P0, pranges: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ITextRange2>,
        P1: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).SetFormattedText)(::windows_core::Interface::as_raw(self), pranged.into_param().abi(), pranges.into_param().abi()).ok()
    }
    pub unsafe fn SetOpCp(&self, istring: i32, cp: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpCp)(::windows_core::Interface::as_raw(self), istring, cp).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuffixTop<P0, P1>(&self, bstr: P0, prange: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<ITextRange2>,
    {
        (::windows_core::Interface::vtable(self).SuffixTop)(::windows_core::Interface::as_raw(self), bstr.into_param().abi(), prange.into_param().abi()).ok()
    }
    pub unsafe fn Swap(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Swap)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(ITextStrings, ::windows_core::IUnknown, super::super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITextStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITextStrings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITextStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStrings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ITextStrings {
    type Vtable = ITextStrings_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITextStrings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for ITextStrings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc241f5e7_7206_11d8_a2c7_00a0d1d6c6b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITextStrings_Vtbl {
    pub base__: super::super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, istring: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append: usize,
    pub Cat2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows_core::HRESULT,
    pub CatTop2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteRange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncodeFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: i32, align: i32, char: i32, char1: i32, char2: i32, count: i32, texstyle: i32, ccol: i32, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncodeFunction: usize,
    pub GetCch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, pcch: *mut i32) -> ::windows_core::HRESULT,
    pub InsertNullStr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32) -> ::windows_core::HRESULT,
    pub MoveBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cch: i32) -> ::windows_core::HRESULT,
    pub PrefixTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cstring: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pranged: *mut ::core::ffi::c_void, pranges: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormattedText: usize,
    pub SetOpCp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, istring: i32, cp: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SuffixTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SuffixTop: usize,
    pub Swap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_NOCHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_NODELIMITER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ATP_REPLACEALLTEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_DISABLEMIXEDLGC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEDRIVELETTERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEAURLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEEMAILADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLETELNO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const AURL_ENABLEURL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_FORCERECALC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_RTLDIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOE_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_CONTEXTALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_CONTEXTREADING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_DEFPARADIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_LEGACYBIDICLASS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_NEUTRALOVERRIDE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_PLAINTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const BOM_UNICODEBIDI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_CUSTOM: CARET_FLAGS = CARET_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_ITALIC: CARET_FLAGS = CARET_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_NONE: CARET_FLAGS = CARET_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_NULL: CARET_FLAGS = CARET_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_ROTATE90: CARET_FLAGS = CARET_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CARET_RTL: CARET_FLAGS = CARET_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CERICHEDIT_CLASSA: ::windows_core::PCSTR = ::windows_core::s!("RichEditCEA");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CERICHEDIT_CLASSW: ::windows_core::PCWSTR = ::windows_core::w!("RichEditCEW");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_ALLCAPS: CFE_EFFECTS = CFE_EFFECTS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_AUTOBACKCOLOR: CFE_EFFECTS = CFE_EFFECTS(67108864u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_AUTOCOLOR: CFE_EFFECTS = CFE_EFFECTS(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_BOLD: CFE_EFFECTS = CFE_EFFECTS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_DISABLED: CFE_EFFECTS = CFE_EFFECTS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_EMBOSS: CFE_EFFECTS = CFE_EFFECTS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_EXTENDED: CFE_EFFECTS = CFE_EFFECTS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_FONTBOUND: CFE_EFFECTS = CFE_EFFECTS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_HIDDEN: CFE_EFFECTS = CFE_EFFECTS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_IMPRINT: CFE_EFFECTS = CFE_EFFECTS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_ITALIC: CFE_EFFECTS = CFE_EFFECTS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_LINK: CFE_EFFECTS = CFE_EFFECTS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_LINKPROTECTED: CFE_EFFECTS = CFE_EFFECTS(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATH: CFE_EFFECTS = CFE_EFFECTS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATHNOBUILDUP: CFE_EFFECTS = CFE_EFFECTS(134217728u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_MATHORDINARY: CFE_EFFECTS = CFE_EFFECTS(536870912u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_OUTLINE: CFE_EFFECTS = CFE_EFFECTS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_PROTECTED: CFE_EFFECTS = CFE_EFFECTS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_REVISED: CFE_EFFECTS = CFE_EFFECTS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SHADOW: CFE_EFFECTS = CFE_EFFECTS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SMALLCAPS: CFE_EFFECTS = CFE_EFFECTS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_STRIKEOUT: CFE_EFFECTS = CFE_EFFECTS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SUBSCRIPT: CFE_EFFECTS = CFE_EFFECTS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_SUPERSCRIPT: CFE_EFFECTS = CFE_EFFECTS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFE_UNDERLINE: CFE_EFFECTS = CFE_EFFECTS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALL: CFM_MASK = CFM_MASK(4160749631u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALL2: CFM_MASK = CFM_MASK(4294967295u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALLCAPS: CFM_MASK = CFM_MASK(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ALLEFFECTS: CFM_MASK = CFM_MASK(2115207167u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ANIMATION: CFM_MASK = CFM_MASK(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_BACKCOLOR: CFM_MASK = CFM_MASK(67108864u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_BOLD: CFM_MASK = CFM_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_CHARSET: CFM_MASK = CFM_MASK(134217728u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_COLOR: CFM_MASK = CFM_MASK(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_COOKIE: CFM_MASK = CFM_MASK(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_DISABLED: CFM_MASK = CFM_MASK(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EFFECTS: CFM_MASK = CFM_MASK(1073741887u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EFFECTS2: CFM_MASK = CFM_MASK(1141080063u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EMBOSS: CFM_MASK = CFM_MASK(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_EXTENDED: CFM_MASK = CFM_MASK(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_FACE: CFM_MASK = CFM_MASK(536870912u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_FONTBOUND: CFM_MASK = CFM_MASK(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_HIDDEN: CFM_MASK = CFM_MASK(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_IMPRINT: CFM_MASK = CFM_MASK(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_ITALIC: CFM_MASK = CFM_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_KERNING: CFM_MASK = CFM_MASK(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LCID: CFM_MASK = CFM_MASK(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LINK: CFM_MASK = CFM_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_LINKPROTECTED: CFM_MASK = CFM_MASK(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATH: CFM_MASK = CFM_MASK(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATHNOBUILDUP: CFM_MASK = CFM_MASK(134217728u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_MATHORDINARY: CFM_MASK = CFM_MASK(536870912u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_OFFSET: CFM_MASK = CFM_MASK(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_OUTLINE: CFM_MASK = CFM_MASK(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_PROTECTED: CFM_MASK = CFM_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_REVAUTHOR: CFM_MASK = CFM_MASK(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_REVISED: CFM_MASK = CFM_MASK(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SHADOW: CFM_MASK = CFM_MASK(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SIZE: CFM_MASK = CFM_MASK(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SMALLCAPS: CFM_MASK = CFM_MASK(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SPACING: CFM_MASK = CFM_MASK(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_STRIKEOUT: CFM_MASK = CFM_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_STYLE: CFM_MASK = CFM_MASK(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SUBSCRIPT: CFM_MASK = CFM_MASK(196608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_SUPERSCRIPT: CFM_MASK = CFM_MASK(196608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_UNDERLINE: CFM_MASK = CFM_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_UNDERLINETYPE: CFM_MASK = CFM_MASK(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CFM_WEIGHT: CFM_MASK = CFM_MASK(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RETEXTOBJ: ::windows_core::PCWSTR = ::windows_core::w!("RichEdit Text and Objects");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RTF: ::windows_core::PCWSTR = ::windows_core::w!("Rich Text Format");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CF_RTFNOOBJS: ::windows_core::PCWSTR = ::windows_core::w!("Rich Text Format Without Objects");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_GENERIC: CHANGETYPE = CHANGETYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_NEWREDO: CHANGETYPE = CHANGETYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_NEWUNDO: CHANGETYPE = CHANGETYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CN_TEXTCHANGED: CHANGETYPE = CHANGETYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_CONVERSATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_DATETIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_FILENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_FULLWIDTHALPHANUMERIC: u32 = 11u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HALFWIDTHALPHANUMERIC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HALFWIDTHKATAKANA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HANGUL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_HIRAGANA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_KATAKANA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_NUMERIC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const CTFMODEBIAS_READING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECN_ENDCOMPOSITION: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECN_NEWTEXT: ENDCOMPOSITIONNOTIFY_CODE = ENDCOMPOSITIONNOTIFY_CODE(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_AND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_OR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECOOP_XOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOHSCROLL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOVSCROLL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_AUTOWORDSELECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_NOHIDESEL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_READONLY: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ECO_WANTRETURN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_END: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ELLIPSIS_WORD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_ENTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPANDDOCUMENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_EXPANDSELECTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_GETVIEWMODE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_MOVESELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EMO_PROMOTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_AUTOURLDETECT: u32 = 1115u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CALLAUTOCORRECTPROC: u32 = 1279u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CANPASTE: u32 = 1074u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CANREDO: u32 = 1109u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_CONVPOSITION: u32 = 1132u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_DISPLAYBAND: u32 = 1075u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXGETSEL: u32 = 1076u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXLIMITTEXT: u32 = 1077u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXLINEFROMCHAR: u32 = 1078u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_EXSETSEL: u32 = 1079u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXT: u32 = 1080u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTEX: u32 = 1103u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTEXW: u32 = 1148u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDTEXTW: u32 = 1147u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FINDWORDBREAK: u32 = 1100u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_FORMATRANGE: u32 = 1081u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETAUTOCORRECTPROC: u32 = 1257u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETAUTOURLDETECT: u32 = 1116u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETBIDIOPTIONS: u32 = 1225u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCHARFORMAT: u32 = 1082u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCTFMODEBIAS: u32 = 1261u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETCTFOPENSTATUS: u32 = 1264u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEDITSTYLE: u32 = 1229u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEDITSTYLEEX: u32 = 1300u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETELLIPSISMODE: u32 = 1329u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETELLIPSISSTATE: u32 = 1346u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETEVENTMASK: u32 = 1083u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETHYPHENATEINFO: u32 = 1254u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOLOR: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOMPMODE: u32 = 1146u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMECOMPTEXT: u32 = 1266u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEMODEBIAS: u32 = 1151u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEOPTIONS: u32 = 1131u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETIMEPROPERTY: u32 = 1268u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETLANGOPTIONS: u32 = 1145u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETOLEINTERFACE: u32 = 1084u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETOPTIONS: u32 = 1102u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPAGE: u32 = 1252u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPAGEROTATE: u32 = 1259u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPARAFORMAT: u32 = 1085u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETPUNCTUATION: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETQUERYRTFOBJ: u32 = 1293u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETREDONAME: u32 = 1111u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSCROLLPOS: u32 = 1245u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSELTEXT: u32 = 1086u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETSTORYTYPE: u32 = 1314u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTABLEPARMS: u32 = 1289u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTEX: u32 = 1118u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTLENGTHEX: u32 = 1119u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTMODE: u32 = 1114u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTEXTRANGE: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTOUCHOPTIONS: u32 = 1334u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETTYPOGRAPHYOPTIONS: u32 = 1227u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETUNDONAME: u32 = 1110u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETVIEWKIND: u32 = 1250u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETWORDBREAKPROCEX: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETWORDWRAPMODE: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_GETZOOM: u32 = 1248u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_HIDESELECTION: u32 = 1087u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_INSERTIMAGE: u32 = 1338u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_INSERTTABLE: u32 = 1256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_ISIME: u32 = 1267u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_OUTLINE: u32 = 1244u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_PASTESPECIAL: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_RECONVERSION: u32 = 1149u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_REDO: u32 = 1108u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_REQUESTRESIZE: u32 = 1089u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SELECTIONTYPE: u32 = 1090u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETAUTOCORRECTPROC: u32 = 1258u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETBIDIOPTIONS: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETBKGNDCOLOR: u32 = 1091u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCHARFORMAT: u32 = 1092u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCTFMODEBIAS: u32 = 1262u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETCTFOPENSTATUS: u32 = 1265u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEDITSTYLE: u32 = 1228u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEDITSTYLEEX: u32 = 1299u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETELLIPSISMODE: u32 = 1330u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETEVENTMASK: u32 = 1093u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETFONTSIZE: u32 = 1247u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETHYPHENATEINFO: u32 = 1255u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMECOLOR: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMEMODEBIAS: u32 = 1150u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETIMEOPTIONS: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETLANGOPTIONS: u32 = 1144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETOLECALLBACK: u32 = 1094u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETOPTIONS: u32 = 1101u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPAGE: u32 = 1253u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPAGEROTATE: u32 = 1260u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPALETTE: u32 = 1117u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPARAFORMAT: u32 = 1095u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETPUNCTUATION: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETQUERYRTFOBJ: u32 = 1294u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETSCROLLPOS: u32 = 1246u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETSTORYTYPE: u32 = 1315u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTABLEPARMS: u32 = 1331u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTARGETDEVICE: u32 = 1096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTEXTEX: u32 = 1121u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTEXTMODE: u32 = 1113u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTOUCHOPTIONS: u32 = 1335u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETTYPOGRAPHYOPTIONS: u32 = 1226u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETUIANAME: u32 = 1344u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETUNDOLIMIT: u32 = 1106u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETVIEWKIND: u32 = 1251u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETWORDBREAKPROCEX: u32 = 1105u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETWORDWRAPMODE: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SETZOOM: u32 = 1249u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_SHOWSCROLLBAR: u32 = 1120u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STOPGROUPTYPING: u32 = 1112u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STREAMIN: u32 = 1097u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EM_STREAMOUT: u32 = 1098u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CLIPFORMAT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_CORRECTTEXT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_DRAGDROPDONE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_DROPFILES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_ENDCOMPOSITION: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_GROUPTYPINGCHANGE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_HIDELINKTOOLTIP: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_IMECHANGE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_KEYEVENTS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LANGCHANGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LINK: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_LOWFIRTF: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_MOUSEEVENTS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_OBJECTPOSITIONS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PAGECHANGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PARAGRAPHEXPANDED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_PROTECTED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_REQUESTRESIZE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SCROLL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SCROLLEVENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_SELCHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_STARTCOMPOSITION: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ENM_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ALIGNLTR: u32 = 1808u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ALIGNRTL: u32 = 1809u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_CLIPFORMAT: u32 = 1810u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_CORRECTTEXT: u32 = 1797u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_DRAGDROPDONE: u32 = 1804u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_DROPFILES: u32 = 1795u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_ENDCOMPOSITION: u32 = 1812u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_IMECHANGE: u32 = 1799u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_LINK: u32 = 1803u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_LOWFIRTF: u32 = 1807u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_MSGFILTER: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_OBJECTPOSITIONS: u32 = 1802u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_OLEOPFAILED: u32 = 1801u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PAGECHANGE: u32 = 1806u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PARAGRAPHEXPANDED: u32 = 1805u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_PROTECTED: u32 = 1796u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_REQUESTRESIZE: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_SAVECLIPBOARD: u32 = 1800u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_SELCHANGE: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_STARTCOMPOSITION: u32 = 1811u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EN_STOPNOUNDO: u32 = 1798u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_180: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_270: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_90: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const EPR_SE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_DISABLENOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_EX_NOCALLOLEINIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_NOIME: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_NOOLEDRAGDROP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SAVESEL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SELECTIONBAR: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SELFIME: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_SUNKEN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ES_VERTICAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHALEFHAMZA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHDIAC: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const FR_MATCHKASHIDA: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_GRIPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_SPELLING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCMF_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_MOUSEMENU: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_RIGHTMOUSEDROP: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(32768u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GCM_TOUCHMENU: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_CLOSE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_DEFAULT: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_NUMBYTES: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_NUMCHARS: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_PRECISE: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GTL_USECRLF: GETTEXTLENGTHEX_FLAGS = GETTEXTLENGTHEX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_DEFAULT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_NOHIDDENTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_RAWTEXT: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_SELECTION: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const GT_USECRLF: GETTEXTEX_FLAGS = GETTEXTEX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_CTF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2_5: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL2_SUI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_LEVEL3: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICM_NOTOPEN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ICT_RESULTREADSTR: IMECOMPTEXT_FLAGS = IMECOMPTEXT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOFONT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOFONTSIZEADJUST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_AUTOKEYBOARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_CLOSESTATUSWINDOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_DUALFONT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEACTIVE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEDISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEENABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEINACTIVE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCENONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_FORCEREMEMBER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMEALWAYSSENDNOTIFY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMECANCELCOMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_IMEUIINTEGRATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_MULTIPLEEDIT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NOIMPLICITLANG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NOKBDLIDFIXUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_NORTFFONTSUBSTITUTE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SMODE_NONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_SPELLCHECKING: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_TKBPREDICTION: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_UIFONTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const IMF_VERTICAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MAX_TABLE_CELLS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MAX_TAB_STOPS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MBOLD: MANCODE = MANCODE(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MFRAK: MANCODE = MANCODE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MGREEK: MANCODE = MANCODE(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MINIT: MANCODE = MANCODE(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MISOL: MANCODE = MANCODE(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MITAL: MANCODE = MANCODE(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MLOOP: MANCODE = MANCODE(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MMATH: MANCODE = MANCODE(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MMONO: MANCODE = MANCODE(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MOPEN: MANCODE = MANCODE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MOPENA: MANCODE = MANCODE(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MROMN: MANCODE = MANCODE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSANS: MANCODE = MANCODE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSCRP: MANCODE = MANCODE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSFTEDIT_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("RICHEDIT50W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MSTRCH: MANCODE = MANCODE(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const MTAIL: MANCODE = MANCODE(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const OLEOP_DOVERB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_AUTOCOLOR: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(64u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_BOTTOM: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(8u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_INSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(16u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_LEFT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_OUTSIDE: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(32u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_RIGHT: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_BORDERS_TOP: PARAFORMAT_BORDERS = PARAFORMAT_BORDERS(4u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(3u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(5u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_HORIZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(6u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(4u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_DARK_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_DOWN_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(9u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_GRID: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(11u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_HORZ: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(7u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_TRELLIS: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(12u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_UP_DIAG: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(10u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_LIGHT_VERT: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(8u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PARAFORMAT_SHADING_STYLE_NONE: PARAFORMAT_SHADING_STYLE = PARAFORMAT_SHADING_STYLE(0u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_DELIMITER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_FOLLOWING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_LEADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PC_OVERFLOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_CENTER: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(3u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_GLYPHS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_INTERLETTER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_INTERWORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_NEWSPAPER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_FULL_SCALED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_JUSTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_LEFT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFA_RIGHT: PARAFORMAT_ALIGNMENT = PARAFORMAT_ALIGNMENT(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_ALIGNMENT: PARAFORMAT_MASK = PARAFORMAT_MASK(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_BORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_BOX: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_COLLAPSED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_DONOTHYPHEN: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_KEEP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_KEEPNEXT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_LINESPACING: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NOLINENUMBER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NOWIDOWCONTROL: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERING: PARAFORMAT_MASK = PARAFORMAT_MASK(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGSTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGSTYLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_NUMBERINGTAB: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OFFSET: PARAFORMAT_MASK = PARAFORMAT_MASK(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OFFSETINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_OUTLINELEVEL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_PAGEBREAKBEFORE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RESERVED2: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RIGHTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_RTLPARA: PARAFORMAT_MASK = PARAFORMAT_MASK(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SHADING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SIDEBYSIDE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SPACEAFTER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_SPACEBEFORE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_STARTINDENT: PARAFORMAT_MASK = PARAFORMAT_MASK(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_STYLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABLE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABLEROWDELIMITER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TABSTOPS: PARAFORMAT_MASK = PARAFORMAT_MASK(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFM_TEXTWRAPPINGBREAK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_NEWNUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(32768u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_NONUMBER: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(1024u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PAREN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(0u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PARENS: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(256u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PERIOD: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(512u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFNS_PLAIN: PARAFORMAT_NUMBERING_STYLE = PARAFORMAT_NUMBERING_STYLE(768u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_ARABIC: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_BULLET: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_LCLETTER: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(3u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_LCROMAN: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(5u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_UCLETTER: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(4u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const PFN_UCROMAN: PARAFORMAT_NUMBERING = PARAFORMAT_NUMBERING(6u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_ALIGNTORIGHT: REOBJECT_FLAGS = REOBJECT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_BELOWBASELINE: REOBJECT_FLAGS = REOBJECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_BLANK: REOBJECT_FLAGS = REOBJECT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_CANROTATE: REOBJECT_FLAGS = REOBJECT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_DONTNEEDPALETTE: REOBJECT_FLAGS = REOBJECT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_DYNAMICSIZE: REOBJECT_FLAGS = REOBJECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETMETAFILE: REOBJECT_FLAGS = REOBJECT_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_ALL_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(7u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_NO_INTERFACES: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_POLEOBJ: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_POLESITE: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_GETOBJ_PSTG: RICH_EDIT_GET_OBJECT_FLAGS = RICH_EDIT_GET_OBJECT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_HILITED: REOBJECT_FLAGS = REOBJECT_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_INPLACEACTIVE: REOBJECT_FLAGS = REOBJECT_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_INVERTEDSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_LINK: REOBJECT_FLAGS = REOBJECT_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_LINKAVAILABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_NULL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_OPEN: REOBJECT_FLAGS = REOBJECT_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_OWNERDRAWSELECT: REOBJECT_FLAGS = REOBJECT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_READWRITEMASK: i32 = 2047i32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_RESIZABLE: REOBJECT_FLAGS = REOBJECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_SELECTED: REOBJECT_FLAGS = REOBJECT_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_STATIC: REOBJECT_FLAGS = REOBJECT_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_USEASBACKGROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const REO_WRAPTEXTAROUND: REOBJECT_FLAGS = REOBJECT_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT60_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("RICHEDIT60W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASS: ::windows_core::PCWSTR = ::windows_core::w!("RichEdit20W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASS10A: ::windows_core::PCSTR = ::windows_core::s!("RICHEDIT");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASSA: ::windows_core::PCSTR = ::windows_core::s!("RichEdit20A");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RICHEDIT_CLASSW: ::windows_core::PCWSTR = ::windows_core::w!("RichEdit20W");
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_DISABLEHANDLES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_READINGMODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const RTO_SHOWHANDLES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ASSOCIATEFONT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_ASSOCIATEFONT2: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_CHARREPFROMLCID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_NOKBUPDATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_SELECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_SMARTFONT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_USEUIRULES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SCF_WORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_EMPTY: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(0u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_MULTICHAR: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(4u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_MULTIOBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(8u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_OBJECT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(2u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SEL_TEXT: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE = RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(1u16);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_ALLOWBEEPS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_BEEPONMAXTEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_BIDI: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWEMBED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWPROOFING: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFALLOWSMARTTAG: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CTFNOLOCK: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_CUSTOMLOOK: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_DEFAULTLATINLIGA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_DRAFTMODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EMULATE10: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EMULATESYSEDIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EXTENDBACKCOLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_HANDLEFRIENDLYURL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_HIDETEMPFORMAT: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_MULTITOUCH: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOACETATESELECTION: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOMATH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOTABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_NOTHEMING: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_USEMOUSEWPARAM: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_EX_USESINGLELINE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_HIDEGRIDLINES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_HYPERLINKTOOLTIPS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LBSCROLLNOTIFY: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LOGICALCARET: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_LOWERCASE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MAPCPS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MAX: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_MULTISELECT: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOEALINEHEIGHTADJUST: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOFOCUSLINKNOTIFY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOIME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_NOINPUTSEQUENCECHK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_SCROLLONKILLFOCUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_SMARTDRAGDROP: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_UPPERCASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USEAIMM: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USEATFONT: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USECRLF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_USECTF: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_WORDDRAGDROP: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SES_XLTCRCRLFTOCR: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_KEEPDOCINFO: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PERSISTVIEWSCALE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PLAINRTF: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_PWD: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_SELECTION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SFF_WRITEXTRAPAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_NCRFORNONASCII: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTFNOOBJS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_RTFVAL: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_TEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_TEXTIZED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_UNICODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SF_USECODEPAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SPF_DONTSETDEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const SPF_SETDEFAULT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_KEEPUNDO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_NEWCHARS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_SELECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const ST_UNICODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const S_MSG_KEY_IGNORED: ::windows_core::HRESULT = ::windows_core::HRESULT(262657i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_MULTICODEPAGE: TEXTMODE = TEXTMODE(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_MULTILEVELUNDO: TEXTMODE = TEXTMODE(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_PLAINTEXT: TEXTMODE = TEXTMODE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_RICHTEXT: TEXTMODE = TEXTMODE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_SINGLECODEPAGE: TEXTMODE = TEXTMODE(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TM_SINGLELEVELUNDO: TEXTMODE = TEXTMODE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_ADVANCEDLAYOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_ADVANCEDTYPOGRAPHY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_DISABLECUSTOMTEXTOUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TO_SIMPLELINEBREAK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXES_ISDIALOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = TXTBACKSTYLE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = TXTBACKSTYLE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_ALLOWBEEP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_AUTOWORDSEL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DDWRITE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_DISABLEDRAG: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_EXTENTCHANGE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_HIDESELECTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_MULTILINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_READONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_RICHTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SAVESELECTION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SELBARCHANGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SHOWACCELERATOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_USECURRENTBKG: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_USEPASSWORD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_VERTICAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTBIT_WORDWRAP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = TXTHITRESULT(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_HIT: TXTHITRESULT = TXTHITRESULT(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = TXTHITRESULT(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = TXTHITRESULT(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_EMU: TXTNATURALSIZE = TXTNATURALSIZE(-2147483648i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = TXTNATURALSIZE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = TXTNATURALSIZE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = TXTNATURALSIZE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = TXTNATURALSIZE(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = TXTNATURALSIZE(1073741824i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = TXTNATURALSIZE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTVIEW_ACTIVE: TXTVIEW = TXTVIEW(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const TXTVIEW_INACTIVE: TXTVIEW = TXTVIEW(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_AUTOTABLE: UNDONAMEID = UNDONAMEID(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_CUT: UNDONAMEID = UNDONAMEID(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_DELETE: UNDONAMEID = UNDONAMEID(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_DRAGDROP: UNDONAMEID = UNDONAMEID(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_PASTE: UNDONAMEID = UNDONAMEID(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_TYPING: UNDONAMEID = UNDONAMEID(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const UID_UNKNOWN: UNDONAMEID = UNDONAMEID(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_NORMAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_OUTLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const VM_PAGE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_CUSTOM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_LEVEL1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_LEVEL2: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_OVERFLOW: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_WORDBREAK: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WBF_WORDWRAP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_MOVEWORDNEXT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_MOVEWORDPREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_NEXTBREAK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const WB_PREVBREAK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const cchTextLimitDefault: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphAddBefore: KHYPH = KHYPH(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphChangeAfter: KHYPH = KHYPH(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphChangeBefore: KHYPH = KHYPH(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphDelAndChange: KHYPH = KHYPH(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphDeleteBefore: KHYPH = KHYPH(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphNil: KHYPH = KHYPH(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const khyphNormal: KHYPH = KHYPH(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const lDefaultTab: u32 = 720u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAboriginal: tomConstants = tomConstants(39i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAccent: OBJECTTYPE = OBJECTTYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAdjustCRLF: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignBar: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignCenter: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignDecimal: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignInterLetter: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignInterWord: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignJustify: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignLeft: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignMatchAscentDescent: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignNewspaper: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignRight: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAlignScaled: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllCaps: tomConstants = tomConstants(-2147483520i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowFinalEOP: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowMathBold: tomConstants = tomConstants(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAllowOffClient: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAnimationMax: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAnsi: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyLater: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyNow: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyRtfDocProps: tomConstants = tomConstants(16384i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomApplyTmp: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomArabic: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomArmenian: tomConstants = tomConstants(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAtEnd: tomConstants = tomConstants(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoBackColor: tomConstants = tomConstants(-2080374784i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoColor: tomConstants = tomConstants(-9999997i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkEmail: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkPath: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkPhone: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoLinkURL: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceAlpha: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceNumeric: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoSpaceParens: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomAutoTextColor: tomConstants = tomConstants(-1073741824i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBIG5: tomConstants = tomConstants(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBackward: tomConstants = tomConstants(-1073741823i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBaltic: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBengali: tomConstants = tomConstants(23i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBlinkingBackground: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBold: tomConstants = tomConstants(-2147483647i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBox: OBJECTTYPE = OBJECTTYPE(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxAlignCenter: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideBottom: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideLeft: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideRight: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxHideTop: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeBLTR: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeH: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeTLBR: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxStrikeV: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBoxedFormula: OBJECTTYPE = OBJECTTYPE(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBrackets: OBJECTTYPE = OBJECTTYPE(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBracketsWithSeps: OBJECTTYPE = OBJECTTYPE(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomBraille: tomConstants = tomConstants(44i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCacheParms: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanCopy: tomConstants = tomConstants(137i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanRedo: tomConstants = tomConstants(138i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCanUndo: tomConstants = tomConstants(139i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCell: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCellStructureChangeOnly: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharFormat: tomConstants = tomConstants(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharRepFromLcid: tomConstants = tomConstants(1073741824i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharRepMax: tomConstants = tomConstants(63i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharacter: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCharset: tomConstants = tomConstants(-2147483648i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCheckTextLimit: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCherokee: tomConstants = tomConstants(38i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomClientCoord: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomClientLink: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCluster: tomConstants = tomConstants(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCollapseEnd: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCollapseStart: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomColumn: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCommentsStory: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressMax: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressNone: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressPunctuation: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCompressPunctuationAndKana: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomConvertMathChar: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomConvertRTF: tomConstants = tomConstants(8192i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCreateAlways: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCreateNew: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomCyrillic: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDash: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashDot: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashDotDot: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDashes: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDecDecSize: tomConstants = tomConstants(254i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDecSize: tomConstants = tomConstants(255i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefault: tomConstants = tomConstants(-9999996i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefaultCharRep: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDefaultTab: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDeseret: tomConstants = tomConstants(61i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDevanagari: tomConstants = tomConstants(22i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDisableSmartFont: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDisabled: tomConstants = tomConstants(-2147475456i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDocAutoLink: tomConstants = tomConstants(141i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDocMathBuild: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDontGrowWithContent: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDots: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDotted: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDouble: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDoubleWave: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomDoublestrike: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEastEurope: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisEnd: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisMode: tomConstants = tomConstants(142i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisNone: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisPresent: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisState: tomConstants = tomConstants(143i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEllipsisWord: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmbeddedFont: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmboss: tomConstants = tomConstants(-2147481600i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEmoji: tomConstants = tomConstants(53i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEnableSmartFont: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEnd: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEndnotesStory: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEq: OBJECTTYPE = OBJECTTYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignBottomRow: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignCenter: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignMask: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayAlignTopRow: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEqArrayLayoutWidth: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEquals: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEquationArray: OBJECTTYPE = OBJECTTYPE(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEthiopic: tomConstants = tomConstants(37i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEvenPagesFooterStory: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomEvenPagesHeaderStory: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomExtend: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomExtendedChar: tomConstants = tomConstants(-2113929216i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFalse: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFindStory: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFirstPageFooterStory: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFirstPageHeaderStory: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFoldMathAlpha: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentAuto: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentBaseline: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentBottom: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentCenter: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentMax: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontAlignmentTop: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontBound: tomConstants = tomConstants(-2146435072i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontPropAlign: tomConstants = tomConstants(829i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontPropTeXStyle: tomConstants = tomConstants(828i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretch: tomConstants = tomConstants(830i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchCondensed: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExpanded: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExtraCondensed: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchExtraExpanded: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchNormal: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchSemiCondensed: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchSemiExpanded: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchUltraCondensed: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStretchUltraExpanded: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyle: tomConstants = tomConstants(831i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleItalic: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleOblique: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontStyleUpright: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightBlack: tomConstants = tomConstants(900i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightBold: tomConstants = tomConstants(700i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraBlack: tomConstants = tomConstants(950i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraBold: tomConstants = tomConstants(800i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightExtraLight: tomConstants = tomConstants(200i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightHeavy: tomConstants = tomConstants(900i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightLight: tomConstants = tomConstants(300i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightMedium: tomConstants = tomConstants(500i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightNormal: tomConstants = tomConstants(400i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightRegular: tomConstants = tomConstants(400i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightSemiBold: tomConstants = tomConstants(600i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFontWeightThin: tomConstants = tomConstants(100i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFootnotesStory: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomForward: tomConstants = tomConstants(1073741823i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFraction: OBJECTTYPE = OBJECTTYPE(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFriendlyLinkAddress: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFriendlyLinkName: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionApply: OBJECTTYPE = OBJECTTYPE(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeIsLim: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeNone: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesArg: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesLim: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomFunctionTypeTakesLim2: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGB2312: tomConstants = tomConstants(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGeorgian: tomConstants = tomConstants(35i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGetHeightOnly: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGlagolitic: tomConstants = tomConstants(54i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGothic: tomConstants = tomConstants(60i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityBack: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityBackward: tomConstants = tomConstants(536870912i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityFore: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityForward: tomConstants = tomConstants(1073741824i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityIn: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityOut: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGravityUI: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGreek: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGrowWithContent: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGujarati: tomConstants = tomConstants(25i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomGurmukhi: tomConstants = tomConstants(24i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHContCell: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHStartCell: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHTML: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHair: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHangul: tomConstants = tomConstants(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHardParagraph: tomConstants = tomConstants(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHeavyWave: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHebrew: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHidden: tomConstants = tomConstants(-2147483392i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHorzVert: OBJECTTYPE = OBJECTTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomHstring: tomConstants = tomConstants(596i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIgnoreCurrentFont: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIgnoreNumberStyle: tomConstants = tomConstants(16777216i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomImprint: tomConstants = tomConstants(-2147479552i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncIncSize: tomConstants = tomConstants(66i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncSize: tomConstants = tomConstants(65i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncludeInset: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomIncludeNumbering: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObject: tomConstants = tomConstants(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObjectArg: tomConstants = tomConstants(21i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomInlineObjectStart: tomConstants = tomConstants(-2130706432i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomItalic: tomConstants = tomConstants(-2147483646i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomJamo: tomConstants = tomConstants(36i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKannada: tomConstants = tomConstants(29i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKayahli: tomConstants = tomConstants(51i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKharoshthi: tomConstants = tomConstants(50i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKhmer: tomConstants = tomConstants(42i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomKoreanBlockCaret: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLanguageTag: tomConstants = tomConstants(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLao: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLasVegasLights: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLayoutColumn: tomConstants = tomConstants(23i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLeafLine: tomConstants = tomConstants(22i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLeftSubSup: OBJECTTYPE = OBJECTTYPE(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimbu: tomConstants = tomConstants(46i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignCenter: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignLeft: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignMask: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitAlignRight: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsOpposite: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsSubSup: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLimitsUnderOver: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLine: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpace1pt5: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceAtLeast: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceDouble: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceExactly: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceMultiple: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpacePercent: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLineSpaceSingle: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLines: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLink: tomConstants = tomConstants(-2147483616i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLinkProtected: tomConstants = tomConstants(-2139095040i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListBullet: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListMinus: tomConstants = tomConstants(524288i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNoNumber: tomConstants = tomConstants(262144i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNone: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsArabic: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsLCLetter: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsLCRoman: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsSequence: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsUCLetter: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberAsUCRoman: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabic1: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabic2: tomConstants = tomConstants(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedArabicWide: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedBlackCircleWingding: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedChS: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedChT: tomConstants = tomConstants(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedCircle: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHebrew: tomConstants = tomConstants(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiAlpha: tomConstants = tomConstants(21i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiAlpha1: tomConstants = tomConstants(22i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedHindiNum: tomConstants = tomConstants(23i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedJpnChS: tomConstants = tomConstants(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedJpnKor: tomConstants = tomConstants(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedThaiAlpha: tomConstants = tomConstants(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedThaiNum: tomConstants = tomConstants(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListNumberedWhiteCircleWingding: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListParentheses: tomConstants = tomConstants(65536i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListPeriod: tomConstants = tomConstants(131072i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomListPlain: tomConstants = tomConstants(196608i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLisu: tomConstants = tomConstants(55i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLongDash: tomConstants = tomConstants(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLowerCase: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomLowerLimit: OBJECTTYPE = OBJECTTYPE(19i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMac: tomConstants = tomConstants(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMainTextStory: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMalayalam: tomConstants = tomConstants(30i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMarchingBlackAnts: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMarchingRedAnts: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchAscii: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchCase: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchCharRep: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchFontSignature: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchMathFont: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchPattern: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatchWord: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMath: OBJECTTYPE = OBJECTTYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathArgShadingEnd: tomConstants = tomConstants(594i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathArgShadingStart: tomConstants = tomConstants(593i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakCenter: tomConstants = tomConstants(126i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakLeft: tomConstants = tomConstants(125i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBreakRight: tomConstants = tomConstants(127i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinAfter: tomConstants = tomConstants(65536i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinBefore: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinDup: tomConstants = tomConstants(131072i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinMask: tomConstants = tomConstants(196608i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMM: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMP: tomConstants = tomConstants(524288i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubMask: tomConstants = tomConstants(786432i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathBrkBinSubPM: tomConstants = tomConstants(262144i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathCFCheck: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignCenter: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignCenterGroup: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignLeft: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignMask: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispAlignRight: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispDef: tomConstants = tomConstants(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispFracTeX: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispIntUnderOver: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispNaryGrow: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDispNarySubSup: tomConstants = tomConstants(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffItalic: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffMask: tomConstants = tomConstants(768i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffOpenItalic: tomConstants = tomConstants(768i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocDiffUpright: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgAlways: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgAuto: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgMask: tomConstants = tomConstants(96i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocEmptyArgNever: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathDocSbSpOpUnchanged: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathEnableRtl: tomConstants = tomConstants(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathEqAlign: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathInterSpace: tomConstants = tomConstants(135i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathIntraSpace: tomConstants = tomConstants(136i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathLMargin: tomConstants = tomConstants(129i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathManualBreakMask: tomConstants = tomConstants(127i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathObjShadingEnd: tomConstants = tomConstants(596i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathObjShadingStart: tomConstants = tomConstants(595i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignCenter: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignCenterGroup: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignLeft: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathParaAlignRight: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathPostSpace: tomConstants = tomConstants(134i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathPreSpace: tomConstants = tomConstants(133i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathRMargin: tomConstants = tomConstants(130i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathRelSize: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathVariant: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathWrapIndent: tomConstants = tomConstants(131i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathWrapRight: tomConstants = tomConstants(132i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZone: tomConstants = tomConstants(-1879048192i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneDisplay: tomConstants = tomConstants(262144i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneNoBuildUp: tomConstants = tomConstants(-2013265920i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMathZoneOrdinary: tomConstants = tomConstants(-1610612736i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrix: OBJECTTYPE = OBJECTTYPE(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignBottomRow: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignCenter: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignMask: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMatrixAlignTopRow: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomModWidthPairs: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomModWidthSpace: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMongolian: tomConstants = tomConstants(43i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMove: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomMyanmar: tomConstants = tomConstants(34i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNKo: tomConstants = tomConstants(57i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNary: OBJECTTYPE = OBJECTTYPE(21i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNewTaiLue: tomConstants = tomConstants(48i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoAnimation: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoBreak: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoHidden: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoIME: tomConstants = tomConstants(524288i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoLink: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoMathZoneBrackets: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoSelection: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoUCGreekItalic: tomConstants = tomConstants(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoUpScroll: tomConstants = tomConstants(65536i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNoVpScroll: tomConstants = tomConstants(262144i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNone: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNormalCaret: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomNullCaret: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOEM: tomConstants = tomConstants(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObject: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObjectArg: tomConstants = tomConstants(2048i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomObjectMax: OBJECTTYPE = OBJECTTYPE(33i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOgham: tomConstants = tomConstants(40i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpChar: OBJECTTYPE = OBJECTTYPE(22i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpenAlways: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOpenExisting: tomConstants = tomConstants(48i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOriya: tomConstants = tomConstants(26i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOsmanya: tomConstants = tomConstants(58i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOutline: tomConstants = tomConstants(-2147483136i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOverbar: OBJECTTYPE = OBJECTTYPE(23i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomOverlapping: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPC437: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPage: tomConstants = tomConstants(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectBox: tomConstants = tomConstants(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectCollapsed: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectDoNotHyphen: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectKeep: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectKeepNext: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectNoLineNumber: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectNoWidowControl: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectOutlineLevel: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectPageBreakBefore: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectRTL: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectSideBySide: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectTable: tomConstants = tomConstants(16384i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaEffectTableRowDelimiter: tomConstants = tomConstants(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaFormat: tomConstants = tomConstants(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaPropMathAlign: tomConstants = tomConstants(1079i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading1: tomConstants = tomConstants(-2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading2: tomConstants = tomConstants(-3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading3: tomConstants = tomConstants(-4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading4: tomConstants = tomConstants(-5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading5: tomConstants = tomConstants(-6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading6: tomConstants = tomConstants(-7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading7: tomConstants = tomConstants(-8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading8: tomConstants = tomConstants(-9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleHeading9: tomConstants = tomConstants(-10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParaStyleNormal: tomConstants = tomConstants(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomParagraph: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPasteFile: tomConstants = tomConstants(4096i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhagsPa: tomConstants = tomConstants(59i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantom: OBJECTTYPE = OBJECTTYPE(24i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomASmash: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomDSmash: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomHSmash: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomHorz: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomShow: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomSmash: tomConstants = tomConstants(13i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomTransparent: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomVert: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroAscent: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroDescent: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPhantomZeroWidth: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPrimaryFooterStory: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomPrimaryHeaderStory: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomProcessId: tomConstants = tomConstants(1073741825i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomProtected: tomConstants = tomConstants(-2147483632i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRE10Mode: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRTF: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRadical: OBJECTTYPE = OBJECTTYPE(25i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomReadOnly: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomReplaceStory: tomConstants = tomConstants(129i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomResume: tomConstants = tomConstants(-9999994i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRevised: tomConstants = tomConstants(-2147467264i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRow: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowApplyDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowHeightActual: tomConstants = tomConstants(2059i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRowUpdate: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRuby: OBJECTTYPE = OBJECTTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlign010: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlign121: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignCenter: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignLeft: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyAlignRight: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRubyBelow: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomRunic: tomConstants = tomConstants(41i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomScratchStory: tomConstants = tomConstants(127i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomScreen: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSection: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelActive: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelAtEOL: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelOvertype: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelRange: tomConstants = tomConstants(597i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelReplace: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelStartActive: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionBlock: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionColumn: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionFrame: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionIP: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionInlineShape: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionNormal: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionRow: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelectionShape: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSelfIME: tomConstants = tomConstants(262144i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSentence: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSentenceCase: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShadow: tomConstants = tomConstants(-2147482624i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShareDenyRead: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShareDenyWrite: tomConstants = tomConstants(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShiftJIS: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShimmer: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowDegPlaceHldr: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowLLimPlaceHldr: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowMatPlaceHldr: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomShowULimPlaceHldr: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSimpleText: OBJECTTYPE = OBJECTTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSingle: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSinhala: tomConstants = tomConstants(31i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeScript: tomConstants = tomConstants(64i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeScriptScript: tomConstants = tomConstants(96i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSizeText: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSlashedFraction: OBJECTTYPE = OBJECTTYPE(26i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSmallCaps: tomConstants = tomConstants(-2147483584i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceBinary: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceDifferential: tomConstants = tomConstants(24i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceMask: tomConstants = tomConstants(28i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceOrd: tomConstants = tomConstants(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceRelational: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceSkip: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaceUnary: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSpaces: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSparkleText: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStack: OBJECTTYPE = OBJECTTYPE(27i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStart: tomConstants = tomConstants(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStory: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveDisplay: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveDisplayUI: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryActiveUI: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStoryInactive: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchBaseAbove: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchBaseBelow: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchCharAbove: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchCharBelow: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStretchStack: OBJECTTYPE = OBJECTTYPE(28i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStrikeout: tomConstants = tomConstants(-2147483640i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDefault: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDisplay: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleDisplayCramped: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScript: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptCramped: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptScript: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleScriptScriptCramped: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleText: tomConstants = tomConstants(6i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomStyleTextCramped: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubSup: OBJECTTYPE = OBJECTTYPE(30i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubSupAlign: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubscript: OBJECTTYPE = OBJECTTYPE(29i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSubscriptCF: tomConstants = tomConstants(-2147418112i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuperscript: OBJECTTYPE = OBJECTTYPE(31i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuperscriptCF: tomConstants = tomConstants(-2147352576i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSuspend: tomConstants = tomConstants(-9999995i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSylotiNagri: tomConstants = tomConstants(49i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSymbol: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomSyriac: tomConstants = tomConstants(20i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabBack: tomConstants = tomConstants(-3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabHere: tomConstants = tomConstants(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTabNext: tomConstants = tomConstants(-2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTable: tomConstants = tomConstants(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTableColumn: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTaiLe: tomConstants = tomConstants(47i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTamil: tomConstants = tomConstants(27i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTelugu: tomConstants = tomConstants(28i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomText: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowES: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowMask: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowNE: tomConstants = tomConstants(12i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowSW: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFlowWN: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextFrameStory: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTextize: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThaana: tomConstants = tomConstants(21i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThai: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThick: tomConstants = tomConstants(9i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDash: tomConstants = tomConstants(14i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDashDot: tomConstants = tomConstants(15i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDashDotDot: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickDotted: tomConstants = tomConstants(17i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickLines: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomThickLongDash: tomConstants = tomConstants(18i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTibetan: tomConstants = tomConstants(33i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTifinagh: tomConstants = tomConstants(62i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTitleCase: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomToggle: tomConstants = tomConstants(-9999998i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomToggleCase: tomConstants = tomConstants(5i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTrackParms: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransform: tomConstants = tomConstants(1024i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTranslateTableCell: tomConstants = tomConstants(128i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransparentForPositioning: tomConstants = tomConstants(256i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTransparentForSpacing: tomConstants = tomConstants(512i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTrue: tomConstants = tomConstants(-1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTruncateExisting: tomConstants = tomConstants(80i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomTurkish: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUndefined: tomConstants = tomConstants(-9999999i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderbar: OBJECTTYPE = OBJECTTYPE(32i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderline: tomConstants = tomConstants(-2147483644i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionAbove: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionAuto: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionBelow: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnderlinePositionMax: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUndoLimit: tomConstants = tomConstants(140i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnhide: tomConstants = tomConstants(16i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnicodeBiDi: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnknownStory: tomConstants = tomConstants(0i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUnlink: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperCase: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperLimit: OBJECTTYPE = OBJECTTYPE(33i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUpperLimitAsSuperScript: tomConstants = tomConstants(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseAtFont: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseCRLF: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUsePoints: tomConstants = tomConstants(10i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUseTwips: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomUsymbol: tomConstants = tomConstants(52i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVLowCell: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVTopCell: tomConstants = tomConstants(1i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVai: tomConstants = tomConstants(56i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomVietnamese: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWarichu: OBJECTTYPE = OBJECTTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWave: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWindow: tomConstants = tomConstants(11i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWipeDown: tomConstants = tomConstants(7i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWipeRight: tomConstants = tomConstants(8i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWord: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWordDocument: tomConstants = tomConstants(4i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomWords: tomConstants = tomConstants(2i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const tomYi: tomConstants = tomConstants(45i32);
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub const yHeightCharPtsMost: u32 = 1638u32;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CARET_FLAGS(pub i32);
impl ::core::marker::Copy for CARET_FLAGS {}
impl ::core::clone::Clone for CARET_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CARET_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CARET_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CARET_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CARET_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CFE_EFFECTS(pub u32);
impl ::core::marker::Copy for CFE_EFFECTS {}
impl ::core::clone::Clone for CFE_EFFECTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CFE_EFFECTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CFE_EFFECTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CFE_EFFECTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CFE_EFFECTS").field(&self.0).finish()
    }
}
impl CFE_EFFECTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CFE_EFFECTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CFE_EFFECTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CFE_EFFECTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CFE_EFFECTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CFE_EFFECTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CFM_MASK(pub u32);
impl ::core::marker::Copy for CFM_MASK {}
impl ::core::clone::Clone for CFM_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CFM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CFM_MASK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CFM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CFM_MASK").field(&self.0).finish()
    }
}
impl CFM_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CFM_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CFM_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CFM_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CFM_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CFM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANGETYPE(pub i32);
impl ::core::marker::Copy for CHANGETYPE {}
impl ::core::clone::Clone for CHANGETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CHANGETYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CHANGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENDCOMPOSITIONNOTIFY_CODE(pub u32);
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY_CODE {}
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ENDCOMPOSITIONNOTIFY_CODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ENDCOMPOSITIONNOTIFY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENDCOMPOSITIONNOTIFY_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GETTEXTEX_FLAGS(pub u32);
impl ::core::marker::Copy for GETTEXTEX_FLAGS {}
impl ::core::clone::Clone for GETTEXTEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GETTEXTEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GETTEXTEX_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GETTEXTEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETTEXTEX_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GETTEXTLENGTHEX_FLAGS(pub u32);
impl ::core::marker::Copy for GETTEXTLENGTHEX_FLAGS {}
impl ::core::clone::Clone for GETTEXTLENGTHEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GETTEXTLENGTHEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GETTEXTLENGTHEX_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GETTEXTLENGTHEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GETTEXTLENGTHEX_FLAGS").field(&self.0).finish()
    }
}
impl GETTEXTLENGTHEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GETTEXTLENGTHEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GETTEXTLENGTHEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMECOMPTEXT_FLAGS(pub u32);
impl ::core::marker::Copy for IMECOMPTEXT_FLAGS {}
impl ::core::clone::Clone for IMECOMPTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMECOMPTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for IMECOMPTEXT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for IMECOMPTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMECOMPTEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KHYPH(pub i32);
impl ::core::marker::Copy for KHYPH {}
impl ::core::clone::Clone for KHYPH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KHYPH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KHYPH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KHYPH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KHYPH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MANCODE(pub i32);
impl ::core::marker::Copy for MANCODE {}
impl ::core::clone::Clone for MANCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MANCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MANCODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MANCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MANCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OBJECTTYPE(pub i32);
impl ::core::marker::Copy for OBJECTTYPE {}
impl ::core::clone::Clone for OBJECTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OBJECTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for OBJECTTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OBJECTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_ALIGNMENT(pub u16);
impl ::core::marker::Copy for PARAFORMAT_ALIGNMENT {}
impl ::core::clone::Clone for PARAFORMAT_ALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_ALIGNMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_ALIGNMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_BORDERS(pub u16);
impl ::core::marker::Copy for PARAFORMAT_BORDERS {}
impl ::core::clone::Clone for PARAFORMAT_BORDERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_BORDERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_BORDERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_BORDERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_BORDERS").field(&self.0).finish()
    }
}
impl PARAFORMAT_BORDERS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_BORDERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_BORDERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_BORDERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_BORDERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_MASK(pub u32);
impl ::core::marker::Copy for PARAFORMAT_MASK {}
impl ::core::clone::Clone for PARAFORMAT_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_MASK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_MASK").field(&self.0).finish()
    }
}
impl PARAFORMAT_MASK {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PARAFORMAT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_NUMBERING(pub u16);
impl ::core::marker::Copy for PARAFORMAT_NUMBERING {}
impl ::core::clone::Clone for PARAFORMAT_NUMBERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_NUMBERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_NUMBERING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_NUMBERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_NUMBERING").field(&self.0).finish()
    }
}
impl PARAFORMAT_NUMBERING {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PARAFORMAT_NUMBERING {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PARAFORMAT_NUMBERING {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PARAFORMAT_NUMBERING {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_NUMBERING_STYLE(pub u16);
impl ::core::marker::Copy for PARAFORMAT_NUMBERING_STYLE {}
impl ::core::clone::Clone for PARAFORMAT_NUMBERING_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_NUMBERING_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_NUMBERING_STYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_NUMBERING_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_NUMBERING_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PARAFORMAT_SHADING_STYLE(pub u16);
impl ::core::marker::Copy for PARAFORMAT_SHADING_STYLE {}
impl ::core::clone::Clone for PARAFORMAT_SHADING_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PARAFORMAT_SHADING_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_SHADING_STYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PARAFORMAT_SHADING_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARAFORMAT_SHADING_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REOBJECT_FLAGS(pub u32);
impl ::core::marker::Copy for REOBJECT_FLAGS {}
impl ::core::clone::Clone for REOBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REOBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for REOBJECT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for REOBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REOBJECT_FLAGS").field(&self.0).finish()
    }
}
impl REOBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for REOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE(pub u16);
impl ::core::marker::Copy for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {}
impl ::core::clone::Clone for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE").field(&self.0).finish()
    }
}
impl RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RICH_EDIT_GET_OBJECT_FLAGS(pub u32);
impl ::core::marker::Copy for RICH_EDIT_GET_OBJECT_FLAGS {}
impl ::core::clone::Clone for RICH_EDIT_GET_OBJECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RICH_EDIT_GET_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RICH_EDIT_GET_OBJECT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RICH_EDIT_GET_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RICH_EDIT_GET_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl RICH_EDIT_GET_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RICH_EDIT_GET_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RICH_EDIT_GET_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXTMODE(pub i32);
impl ::core::marker::Copy for TEXTMODE {}
impl ::core::clone::Clone for TEXTMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXTMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TEXTMODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TEXTMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXTMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXTBACKSTYLE(pub i32);
impl ::core::marker::Copy for TXTBACKSTYLE {}
impl ::core::clone::Clone for TXTBACKSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXTBACKSTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TXTBACKSTYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TXTBACKSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTBACKSTYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXTHITRESULT(pub i32);
impl ::core::marker::Copy for TXTHITRESULT {}
impl ::core::clone::Clone for TXTHITRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXTHITRESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TXTHITRESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TXTHITRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTHITRESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXTNATURALSIZE(pub i32);
impl ::core::marker::Copy for TXTNATURALSIZE {}
impl ::core::clone::Clone for TXTNATURALSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXTNATURALSIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TXTNATURALSIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TXTNATURALSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTNATURALSIZE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TXTVIEW(pub i32);
impl ::core::marker::Copy for TXTVIEW {}
impl ::core::clone::Clone for TXTVIEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TXTVIEW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TXTVIEW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TXTVIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXTVIEW").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UNDONAMEID(pub i32);
impl ::core::marker::Copy for UNDONAMEID {}
impl ::core::clone::Clone for UNDONAMEID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNDONAMEID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for UNDONAMEID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UNDONAMEID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNDONAMEID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct tomConstants(pub i32);
impl ::core::marker::Copy for tomConstants {}
impl ::core::clone::Clone for tomConstants {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for tomConstants {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for tomConstants {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for tomConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("tomConstants").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct BIDIOPTIONS {
    pub cbSize: u32,
    pub wMask: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for BIDIOPTIONS {}
impl ::core::clone::Clone for BIDIOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIDIOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIDIOPTIONS").field("cbSize", &self.cbSize).field("wMask", &self.wMask).field("wEffects", &self.wEffects).finish()
    }
}
impl ::windows_core::TypeKind for BIDIOPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for BIDIOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wMask == other.wMask && self.wEffects == other.wEffects
    }
}
impl ::core::cmp::Eq for BIDIOPTIONS {}
impl ::core::default::Default for BIDIOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub union CARET_INFO {
    pub hbitmap: super::super::super::Graphics::Gdi::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for CARET_INFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for CARET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::TypeKind for CARET_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for CARET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct CHANGENOTIFY {
    pub dwChangeType: u32,
    pub pvCookieData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for CHANGENOTIFY {}
impl ::core::clone::Clone for CHANGENOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGENOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGENOTIFY").field("dwChangeType", &self.dwChangeType).field("pvCookieData", &self.pvCookieData).finish()
    }
}
impl ::windows_core::TypeKind for CHANGENOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CHANGENOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.dwChangeType == other.dwChangeType && self.pvCookieData == other.pvCookieData
    }
}
impl ::core::cmp::Eq for CHANGENOTIFY {}
impl ::core::default::Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMAT2A {
    pub Base: CHARFORMATA,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2A_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2A {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMAT2A {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union CHARFORMAT2A_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2A_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMAT2A_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMAT2W {
    pub Base: CHARFORMATW,
    pub wWeight: u16,
    pub sSpacing: i16,
    pub crBackColor: super::super::super::Foundation::COLORREF,
    pub lcid: u32,
    pub Anonymous: CHARFORMAT2W_0,
    pub sStyle: i16,
    pub wKerning: u16,
    pub bUnderlineType: u8,
    pub bAnimation: u8,
    pub bRevAuthor: u8,
    pub bUnderlineColor: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2W {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMAT2W {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union CHARFORMAT2W_0 {
    pub dwReserved: u32,
    pub dwCookie: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMAT2W_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMAT2W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMAT2W_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMAT2W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMATA {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u8; 32],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHARFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATA").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHARFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHARFORMATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHARFORMATW {
    pub cbSize: u32,
    pub dwMask: CFM_MASK,
    pub dwEffects: CFE_EFFECTS,
    pub yHeight: i32,
    pub yOffset: i32,
    pub crTextColor: super::super::super::Foundation::COLORREF,
    pub bCharSet: super::super::super::Graphics::Gdi::FONT_CHARSET,
    pub bPitchAndFamily: u8,
    pub szFaceName: [u16; 32],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHARFORMATW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHARFORMATW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHARFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARFORMATW").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwEffects", &self.dwEffects).field("yHeight", &self.yHeight).field("yOffset", &self.yOffset).field("crTextColor", &self.crTextColor).field("bCharSet", &self.bCharSet).field("bPitchAndFamily", &self.bPitchAndFamily).field("szFaceName", &self.szFaceName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for CHARFORMATW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHARFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwEffects == other.dwEffects && self.yHeight == other.yHeight && self.yOffset == other.yOffset && self.crTextColor == other.crTextColor && self.bCharSet == other.bCharSet && self.bPitchAndFamily == other.bPitchAndFamily && self.szFaceName == other.szFaceName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHARFORMATW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHARFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct CHARRANGE {
    pub cpMin: i32,
    pub cpMax: i32,
}
impl ::core::marker::Copy for CHARRANGE {}
impl ::core::clone::Clone for CHARRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHARRANGE").field("cpMin", &self.cpMin).field("cpMax", &self.cpMax).finish()
    }
}
impl ::windows_core::TypeKind for CHARRANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CHARRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.cpMin == other.cpMin && self.cpMax == other.cpMax
    }
}
impl ::core::cmp::Eq for CHARRANGE {}
impl ::core::default::Default for CHARRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIPBOARDFORMAT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIPBOARDFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for CLIPBOARDFORMAT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPBOARDFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIPBOARDFORMAT {
    pub nmhdr: super::NMHDR,
    pub cf: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLIPBOARDFORMAT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLIPBOARDFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for CLIPBOARDFORMAT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPBOARDFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPCOLOR {
    pub crText: super::super::super::Foundation::COLORREF,
    pub crBackground: super::super::super::Foundation::COLORREF,
    pub dwEffects: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPCOLOR").field("crText", &self.crText).field("crBackground", &self.crBackground).field("dwEffects", &self.dwEffects).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for COMPCOLOR {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.crText == other.crText && self.crBackground == other.crBackground && self.dwEffects == other.dwEffects
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for EDITSTREAM {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for EDITSTREAM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for EDITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct EDITSTREAM {
    pub dwCookie: usize,
    pub dwError: u32,
    pub pfnCallback: EDITSTREAMCALLBACK,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for EDITSTREAM {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for EDITSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for EDITSTREAM {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for EDITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENCORRECTTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENCORRECTTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENCORRECTTEXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCORRECTTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENCORRECTTEXT {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENCORRECTTEXT {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENCORRECTTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENCORRECTTEXT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENCORRECTTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENDCOMPOSITIONNOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDCOMPOSITIONNOTIFY {
    pub nmhdr: super::NMHDR,
    pub dwCode: ENDCOMPOSITIONNOTIFY_CODE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDCOMPOSITIONNOTIFY {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDCOMPOSITIONNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENDCOMPOSITIONNOTIFY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDCOMPOSITIONNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: super::super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDROPFILES {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDROPFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENDROPFILES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENDROPFILES {
    pub nmhdr: super::NMHDR,
    pub hDrop: super::super::super::Foundation::HANDLE,
    pub cp: i32,
    pub fProtected: super::super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENDROPFILES {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENDROPFILES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENDROPFILES {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENDROPFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLINK {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENLINK {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLINK {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLINK {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLINK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENLINK {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLINK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: ::windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLOWFIRTF {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLOWFIRTF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENLOWFIRTF {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENLOWFIRTF {
    pub nmhdr: super::NMHDR,
    pub szControl: ::windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENLOWFIRTF {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENLOWFIRTF {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENLOWFIRTF {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENLOWFIRTF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows_core::HRESULT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENOLEOPFAILED {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENOLEOPFAILED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENOLEOPFAILED {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENOLEOPFAILED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENOLEOPFAILED {
    pub nmhdr: super::NMHDR,
    pub iob: i32,
    pub lOper: i32,
    pub hr: ::windows_core::HRESULT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENOLEOPFAILED {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENOLEOPFAILED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENOLEOPFAILED {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENOLEOPFAILED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENPROTECTED {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENPROTECTED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENPROTECTED {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENPROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENPROTECTED {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
    pub chrg: CHARRANGE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENPROTECTED {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENPROTECTED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENPROTECTED {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENPROTECTED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENSAVECLIPBOARD {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENSAVECLIPBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENSAVECLIPBOARD {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENSAVECLIPBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct ENSAVECLIPBOARD {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub cch: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENSAVECLIPBOARD {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENSAVECLIPBOARD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for ENSAVECLIPBOARD {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENSAVECLIPBOARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FINDTEXTA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FINDTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for FINDTEXTA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct FINDTEXTA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FINDTEXTA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FINDTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for FINDTEXTA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FINDTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FINDTEXTEXA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FINDTEXTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for FINDTEXTEXA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct FINDTEXTEXA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FINDTEXTEXA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FINDTEXTEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for FINDTEXTEXA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FINDTEXTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FINDTEXTEXW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FINDTEXTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for FINDTEXTEXW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct FINDTEXTEXW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCWSTR,
    pub chrgText: CHARRANGE,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FINDTEXTEXW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FINDTEXTEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for FINDTEXTEXW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FINDTEXTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FINDTEXTW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FINDTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for FINDTEXTW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct FINDTEXTW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FINDTEXTW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FINDTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for FINDTEXTW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FINDTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FORMATRANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FORMATRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for FORMATRANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct FORMATRANGE {
    pub hdc: super::super::super::Graphics::Gdi::HDC,
    pub hdcTarget: super::super::super::Graphics::Gdi::HDC,
    pub rc: super::super::super::Foundation::RECT,
    pub rcPage: super::super::super::Foundation::RECT,
    pub chrg: CHARRANGE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for FORMATRANGE {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for FORMATRANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::TypeKind for FORMATRANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for FORMATRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GETCONTEXTMENUEX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GETCONTEXTMENUEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GETCONTEXTMENUEX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct GETCONTEXTMENUEX {
    pub chrg: CHARRANGE,
    pub dwFlags: u32,
    pub pt: super::super::super::Foundation::POINT,
    pub pvReserved: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GETCONTEXTMENUEX {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GETCONTEXTMENUEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GETCONTEXTMENUEX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GETCONTEXTMENUEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: ::windows_core::PCSTR,
    pub lpUsedDefChar: *mut i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for GETTEXTEX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for GETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for GETTEXTEX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct GETTEXTEX {
    pub cb: u32,
    pub flags: GETTEXTEX_FLAGS,
    pub codepage: u32,
    pub lpDefaultChar: ::windows_core::PCSTR,
    pub lpUsedDefChar: *mut i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for GETTEXTEX {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for GETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for GETTEXTEX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for GETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct GETTEXTLENGTHEX {
    pub flags: GETTEXTLENGTHEX_FLAGS,
    pub codepage: u32,
}
impl ::core::marker::Copy for GETTEXTLENGTHEX {}
impl ::core::clone::Clone for GETTEXTLENGTHEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GETTEXTLENGTHEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GETTEXTLENGTHEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
impl ::windows_core::TypeKind for GETTEXTLENGTHEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GETTEXTLENGTHEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for GETTEXTLENGTHEX {}
impl ::core::default::Default for GETTEXTLENGTHEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUPTYPINGCHANGE {
    pub nmhdr: super::NMHDR,
    pub fGroupTyping: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUPTYPINGCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUPTYPINGCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GROUPTYPINGCHANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUPTYPINGCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for HYPHENATEINFO {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for HYPHENATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for HYPHENATEINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct HYPHENATEINFO {
    pub cbSize: i16,
    pub dxHyphenateZone: i16,
    pub pfnHyphenate: isize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for HYPHENATEINFO {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for HYPHENATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for HYPHENATEINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for HYPHENATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct HYPHRESULT {
    pub khyph: KHYPH,
    pub ichHyph: i32,
    pub chHyph: u16,
}
impl ::core::marker::Copy for HYPHRESULT {}
impl ::core::clone::Clone for HYPHRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPHRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPHRESULT").field("khyph", &self.khyph).field("ichHyph", &self.ichHyph).field("chHyph", &self.chHyph).finish()
    }
}
impl ::windows_core::TypeKind for HYPHRESULT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for HYPHRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.khyph == other.khyph && self.ichHyph == other.ichHyph && self.chHyph == other.chHyph
    }
}
impl ::core::cmp::Eq for HYPHRESULT {}
impl ::core::default::Default for HYPHRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct IMECOMPTEXT {
    pub cb: i32,
    pub flags: IMECOMPTEXT_FLAGS,
}
impl ::core::marker::Copy for IMECOMPTEXT {}
impl ::core::clone::Clone for IMECOMPTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMECOMPTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMECOMPTEXT").field("cb", &self.cb).field("flags", &self.flags).finish()
    }
}
impl ::windows_core::TypeKind for IMECOMPTEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IMECOMPTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IMECOMPTEXT {}
impl ::core::default::Default for IMECOMPTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSGFILTER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSGFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for MSGFILTER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSGFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct MSGFILTER {
    pub nmhdr: super::NMHDR,
    pub msg: u32,
    pub wParam: super::super::super::Foundation::WPARAM,
    pub lParam: super::super::super::Foundation::LPARAM,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSGFILTER {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSGFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for MSGFILTER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSGFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTPOSITIONS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTPOSITIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for OBJECTPOSITIONS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECTPOSITIONS {
    pub nmhdr: super::NMHDR,
    pub cObjectCount: i32,
    pub pcpPositions: *mut i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OBJECTPOSITIONS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OBJECTPOSITIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for OBJECTPOSITIONS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OBJECTPOSITIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct PARAFORMAT {
    pub cbSize: u32,
    pub dwMask: PARAFORMAT_MASK,
    pub wNumbering: PARAFORMAT_NUMBERING,
    pub Anonymous: PARAFORMAT_0,
    pub dxStartIndent: i32,
    pub dxRightIndent: i32,
    pub dxOffset: i32,
    pub wAlignment: PARAFORMAT_ALIGNMENT,
    pub cTabCount: i16,
    pub rgxTabs: [u32; 32],
}
impl ::core::marker::Copy for PARAFORMAT {}
impl ::core::clone::Clone for PARAFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PARAFORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PARAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub union PARAFORMAT_0 {
    pub wReserved: u16,
    pub wEffects: u16,
}
impl ::core::marker::Copy for PARAFORMAT_0 {}
impl ::core::clone::Clone for PARAFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PARAFORMAT_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PARAFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct PARAFORMAT2 {
    pub Base: PARAFORMAT,
    pub dySpaceBefore: i32,
    pub dySpaceAfter: i32,
    pub dyLineSpacing: i32,
    pub sStyle: i16,
    pub bLineSpacingRule: u8,
    pub bOutlineLevel: u8,
    pub wShadingWeight: u16,
    pub wShadingStyle: PARAFORMAT_SHADING_STYLE,
    pub wNumberingStart: u16,
    pub wNumberingStyle: PARAFORMAT_NUMBERING_STYLE,
    pub wNumberingTab: u16,
    pub wBorderSpace: u16,
    pub wBorderWidth: u16,
    pub wBorders: PARAFORMAT_BORDERS,
}
impl ::core::marker::Copy for PARAFORMAT2 {}
impl ::core::clone::Clone for PARAFORMAT2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for PARAFORMAT2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for PARAFORMAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: ::windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PUNCTUATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PUNCTUATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for PUNCTUATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct PUNCTUATION {
    pub iSize: u32,
    pub szPunctuation: ::windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PUNCTUATION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PUNCTUATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for PUNCTUATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for PUNCTUATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
pub struct REOBJECT {
    pub cbStruct: u32,
    pub cp: i32,
    pub clsid: ::windows_core::GUID,
    pub poleobj: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Ole::IOleObject>>,
    pub pstg: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Com::StructuredStorage::IStorage>>,
    pub polesite: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Ole::IOleClientSite>>,
    pub sizel: super::super::super::Foundation::SIZE,
    pub dvaspect: u32,
    pub dwFlags: REOBJECT_FLAGS,
    pub dwUser: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for REOBJECT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for REOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REOBJECT").field("cbStruct", &self.cbStruct).field("cp", &self.cp).field("clsid", &self.clsid).field("poleobj", &self.poleobj).field("pstg", &self.pstg).field("polesite", &self.polesite).field("sizel", &self.sizel).field("dvaspect", &self.dvaspect).field("dwFlags", &self.dwFlags).field("dwUser", &self.dwUser).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::windows_core::TypeKind for REOBJECT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for REOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cp == other.cp && self.clsid == other.clsid && self.poleobj == other.poleobj && self.pstg == other.pstg && self.polesite == other.polesite && self.sizel == other.sizel && self.dvaspect == other.dvaspect && self.dwFlags == other.dwFlags && self.dwUser == other.dwUser
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for REOBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for REOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for REPASTESPECIAL {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for REPASTESPECIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::TypeKind for REPASTESPECIAL {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for REPASTESPECIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct REPASTESPECIAL {
    pub dwAspect: super::super::super::System::Com::DVASPECT,
    pub dwParam: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for REPASTESPECIAL {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for REPASTESPECIAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::TypeKind for REPASTESPECIAL {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for REPASTESPECIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQRESIZE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQRESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for REQRESIZE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQRESIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct REQRESIZE {
    pub nmhdr: super::NMHDR,
    pub rc: super::super::super::Foundation::RECT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REQRESIZE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REQRESIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for REQRESIZE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQRESIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: ::windows_core::PCWSTR,
    pub pIStream: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Com::IStream>>,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::TypeKind for RICHEDIT_IMAGE_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_System_Com\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
pub struct RICHEDIT_IMAGE_PARAMETERS {
    pub xWidth: i32,
    pub yHeight: i32,
    pub Ascent: i32,
    pub Type: i32,
    pub pwszAlternateText: ::windows_core::PCWSTR,
    pub pIStream: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::System::Com::IStream>>,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RICHEDIT_IMAGE_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::TypeKind for RICHEDIT_IMAGE_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RICHEDIT_IMAGE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SELCHANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for SELCHANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SELCHANGE {
    pub nmhdr: super::NMHDR,
    pub chrg: CHARRANGE,
    pub seltyp: RICH_EDIT_GET_CONTEXT_MENU_SEL_TYPE,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SELCHANGE {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SELCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for SELCHANGE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SELCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct SETTEXTEX {
    pub flags: u32,
    pub codepage: u32,
}
impl ::core::marker::Copy for SETTEXTEX {}
impl ::core::clone::Clone for SETTEXTEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SETTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SETTEXTEX").field("flags", &self.flags).field("codepage", &self.codepage).finish()
    }
}
impl ::windows_core::TypeKind for SETTEXTEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SETTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.codepage == other.codepage
    }
}
impl ::core::cmp::Eq for SETTEXTEX {}
impl ::core::default::Default for SETTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TABLECELLPARMS {
    pub dxWidth: i32,
    pub _bitfield: u16,
    pub wShading: u16,
    pub dxBrdrLeft: i16,
    pub dyBrdrTop: i16,
    pub dxBrdrRight: i16,
    pub dyBrdrBottom: i16,
    pub crBrdrLeft: super::super::super::Foundation::COLORREF,
    pub crBrdrTop: super::super::super::Foundation::COLORREF,
    pub crBrdrRight: super::super::super::Foundation::COLORREF,
    pub crBrdrBottom: super::super::super::Foundation::COLORREF,
    pub crBackPat: super::super::super::Foundation::COLORREF,
    pub crForePat: super::super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TABLECELLPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TABLECELLPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TABLECELLPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLECELLPARMS")
            .field("dxWidth", &self.dxWidth)
            .field("_bitfield", &self._bitfield)
            .field("wShading", &self.wShading)
            .field("dxBrdrLeft", &self.dxBrdrLeft)
            .field("dyBrdrTop", &self.dyBrdrTop)
            .field("dxBrdrRight", &self.dxBrdrRight)
            .field("dyBrdrBottom", &self.dyBrdrBottom)
            .field("crBrdrLeft", &self.crBrdrLeft)
            .field("crBrdrTop", &self.crBrdrTop)
            .field("crBrdrRight", &self.crBrdrRight)
            .field("crBrdrBottom", &self.crBrdrBottom)
            .field("crBackPat", &self.crBackPat)
            .field("crForePat", &self.crForePat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for TABLECELLPARMS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TABLECELLPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dxWidth == other.dxWidth && self._bitfield == other._bitfield && self.wShading == other.wShading && self.dxBrdrLeft == other.dxBrdrLeft && self.dyBrdrTop == other.dyBrdrTop && self.dxBrdrRight == other.dxBrdrRight && self.dyBrdrBottom == other.dyBrdrBottom && self.crBrdrLeft == other.crBrdrLeft && self.crBrdrTop == other.crBrdrTop && self.crBrdrRight == other.crBrdrRight && self.crBrdrBottom == other.crBrdrBottom && self.crBackPat == other.crBackPat && self.crForePat == other.crForePat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TABLECELLPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TABLECELLPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub struct TABLEROWPARMS {
    pub cbRow: u8,
    pub cbCell: u8,
    pub cCell: u8,
    pub cRow: u8,
    pub dxCellMargin: i32,
    pub dxIndent: i32,
    pub dyHeight: i32,
    pub _bitfield: u32,
    pub cpStartRow: i32,
    pub bTableLevel: u8,
    pub iCell: u8,
}
impl ::core::marker::Copy for TABLEROWPARMS {}
impl ::core::clone::Clone for TABLEROWPARMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TABLEROWPARMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TABLEROWPARMS").field("cbRow", &self.cbRow).field("cbCell", &self.cbCell).field("cCell", &self.cCell).field("cRow", &self.cRow).field("dxCellMargin", &self.dxCellMargin).field("dxIndent", &self.dxIndent).field("dyHeight", &self.dyHeight).field("_bitfield", &self._bitfield).field("cpStartRow", &self.cpStartRow).field("bTableLevel", &self.bTableLevel).field("iCell", &self.iCell).finish()
    }
}
impl ::windows_core::TypeKind for TABLEROWPARMS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TABLEROWPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbRow == other.cbRow && self.cbCell == other.cbCell && self.cCell == other.cCell && self.cRow == other.cRow && self.dxCellMargin == other.dxCellMargin && self.dxIndent == other.dxIndent && self.dyHeight == other.dyHeight && self._bitfield == other._bitfield && self.cpStartRow == other.cpStartRow && self.bTableLevel == other.bTableLevel && self.iCell == other.iCell
    }
}
impl ::core::cmp::Eq for TABLEROWPARMS {}
impl ::core::default::Default for TABLEROWPARMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for TEXTRANGEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for TEXTRANGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for TEXTRANGEA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct TEXTRANGEA {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for TEXTRANGEA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for TEXTRANGEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for TEXTRANGEA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for TEXTRANGEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for TEXTRANGEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for TEXTRANGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows_core::TypeKind for TEXTRANGEW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
#[cfg(target_arch = "x86")]
pub struct TEXTRANGEW {
    pub chrg: CHARRANGE,
    pub lpstrText: ::windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for TEXTRANGEW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for TEXTRANGEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows_core::TypeKind for TEXTRANGEW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for TEXTRANGEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type AutoCorrectProc = ::core::option::Option<unsafe extern "system" fn(langid: u16, pszbefore: ::windows_core::PCWSTR, pszafter: ::windows_core::PCWSTR, cchafter: i32, pcchreplaced: *mut i32) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type EDITSTREAMCALLBACK = ::core::option::Option<unsafe extern "system" fn(dwcookie: usize, pbbuff: *mut u8, cb: i32, pcb: *mut i32) -> u32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type EDITWORDBREAKPROCEX = ::core::option::Option<unsafe extern "system" fn(pchtext: ::windows_core::PCSTR, cchtext: i32, bcharset: u8, action: i32) -> i32>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PCreateTextServices = ::core::option::Option<unsafe extern "system" fn(punkouter: ::core::option::Option<::windows_core::IUnknown>, pitexthost: ::core::option::Option<ITextHost>, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_UI_Controls_RichEdit\"`*"]
pub type PShutdownTextServices = ::core::option::Option<unsafe extern "system" fn(ptextservices: ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
