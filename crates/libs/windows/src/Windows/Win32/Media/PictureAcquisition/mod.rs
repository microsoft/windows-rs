#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquire(::windows::core::IUnknown);
impl IPhotoAcquire {
    pub unsafe fn CreatePhotoSource<P0>(&self, pszdevice: P0) -> ::windows::core::Result<IPhotoAcquireSource>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IPhotoAcquireSource>();
        (::windows::core::Interface::vtable(self).CreatePhotoSource)(::windows::core::Interface::as_raw(self), pszdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Acquire<P0, P1, P2, P3, P4>(&self, pphotoacquiresource: P0, fshowprogress: P1, hwndparent: P2, pszapplicationname: P3, pphotoacquireprogresscb: P4) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireSource>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P4: ::windows::core::IntoParam<IPhotoAcquireProgressCB>,
    {
        (::windows::core::Interface::vtable(self).Acquire)(::windows::core::Interface::as_raw(self), pphotoacquiresource.into_param().abi(), fshowprogress.into_param().abi(), hwndparent.into_param().abi(), pszapplicationname.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumResults(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumString>();
        (::windows::core::Interface::vtable(self).EnumResults)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquire, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquire {
    type Vtable = IPhotoAcquire_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquire {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquire {
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
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetTitle)(::windows::core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    pub unsafe fn SetSubmitButtonText<P0>(&self, pszsubmitbuttontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSubmitButtonText)(::windows::core::Interface::as_raw(self), pszsubmitbuttontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, dwdeviceflags: u32, pbstrdeviceid: ::core::option::Option<*mut ::windows::core::BSTR>, pndevicetype: ::core::option::Option<*mut DEVICE_SELECTION_DEVICE_TYPE>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).DoModal)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), dwdeviceflags, ::core::mem::transmute(pbstrdeviceid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pndevicetype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireDeviceSelectionDialog, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireDeviceSelectionDialog {
    type Vtable = IPhotoAcquireDeviceSelectionDialog_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireDeviceSelectionDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireDeviceSelectionDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f28837_55dd_4f37_aaf5_6855a9640467);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireDeviceSelectionDialog_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitle: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubmitbuttontext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwdeviceflags: u32, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pndevicetype: *mut DEVICE_SELECTION_DEVICE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoModal: usize,
}
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`*"]
#[repr(transparent)]
pub struct IPhotoAcquireItem(::windows::core::IUnknown);
impl IPhotoAcquireItem {
    pub unsafe fn GetItemName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetItemName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetThumbnail(&self, sizethumbnail: super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__ = ::windows::core::zeroed::<super::super::Graphics::Gdi::HBITMAP>();
        (::windows::core::Interface::vtable(self).GetThumbnail)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sizethumbnail), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::PROPVARIANT>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), key, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), key, pv).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).GetStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanDelete(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanDelete)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSubItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSubItemCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSubItemAt(&self, nitemindex: u32) -> ::windows::core::Result<IPhotoAcquireItem> {
        let mut result__ = ::windows::core::zeroed::<IPhotoAcquireItem>();
        (::windows::core::Interface::vtable(self).GetSubItemAt)(::windows::core::Interface::as_raw(self), nitemindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireItem, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireItem {
    type Vtable = IPhotoAcquireItem_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f21c97_28bf_4c02_b842_5e4e90139a30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetItemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstritemname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
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
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszregistryroot.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<P0>(&self, hwndparent: P0) -> ::windows::core::Result<super::super::Foundation::HWND>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Destroy)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoModal<P0>(&self, hwndparent: P0, ppnreturncode: ::core::option::Option<*mut isize>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).DoModal)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), ::core::mem::transmute(ppnreturncode.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireOptionsDialog, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireOptionsDialog {
    type Vtable = IPhotoAcquireOptionsDialog_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireOptionsDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireOptionsDialog {
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
        P0: ::windows::core::IntoParam<IPhotoAcquireSource>,
        P1: ::windows::core::IntoParam<IPhotoAcquireProgressCB>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pphotoacquiresource.into_param().abi(), pphotoacquireprogresscb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ProcessItem<P0, P1, P2, P3>(&self, dwacquirestage: u32, pphotoacquireitem: P0, poriginalitemstream: P1, pszfinalfilename: P2, ppropertystore: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<super::super::UI::Shell::PropertiesSystem::IPropertyStore>,
    {
        (::windows::core::Interface::vtable(self).ProcessItem)(::windows::core::Interface::as_raw(self), dwacquirestage, pphotoacquireitem.into_param().abi(), poriginalitemstream.into_param().abi(), pszfinalfilename.into_param().abi(), ppropertystore.into_param().abi()).ok()
    }
    pub unsafe fn TransferComplete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TransferComplete)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayConfigureDialog<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).DisplayConfigureDialog)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquirePlugin, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquirePlugin {
    type Vtable = IPhotoAcquirePlugin_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquirePlugin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquirePlugin {
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
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Cancelled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartEnumeration<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireSource>,
    {
        (::windows::core::Interface::vtable(self).StartEnumeration)(::windows::core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    pub unsafe fn FoundItem<P0>(&self, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
    {
        (::windows::core::Interface::vtable(self).FoundItem)(::windows::core::Interface::as_raw(self), pphotoacquireitem.into_param().abi()).ok()
    }
    pub unsafe fn EndEnumeration(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndEnumeration)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn StartTransfer<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireSource>,
    {
        (::windows::core::Interface::vtable(self).StartTransfer)(::windows::core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    pub unsafe fn StartItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
    {
        (::windows::core::Interface::vtable(self).StartItemTransfer)(::windows::core::Interface::as_raw(self), nitemindex, pphotoacquireitem.into_param().abi()).ok()
    }
    pub unsafe fn DirectoryCreated<P0>(&self, pszdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DirectoryCreated)(::windows::core::Interface::as_raw(self), pszdirectory.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTransferPercent<P0>(&self, foverall: P0, npercent: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).UpdateTransferPercent)(::windows::core::Interface::as_raw(self), foverall.into_param().abi(), npercent).ok()
    }
    pub unsafe fn EndItemTransfer<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
    {
        (::windows::core::Interface::vtable(self).EndItemTransfer)(::windows::core::Interface::as_raw(self), nitemindex, pphotoacquireitem.into_param().abi(), hr).ok()
    }
    pub unsafe fn EndTransfer(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndTransfer)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn StartDelete<P0>(&self, pphotoacquiresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireSource>,
    {
        (::windows::core::Interface::vtable(self).StartDelete)(::windows::core::Interface::as_raw(self), pphotoacquiresource.into_param().abi()).ok()
    }
    pub unsafe fn StartItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
    {
        (::windows::core::Interface::vtable(self).StartItemDelete)(::windows::core::Interface::as_raw(self), nitemindex, pphotoacquireitem.into_param().abi()).ok()
    }
    pub unsafe fn UpdateDeletePercent(&self, npercent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateDeletePercent)(::windows::core::Interface::as_raw(self), npercent).ok()
    }
    pub unsafe fn EndItemDelete<P0>(&self, nitemindex: u32, pphotoacquireitem: P0, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoAcquireItem>,
    {
        (::windows::core::Interface::vtable(self).EndItemDelete)(::windows::core::Interface::as_raw(self), nitemindex, pphotoacquireitem.into_param().abi(), hr).ok()
    }
    pub unsafe fn EndDelete(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDelete)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn EndSession(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndSession)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeleteAfterAcquire(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetDeleteAfterAcquire)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ErrorAdvise<P0>(&self, hr: ::windows::core::HRESULT, pszerrormessage: P0, nmessagetype: ERROR_ADVISE_MESSAGE_TYPE) -> ::windows::core::Result<ERROR_ADVISE_RESULT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ERROR_ADVISE_RESULT>();
        (::windows::core::Interface::vtable(self).ErrorAdvise)(::windows::core::Interface::as_raw(self), hr, pszerrormessage.into_param().abi(), nmessagetype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const ::windows::core::GUID, punknown: P0, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).GetUserInput)(::windows::core::Interface::as_raw(self), riidtype, punknown.into_param().abi(), ppropvarresult, ::core::mem::transmute(ppropvardefault.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireProgressCB, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireProgressCB {
    type Vtable = IPhotoAcquireProgressCB_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireProgressCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireProgressCB {
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
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromRegistry)(::windows::core::Interface::as_raw(self), pszregistrykey.into_param().abi()).ok()
    }
    pub unsafe fn SetFlags(&self, dwphotoacquireflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), dwphotoacquireflags).ok()
    }
    pub unsafe fn SetOutputFilenameTemplate<P0>(&self, psztemplate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetOutputFilenameTemplate)(::windows::core::Interface::as_raw(self), psztemplate.into_param().abi()).ok()
    }
    pub unsafe fn SetSequencePaddingWidth(&self, dwwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSequencePaddingWidth)(::windows::core::Interface::as_raw(self), dwwidth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSequenceZeroPadding<P0>(&self, fzeropad: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetSequenceZeroPadding)(::windows::core::Interface::as_raw(self), fzeropad.into_param().abi()).ok()
    }
    pub unsafe fn SetGroupTag<P0>(&self, pszgrouptag: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetGroupTag)(::windows::core::Interface::as_raw(self), pszgrouptag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAcquisitionTime(&self, pftacquisitiontime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAcquisitionTime)(::windows::core::Interface::as_raw(self), pftacquisitiontime).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOutputFilenameTemplate(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetOutputFilenameTemplate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSequencePaddingWidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSequencePaddingWidth)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSequenceZeroPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetSequenceZeroPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGroupTag(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetGroupTag)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAcquisitionTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
        (::windows::core::Interface::vtable(self).GetAcquisitionTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireSettings, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireSettings {
    type Vtable = IPhotoAcquireSettings_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireSettings {
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
    pub GetOutputFilenameTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplate: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetSequencePaddingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSequenceZeroPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfzeropad: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSequenceZeroPadding: usize,
    pub GetGroupTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrgrouptag: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
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
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetFriendlyName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetDeviceIcons(&self, nsize: u32, phlargeicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>, phsmallicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceIcons)(::windows::core::Interface::as_raw(self), nsize, ::core::mem::transmute(phlargeicon.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phsmallicon.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeItemList<P0, P1>(&self, fforceenumeration: P0, pphotoacquireprogresscb: P1, pnitemcount: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<IPhotoAcquireProgressCB>,
    {
        (::windows::core::Interface::vtable(self).InitializeItemList)(::windows::core::Interface::as_raw(self), fforceenumeration.into_param().abi(), pphotoacquireprogresscb.into_param().abi(), ::core::mem::transmute(pnitemcount.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetItemCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItemAt(&self, nindex: u32) -> ::windows::core::Result<IPhotoAcquireItem> {
        let mut result__ = ::windows::core::zeroed::<IPhotoAcquireItem>();
        (::windows::core::Interface::vtable(self).GetItemAt)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPhotoAcquireSettings(&self) -> ::windows::core::Result<IPhotoAcquireSettings> {
        let mut result__ = ::windows::core::zeroed::<IPhotoAcquireSettings>();
        (::windows::core::Interface::vtable(self).GetPhotoAcquireSettings)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDeviceId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BindToObject(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindToObject)(::windows::core::Interface::as_raw(self), riid, ppv).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoAcquireSource, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoAcquireSource {
    type Vtable = IPhotoAcquireSource_Vtbl;
}
impl ::core::clone::Clone for IPhotoAcquireSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoAcquireSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f2c703_8613_4282_a53b_6ec59c5883ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoAcquireSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfriendlyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
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
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
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
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).DoAction)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoProgressActionCB, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoProgressActionCB {
    type Vtable = IPhotoProgressActionCB_Vtbl;
}
impl ::core::clone::Clone for IPhotoProgressActionCB {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoProgressActionCB {
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
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Destroy)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetTitle)(::windows::core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCheckbox<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ShowCheckbox)(::windows::core::Interface::as_raw(self), ncheckboxid, fshow.into_param().abi()).ok()
    }
    pub unsafe fn SetCheckboxText<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCheckboxText)(::windows::core::Interface::as_raw(self), ncheckboxid, pszcheckboxtext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCheckboxCheck<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, fchecked: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetCheckboxCheck)(::windows::core::Interface::as_raw(self), ncheckboxid, fchecked.into_param().abi()).ok()
    }
    pub unsafe fn SetCheckboxTooltip<P0>(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID, pszcheckboxtooltiptext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCheckboxTooltip)(::windows::core::Interface::as_raw(self), ncheckboxid, pszcheckboxtooltiptext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCheckboxChecked(&self, ncheckboxid: PROGRESS_DIALOG_CHECKBOX_ID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsCheckboxChecked)(::windows::core::Interface::as_raw(self), ncheckboxid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCaption<P0>(&self, psztitle: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCaption)(::windows::core::Interface::as_raw(self), psztitle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn SetImage<P0, P1>(&self, nimagetype: PROGRESS_DIALOG_IMAGE_TYPE, hicon: P0, hbitmap: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::UI::WindowsAndMessaging::HICON>,
        P1: ::windows::core::IntoParam<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Interface::vtable(self).SetImage)(::windows::core::Interface::as_raw(self), nimagetype, hicon.into_param().abi(), hbitmap.into_param().abi()).ok()
    }
    pub unsafe fn SetPercentComplete(&self, npercent: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPercentComplete)(::windows::core::Interface::as_raw(self), npercent).ok()
    }
    pub unsafe fn SetProgressText<P0>(&self, pszprogresstext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetProgressText)(::windows::core::Interface::as_raw(self), pszprogresstext.into_param().abi()).ok()
    }
    pub unsafe fn SetActionLinkCallback<P0>(&self, pphotoprogressactioncb: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPhotoProgressActionCB>,
    {
        (::windows::core::Interface::vtable(self).SetActionLinkCallback)(::windows::core::Interface::as_raw(self), pphotoprogressactioncb.into_param().abi()).ok()
    }
    pub unsafe fn SetActionLinkText<P0>(&self, pszcaption: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetActionLinkText)(::windows::core::Interface::as_raw(self), pszcaption.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowActionLink<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ShowActionLink)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCancelled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsCancelled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetUserInput<P0>(&self, riidtype: *const ::windows::core::GUID, punknown: P0, ppropvarresult: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvardefault: ::core::option::Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).GetUserInput)(::windows::core::Interface::as_raw(self), riidtype, punknown.into_param().abi(), ppropvarresult, ::core::mem::transmute(ppropvardefault.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IPhotoProgressDialog, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IPhotoProgressDialog {
    type Vtable = IPhotoProgressDialog_Vtbl;
}
impl ::core::clone::Clone for IPhotoProgressDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhotoProgressDialog {
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
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSubmitButtonText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrompt(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetPrompt)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStringId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStringType(&self) -> ::windows::core::Result<USER_INPUT_STRING_TYPE> {
        let mut result__ = ::windows::core::zeroed::<USER_INPUT_STRING_TYPE>();
        (::windows::core::Interface::vtable(self).GetStringType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTooltipText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetTooltipText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefault(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMruCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMruCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMruEntryAt(&self, nindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetMruEntryAt)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetImage(&self, nsize: u32, phbitmap: ::core::option::Option<*mut super::super::Graphics::Gdi::HBITMAP>, phicon: ::core::option::Option<*mut super::super::UI::WindowsAndMessaging::HICON>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetImage)(::windows::core::Interface::as_raw(self), nsize, ::core::mem::transmute(phbitmap.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(phicon.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IUserInputString, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IUserInputString {
    type Vtable = IUserInputString_Vtbl;
}
impl ::core::clone::Clone for IUserInputString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserInputString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f243a1_205b_45ba_ae26_abbc53aa7a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserInputString_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSubmitButtonText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubmitbuttontext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprompttitle: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetStringId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrstringid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetStringType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnstringtype: *mut USER_INPUT_STRING_TYPE) -> ::windows::core::HRESULT,
    pub GetTooltipText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltiptext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetMaxLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchmaxlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdefault: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetMruCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnmrucount: *mut u32) -> ::windows::core::HRESULT,
    pub GetMruEntryAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pbstrmruentry: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
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
pub const PKEY_PhotoAcquire_CameraSequenceNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 7 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_DuplicateDetectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 10 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_FinalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 3 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_GroupTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 4 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_IntermediateFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 8 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_OriginalFilename: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 6 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_RelativePathname: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 2 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_SkipImport: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 9 };
#[doc = "*Required features: `\"Win32_Media_PictureAcquisition\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PhotoAcquire_TransferResult: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x00f23377_7ac6_4b7a_8443_345e731fa57a), pid: 5 };
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
impl ::windows::core::TypeKind for DEVICE_SELECTION_DEVICE_TYPE {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for ERROR_ADVISE_MESSAGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for ERROR_ADVISE_RESULT {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for PROGRESS_DIALOG_CHECKBOX_ID {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for PROGRESS_DIALOG_IMAGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
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
impl ::windows::core::TypeKind for USER_INPUT_STRING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_INPUT_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_INPUT_STRING_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
