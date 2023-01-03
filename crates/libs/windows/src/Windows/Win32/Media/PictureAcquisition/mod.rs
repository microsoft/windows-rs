#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquire(::windows::core::IUnknown);
impl IPhotoAcquire {
    pub unsafe fn CreatePhotoSource<P0>(&self, pszdevice: P0) -> ::windows::core::Result<IPhotoAcquireSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePhotoSource)(::windows::core::Vtable::as_raw(self), pszdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Acquire<P0, P1, P2, P3, P4>(&self, pphotoacquiresource: P0, fshowprogress: P1, hwndparent: P2, pszapplicationname: P3, pphotoacquireprogresscb: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireSource>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::HWND>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireProgressCB>>,
    {
        (::windows::core::Vtable::vtable(self).Acquire)(::windows::core::Vtable::as_raw(self), pphotoacquiresource.into().abi(), fshowprogress.into(), hwndparent.into(), pszapplicationname.into().abi(), pphotoacquireprogresscb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumResults(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumResults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquire, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquire {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquire {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquire {}
impl ::core::fmt::Debug for IPhotoAcquire {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquire").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquire {
    type Vtable = IPhotoAcquire_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquire {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f23353_e31b_4955_a8ad_ca5ebf31e2ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquire_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreatePhotoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdevice: ::windows::core::PCWSTR, ppphotoacquiresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Acquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, fshowprogress: super::super::Foundation::BOOL, hwndparent: super::super::Foundation::HWND, pszapplicationname: ::windows::core::PCWSTR, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Acquire: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumfilepaths: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumResults: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireDeviceSelectionDialog(::windows::core::IUnknown);
impl IPhotoAcquireDeviceSelectionDialog {
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTitle)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    pub unsafe fn SetSubmitButtonText<P0>(&self, pszsubmitbuttontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSubmitButtonText)(::windows::core::Vtable::as_raw(self), pszsubmitbuttontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, dwdeviceflags: u32, pbstrdeviceid: ::core::option::Option<*mut ::windows::core::BSTR>, pndevicetype: ::core::option::Option<*mut DEVICE_SELECTION_DEVICE_TYPE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DoModal)(::windows::core::Vtable::as_raw(self), hwndparent.into(), dwdeviceflags, ::core::mem::transmute(pbstrdeviceid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pndevicetype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireDeviceSelectionDialog, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireDeviceSelectionDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireDeviceSelectionDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireDeviceSelectionDialog {}
impl ::core::fmt::Debug for IPhotoAcquireDeviceSelectionDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireDeviceSelectionDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireDeviceSelectionDialog {
    type Vtable = IPhotoAcquireDeviceSelectionDialog_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireDeviceSelectionDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f28837_55dd_4f37_aaf5_6855a9640467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut *mut ::core::ffi::c_void, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireItem(::windows::core::IUnknown);
impl IPhotoAcquireItem {
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetThumbnail(&self, sizethumbnail: super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThumbnail)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(sizethumbnail), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), key, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), key, pv).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanDelete(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CanDelete)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSubItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> ::windows::core::Result<IPhotoAcquireItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubItemAt)(::windows::core::Vtable::as_raw(self), nitemindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireItem, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireItem {}
impl ::core::fmt::Debug for IPhotoAcquireItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireItem {
    type Vtable = IPhotoAcquireItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f21c97_28bf_4c02_b842_5e4e90139a30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstritemname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizethumbnail: super::super::Foundation::SIZE, phbmpthumbnail: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetThumbnail: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    SetProperty: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcandelete: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanDelete: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSubItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSubItemAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireOptionsDialog(::windows::core::IUnknown);
impl IPhotoAcquireOptionsDialog {
    pub unsafe fn Initialize<P0>(&self, pszregistryroot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pszregistryroot.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), hwndparent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Destroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, ppnreturncode: ::core::option::Option<*mut isize>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DoModal)(::windows::core::Vtable::as_raw(self), hwndparent.into(), ::core::mem::transmute(ppnreturncode.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SaveData)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireOptionsDialog, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireOptionsDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireOptionsDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireOptionsDialog {}
impl ::core::fmt::Debug for IPhotoAcquireOptionsDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireOptionsDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireOptionsDialog {
    type Vtable = IPhotoAcquireOptionsDialog_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireOptionsDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2b3ee_bf64_47ee_89f4_4dedd79643f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireOptionsDialog_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszregistryroot: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwnddialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, ppnreturncode: *mut isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
    pub SaveData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquirePlugin(::windows::core::IUnknown);
impl IPhotoAcquirePlugin {
    pub unsafe fn Initialize<P0, P1>(&self, pphotoacquiresource: P0, pphotoacquireprogresscb: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireProgressCB>>,
    {
        (::windows::core::Vtable::vtable(self).Initialize)(::windows::core::Vtable::as_raw(self), pphotoacquiresource.into().abi(), pphotoacquireprogresscb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ProcessItem<P0, P1, P2, P3>(&self, dwacquirestage: u32, pphotoacquireitem: P0, poriginalitemstream: P1, pszfinalfilename: P2, ppropertystore: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Vtable::vtable(self).ProcessItem)(::windows::core::Vtable::as_raw(self), dwacquirestage, pphotoacquireitem.into().abi(), poriginalitemstream.into().abi(), pszfinalfilename.into().abi(), ppropertystore.into().abi()).ok()
    }
    pub unsafe fn TransferComplete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).TransferComplete)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayConfigureDialog<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DisplayConfigureDialog)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquirePlugin, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquirePlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquirePlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquirePlugin {}
impl ::core::fmt::Debug for IPhotoAcquirePlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquirePlugin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquirePlugin {
    type Vtable = IPhotoAcquirePlugin_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquirePlugin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2dceb_ecb8_4f77_8e47_e7a987c83dd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquirePlugin_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void, pphotoacquireprogresscb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ProcessItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwacquirestage: u32, pphotoacquireitem: *mut ::core::ffi::c_void, poriginalitemstream: *mut ::core::ffi::c_void, pszfinalfilename: ::windows::core::PCWSTR, ppropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ProcessItem: usize,
    pub TransferComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayConfigureDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayConfigureDialog: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireProgressCB(::windows::core::IUnknown);
impl IPhotoAcquireProgressCB {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Cancelled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Cancelled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StartEnumeration<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireSource>>,
    {
        (::windows::core::Vtable::vtable(self).StartEnumeration)(::windows::core::Vtable::as_raw(self), pphotoacquiresource.into().abi()).ok()
    }
    pub unsafe fn FoundItem<P0>(&self, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
    {
        (::windows::core::Vtable::vtable(self).FoundItem)(::windows::core::Vtable::as_raw(self), pphotoacquireitem.into().abi()).ok()
    }
    pub unsafe fn EndEnumeration(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndEnumeration)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn StartTransfer<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireSource>>,
    {
        (::windows::core::Vtable::vtable(self).StartTransfer)(::windows::core::Vtable::as_raw(self), pphotoacquiresource.into().abi()).ok()
    }
    pub unsafe fn StartItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
    {
        (::windows::core::Vtable::vtable(self).StartItemTransfer)(::windows::core::Vtable::as_raw(self), nitemindex, pphotoacquireitem.into().abi()).ok()
    }
    pub unsafe fn DirectoryCreated<P0>(&self, pszdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).DirectoryCreated)(::windows::core::Vtable::as_raw(self), pszdirectory.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTransferPercent<P0>(&self, foverall: P0, npercent: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UpdateTransferPercent)(::windows::core::Vtable::as_raw(self), foverall.into(), npercent).ok()
    }
    pub unsafe fn EndItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
    {
        (::windows::core::Vtable::vtable(self).EndItemTransfer)(::windows::core::Vtable::as_raw(self), nitemindex, pphotoacquireitem.into().abi(), hr).ok()
    }
    pub unsafe fn EndTransfer(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndTransfer)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn StartDelete<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireSource>>,
    {
        (::windows::core::Vtable::vtable(self).StartDelete)(::windows::core::Vtable::as_raw(self), pphotoacquiresource.into().abi()).ok()
    }
    pub unsafe fn StartItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
    {
        (::windows::core::Vtable::vtable(self).StartItemDelete)(::windows::core::Vtable::as_raw(self), nitemindex, pphotoacquireitem.into().abi()).ok()
    }
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).UpdateDeletePercent)(::windows::core::Vtable::as_raw(self), npercent).ok()
    }
    pub unsafe fn EndItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireItem>>,
    {
        (::windows::core::Vtable::vtable(self).EndItemDelete)(::windows::core::Vtable::as_raw(self), nitemindex, pphotoacquireitem.into().abi(), hr).ok()
    }
    pub unsafe fn EndDelete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndDelete)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    pub unsafe fn EndSession(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EndSession)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeleteAfterAcquire(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeleteAfterAcquire)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ErrorAdvise<P0>(&self, hr: ::windows::core::HRESULT, pszerrormessage: P0, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows::core::Result<ERROR_ADVISE_RESULT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ErrorAdvise)(::windows::core::Vtable::as_raw(self), hr, pszerrormessage.into().abi(), nmessagetype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const ::windows::core::GUID, punknown: P0, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).GetUserInput)(::windows::core::Vtable::as_raw(self), riidtype, punknown.into().abi(), ppropvarresult, ::core::mem::transmute(ppropvardefault.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireProgressCB, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireProgressCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireProgressCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireProgressCB {}
impl ::core::fmt::Debug for IPhotoAcquireProgressCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireProgressCB").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireProgressCB {
    type Vtable = IPhotoAcquireProgressCB_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireProgressCB {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2ce1e_935e_4248_892c_130f32c45cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireProgressCB_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Cancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Cancelled: usize,
    pub StartEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FoundItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndEnumeration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub StartTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartItemTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DirectoryCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateTransferPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foverall: super::super::Foundation::BOOL, npercent: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateTransferPercent: usize,
    pub EndItemTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub EndTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub StartDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoacquiresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartItemDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateDeletePercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npercent: u32) -> ::windows::core::HRESULT,
    pub EndItemDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemindex: u32, pphotoacquireitem: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub EndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDeleteAfterAcquire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfdeleteafteracquire: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDeleteAfterAcquire: usize,
    pub ErrorAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, pszerrormessage: ::windows::core::PCWSTR, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE, pnerroradviseresult: *mut ERROR_ADVISE_RESULT) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireSettings(::windows::core::IUnknown);
impl IPhotoAcquireSettings {
    pub unsafe fn InitializeFromRegistry<P0>(&self, pszregistrykey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).InitializeFromRegistry)(::windows::core::Vtable::as_raw(self), pszregistrykey.into().abi()).ok()
    }
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFlags)(::windows::core::Vtable::as_raw(self), dwphotoacquireflags).ok()
    }
    pub unsafe fn SetOutputFilenameTemplate<P0>(&self, psztemplate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetOutputFilenameTemplate)(::windows::core::Vtable::as_raw(self), psztemplate.into().abi()).ok()
    }
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSequencePaddingWidth)(::windows::core::Vtable::as_raw(self), dwwidth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSequenceZeroPadding<P0>(&self, fzeropad: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetSequenceZeroPadding)(::windows::core::Vtable::as_raw(self), fzeropad.into()).ok()
    }
    pub unsafe fn SetGroupTag<P0>(&self, pszgrouptag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetGroupTag)(::windows::core::Vtable::as_raw(self), pszgrouptag.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAcquisitionTime)(::windows::core::Vtable::as_raw(self), pftacquisitiontime).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputFilenameTemplate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputFilenameTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSequencePaddingWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSequencePaddingWidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSequenceZeroPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSequenceZeroPadding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGroupTag(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGroupTag)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAcquisitionTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAcquisitionTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireSettings, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireSettings {}
impl ::core::fmt::Debug for IPhotoAcquireSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireSettings {
    type Vtable = IPhotoAcquireSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2b868_dd67_487c_9553_049240767e91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitializeFromRegistry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszregistrykey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwphotoacquireflags: u32) -> ::windows::core::HRESULT,
    pub SetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztemplate: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSequencePaddingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwidth: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSequenceZeroPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fzeropad: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSequenceZeroPadding: usize,
    pub SetGroupTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgrouptag: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAcquisitionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAcquisitionTime: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwphotoacquireflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSequenceZeroPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSequenceZeroPadding: usize,
    pub GetGroupTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAcquisitionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftacquisitiontime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAcquisitionTime: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireSource(::windows::core::IUnknown);
impl IPhotoAcquireSource {
    pub unsafe fn GetFriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFriendlyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>, phsmallicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetDeviceIcons)(::windows::core::Vtable::as_raw(self), nsize, ::core::mem::transmute(phlargeicon.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phsmallicon.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeItemList<P0, P1>(&self, fforceenumeration: P0, pphotoacquireprogresscb: P1, pnitemcount: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IPhotoAcquireProgressCB>>,
    {
        (::windows::core::Vtable::vtable(self).InitializeItemList)(::windows::core::Vtable::as_raw(self), fforceenumeration.into(), pphotoacquireprogresscb.into().abi(), ::core::mem::transmute(pnitemcount.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItemAt(&self, nindex: u32) -> ::windows::core::Result<IPhotoAcquireItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetItemAt)(::windows::core::Vtable::as_raw(self), nindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPhotoAcquireSettings(&self) -> ::windows::core::Result<IPhotoAcquireSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPhotoAcquireSettings)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeviceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BindToObject(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).BindToObject)(::windows::core::Vtable::as_raw(self), riid, ppv).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoAcquireSource, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoAcquireSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoAcquireSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoAcquireSource {}
impl ::core::fmt::Debug for IPhotoAcquireSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoAcquireSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoAcquireSource {
    type Vtable = IPhotoAcquireSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoAcquireSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2c703_8613_4282_a53b_6ec59c5883ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetDeviceIcons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nsize: u32, phlargeicon: *mut super::super::UI::WindowsAndMessaging::HICON, phsmallicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetDeviceIcons: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeItemList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforceenumeration: super::super::Foundation::BOOL, pphotoacquireprogresscb: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeItemList: usize,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnitemcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppphotoacquireitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPhotoAcquireSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppphotoacquiresettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoProgressActionCB(::windows::core::IUnknown);
impl IPhotoProgressActionCB {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoAction<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).DoAction)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoProgressActionCB, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoProgressActionCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoProgressActionCB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoProgressActionCB {}
impl ::core::fmt::Debug for IPhotoProgressActionCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoProgressActionCB").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoProgressActionCB {
    type Vtable = IPhotoProgressActionCB_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoProgressActionCB {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f242d0_b206_4e7d_b4c1_4755bcbb9c9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressActionCB_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DoAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoAction: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoProgressDialog(::windows::core::IUnknown);
impl IPhotoProgressDialog {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).Create)(::windows::core::Vtable::as_raw(self), hwndparent.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Destroy)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTitle)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCheckbox<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ShowCheckbox)(::windows::core::Vtable::as_raw(self), ncheckboxid, fshow.into()).ok()
    }
    pub unsafe fn SetCheckboxText<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCheckboxText)(::windows::core::Vtable::as_raw(self), ncheckboxid, pszcheckboxtext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCheckboxCheck<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetCheckboxCheck)(::windows::core::Vtable::as_raw(self), ncheckboxid, fchecked.into()).ok()
    }
    pub unsafe fn SetCheckboxTooltip<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCheckboxTooltip)(::windows::core::Vtable::as_raw(self), ncheckboxid, pszcheckboxtooltiptext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsCheckboxChecked)(::windows::core::Vtable::as_raw(self), ncheckboxid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCaption<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCaption)(::windows::core::Vtable::as_raw(self), psztitle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetImage<P0, P1>(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: P0, hbitmap: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HICON>,
        P1: ::std::convert::Into<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Vtable::vtable(self).SetImage)(::windows::core::Vtable::as_raw(self), nimagetype, hicon.into(), hbitmap.into()).ok()
    }
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPercentComplete)(::windows::core::Vtable::as_raw(self), npercent).ok()
    }
    pub unsafe fn SetProgressText<P0>(&self, pszprogresstext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetProgressText)(::windows::core::Vtable::as_raw(self), pszprogresstext.into().abi()).ok()
    }
    pub unsafe fn SetActionLinkCallback<P0>(&self, pphotoprogressactioncb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPhotoProgressActionCB>>,
    {
        (::windows::core::Vtable::vtable(self).SetActionLinkCallback)(::windows::core::Vtable::as_raw(self), pphotoprogressactioncb.into().abi()).ok()
    }
    pub unsafe fn SetActionLinkText<P0>(&self, pszcaption: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetActionLinkText)(::windows::core::Vtable::as_raw(self), pszcaption.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowActionLink<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).ShowActionLink)(::windows::core::Vtable::as_raw(self), fshow.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCancelled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsCancelled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const ::windows::core::GUID, punknown: P0, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).GetUserInput)(::windows::core::Vtable::as_raw(self), riidtype, punknown.into().abi(), ppropvarresult, ::core::mem::transmute(ppropvardefault.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::core::interface_hierarchy!(IPhotoProgressDialog, ::windows::core::IUnknown);
impl ::core::clone::Clone for IPhotoProgressDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPhotoProgressDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhotoProgressDialog {}
impl ::core::fmt::Debug for IPhotoProgressDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhotoProgressDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IPhotoProgressDialog {
    type Vtable = IPhotoProgressDialog_Vtbl;
}
unsafe impl ::windows::core::Interface for IPhotoProgressDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f246f9_0750_4f08_9381_2cd8e906a4ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoProgressDialog_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Create: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwndprogressdialog: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWindow: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCheckbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCheckbox: usize,
    pub SetCheckboxText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCheckboxCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCheckboxCheck: usize,
    pub SetCheckboxTooltip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCheckboxChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pfchecked: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCheckboxChecked: usize,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: super::super::UI::WindowsAndMessaging::HICON, hbitmap: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    SetImage: usize,
    pub SetPercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npercent: i32) -> ::windows::core::HRESULT,
    pub SetProgressText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszprogresstext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetActionLinkCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphotoprogressactioncb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetActionLinkText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcaption: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowActionLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowActionLink: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCancelled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCancelled: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riidtype: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetUserInput: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IUserInputString(::windows::core::IUnknown);
impl IUserInputString {
    pub unsafe fn GetSubmitButtonText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubmitButtonText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrompt(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrompt)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringType(&self) -> ::windows::core::Result<USER_INPUT_STRING_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTooltipText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTooltipText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMaxLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDefault(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefault)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMruCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMruCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMruEntryAt)(::windows::core::Vtable::as_raw(self), nindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: ::core::option::Option<*mut super::super::Graphics::Gdi::HBITMAP>, phicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetImage)(::windows::core::Vtable::as_raw(self), nsize, ::core::mem::transmute(phbitmap.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phicon.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::core::interface_hierarchy!(IUserInputString, ::windows::core::IUnknown);
impl ::core::clone::Clone for IUserInputString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserInputString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserInputString {}
impl ::core::fmt::Debug for IUserInputString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserInputString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IUserInputString {
    type Vtable = IUserInputString_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserInputString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f243a1_205b_45ba_ae26_abbc53aa7a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInputString_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstringid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT,
    pub GetTooltipText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdefault: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMruCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT,
    pub GetMruEntryAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nsize: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging")))]
    GetImage: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_ALL_DEVICES: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_CPL_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_DV_DEVICES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_FS_DEVICES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_SHOW_OFFLINE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_STI_DEVICES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_TWAIN_DEVICES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WIA_CAMERAS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WIA_SCANNERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_WPD_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_CLEANUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_POSTSAVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PAPS_PRESAVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ABORT_ON_SETTINGS_UPDATE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DELETE_AFTER_ACQUIRE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_AUTO_ROTATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_DB_INTEGRATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_DUPLICATE_DETECTION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_GROUP_TAG_PROMPT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_METADATA_WRITE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_PLUGINS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_SETTINGS_LINK: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_DISABLE_THUMBNAIL_PROGRESS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ENABLE_THUMBNAIL_CACHING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_ERROR_RESTART_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147180543i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_IMPORT_VIDEO_AS_MULTIPLE_FILES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_NO_GALLERY_LAUNCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQ_RUN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 10u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_INDETERMINATE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoAcquire: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f26e02_e9f2_4a9f_9fdd_5a962fb26a98);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoAcquireAutoPlayDropTarget: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f20eb5_8fd6_4d9d_b75e_36801766c8f1);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoAcquireAutoPlayHWEventHandler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2b433_44e4_4d88_b2b0_2698a0a91dba);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoAcquireDeviceSelectionDialog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f29a34_b8a1_482c_bcf8_3ac7b0fe8f62);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoAcquireOptionsDialog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f210a1_62f0_438b_9f7e_9618d72a1831);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PhotoProgressDialog: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f24ca0_748f_4e8a_894f_0e0357c6799f);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICE_SELECTION_DEVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_UNKNOWN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WPD_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_WIA_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_STI_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DSF_TWAIN_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_FS_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const DST_DV_DEVICE: DEVICE_SELECTION_DEVICE_TYPE = DEVICE_SELECTION_DEVICE_TYPE(6i32);
impl ::core::marker::Copy for DEVICE_SELECTION_DEVICE_TYPE {}
impl ::core::clone::Clone for DEVICE_SELECTION_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICE_SELECTION_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DEVICE_SELECTION_DEVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DEVICE_SELECTION_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_SELECTION_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ERROR_ADVISE_MESSAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_SKIPRETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_RETRYCANCEL: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_YESNO: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_ERROR_OK: ERROR_ADVISE_MESSAGE_TYPE = ERROR_ADVISE_MESSAGE_TYPE(3i32);
impl ::core::marker::Copy for ERROR_ADVISE_MESSAGE_TYPE {}
impl ::core::clone::Clone for ERROR_ADVISE_MESSAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERROR_ADVISE_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ERROR_ADVISE_MESSAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ERROR_ADVISE_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ERROR_ADVISE_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_YES: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_NO: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_OK: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(3i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_SKIP_ALL: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(4i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_RETRY: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(5i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PHOTOACQUIRE_RESULT_ABORT: ERROR_ADVISE_RESULT = ERROR_ADVISE_RESULT(6i32);
impl ::core::marker::Copy for ERROR_ADVISE_RESULT {}
impl ::core::clone::Clone for ERROR_ADVISE_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERROR_ADVISE_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ERROR_ADVISE_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for ERROR_ADVISE_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERROR_ADVISE_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROGRESS_DIALOG_CHECKBOX_ID(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_CHECKBOX_ID_DEFAULT: PROGRESS_DIALOG_CHECKBOX_ID = PROGRESS_DIALOG_CHECKBOX_ID(0i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_CHECKBOX_ID {}
impl ::core::clone::Clone for PROGRESS_DIALOG_CHECKBOX_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROGRESS_DIALOG_CHECKBOX_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROGRESS_DIALOG_CHECKBOX_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROGRESS_DIALOG_CHECKBOX_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_CHECKBOX_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROGRESS_DIALOG_IMAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_SMALL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_LARGE: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_ICON_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const PROGRESS_DIALOG_BITMAP_THUMBNAIL: PROGRESS_DIALOG_IMAGE_TYPE = PROGRESS_DIALOG_IMAGE_TYPE(3i32);
impl ::core::marker::Copy for PROGRESS_DIALOG_IMAGE_TYPE {}
impl ::core::clone::Clone for PROGRESS_DIALOG_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROGRESS_DIALOG_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROGRESS_DIALOG_IMAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROGRESS_DIALOG_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROGRESS_DIALOG_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_INPUT_STRING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_DEFAULT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
pub const USER_INPUT_PATH_ELEMENT: USER_INPUT_STRING_TYPE = USER_INPUT_STRING_TYPE(1i32);
impl ::core::marker::Copy for USER_INPUT_STRING_TYPE {}
impl ::core::clone::Clone for USER_INPUT_STRING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_INPUT_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USER_INPUT_STRING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for USER_INPUT_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_INPUT_STRING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
