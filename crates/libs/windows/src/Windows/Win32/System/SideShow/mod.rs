#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowBulkCapabilities(::windows_core::IUnknown);
impl ISideShowBulkCapabilities {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetCapability)(::windows_core::Interface::as_raw(self), in_keycapability, inout_pvalue).ok()
    }
    pub unsafe fn GetCapabilities<P0>(&self, in_keycollection: P0, inout_pvalues: *mut ::core::option::Option<ISideShowPropVariantCollection>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowKeyCollection>,
    {
        (::windows_core::Interface::vtable(self).GetCapabilities)(::windows_core::Interface::as_raw(self), in_keycollection.into_param().abi(), ::core::mem::transmute(inout_pvalues)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowBulkCapabilities, ::windows_core::IUnknown, ISideShowCapabilities);
unsafe impl ::windows_core::Interface for ISideShowBulkCapabilities {
    type Vtable = ISideShowBulkCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowBulkCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a2b7fbc_3ad5_48bd_bbf1_0e6cfbd10807);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowBulkCapabilities_Vtbl {
    pub base__: ISideShowCapabilities_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_keycollection: *mut ::core::ffi::c_void, inout_pvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowCapabilities(::windows_core::IUnknown);
impl ISideShowCapabilities {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCapability)(::windows_core::Interface::as_raw(self), in_keycapability, inout_pvalue).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowCapabilities, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowCapabilities {
    type Vtable = ISideShowCapabilities_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowCapabilities {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x535e1379_c09e_4a54_a511_597bab3a72b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilities_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub GetCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem")))]
    GetCapability: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowCapabilitiesCollection(::windows_core::IUnknown);
impl ISideShowCapabilitiesCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, in_dwindex: u32) -> ::windows_core::Result<ISideShowCapabilities> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), in_dwindex, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowCapabilitiesCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowCapabilitiesCollection {
    type Vtable = ISideShowCapabilitiesCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowCapabilitiesCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50305597_5e0d_4ff7_b3af_33d0d9bd52dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilitiesCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowContent(::windows_core::IUnknown);
impl ISideShowContent {
    pub unsafe fn GetContent<P0>(&self, in_picapabilities: P0, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowCapabilities>,
    {
        (::windows_core::Interface::vtable(self).GetContent)(::windows_core::Interface::as_raw(self), in_picapabilities.into_param().abi(), out_pdwsize, out_ppbdata).ok()
    }
    pub unsafe fn ContentId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ContentId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DifferentiateContent(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DifferentiateContent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowContent, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowContent {
    type Vtable = ISideShowContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc18552ed_74ff_4fec_be07_4cfed29d4887);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContent_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picapabilities: *mut ::core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows_core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DifferentiateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DifferentiateContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowContentManager(::windows_core::IUnknown);
impl ISideShowContentManager {
    pub unsafe fn Add<P0>(&self, in_picontent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowContent>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), in_picontent.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, in_contentid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), in_contentid).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetEventSink<P0>(&self, in_pievents: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowEvents>,
    {
        (::windows_core::Interface::vtable(self).SetEventSink)(::windows_core::Interface::as_raw(self), in_pievents.into_param().abi()).ok()
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows_core::Result<ISideShowCapabilitiesCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowContentManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowContentManager {
    type Vtable = ISideShowContentManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowContentManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5d5b66b_eef9_41db_8d7e_e17c33ab10b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContentManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picontent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pievents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowEvents(::windows_core::IUnknown);
impl ISideShowEvents {
    pub unsafe fn ContentMissing(&self, in_contentid: u32) -> ::windows_core::Result<ISideShowContent> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ContentMissing)(::windows_core::Interface::as_raw(self), in_contentid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ApplicationEvent<P0>(&self, in_picapabilities: P0, in_dweventid: u32, in_pbeventdata: ::core::option::Option<&[u8]>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowCapabilities>,
    {
        (::windows_core::Interface::vtable(self).ApplicationEvent)(::windows_core::Interface::as_raw(self), in_picapabilities.into_param().abi(), in_dweventid, in_pbeventdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(in_pbeventdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
    pub unsafe fn DeviceAdded<P0>(&self, in_pidevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowCapabilities>,
    {
        (::windows_core::Interface::vtable(self).DeviceAdded)(::windows_core::Interface::as_raw(self), in_pidevice.into_param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<P0>(&self, in_pidevice: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowCapabilities>,
    {
        (::windows_core::Interface::vtable(self).DeviceRemoved)(::windows_core::Interface::as_raw(self), in_pidevice.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowEvents {
    type Vtable = ISideShowEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61feca4c_deb4_4a7e_8d75_51f1132d615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ContentMissing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ApplicationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picapabilities: *mut ::core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows_core::HRESULT,
    pub DeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowKeyCollection(::windows_core::IUnknown);
impl ISideShowKeyCollection {
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), key).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), dwindex, pkey).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), pcelems).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), dwindex).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowKeyCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowKeyCollection {
    type Vtable = ISideShowKeyCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowKeyCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x045473bc_a37b_4957_b144_68105411ed8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowKeyCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowNotification(::windows_core::IUnknown);
impl ISideShowNotification {
    pub unsafe fn NotificationId(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).NotificationId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotificationId(&self, in_notificationid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNotificationId)(::windows_core::Interface::as_raw(self), in_notificationid).ok()
    }
    pub unsafe fn Title(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, in_pwsztitle: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), in_pwsztitle.into_param().abi()).ok()
    }
    pub unsafe fn Message(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMessage<P0>(&self, in_pwszmessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetMessage)(::windows_core::Interface::as_raw(self), in_pwszmessage.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Image)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetImage<P0>(&self, in_hicon: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::UI::WindowsAndMessaging::HICON>,
    {
        (::windows_core::Interface::vtable(self).SetImage)(::windows_core::Interface::as_raw(self), in_hicon.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationTime(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExpirationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationTime(&self, in_ptime: ::core::option::Option<*const super::super::Foundation::SYSTEMTIME>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetExpirationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(in_ptime.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowNotification, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowNotification {
    type Vtable = ISideShowNotification_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowNotification {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03c93300_8ab2_41c5_9b79_46127a30e148);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotification_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub NotificationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows_core::HRESULT,
    pub SetNotificationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pwsztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pwszmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Image: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpirationTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExpirationTime: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowNotificationManager(::windows_core::IUnknown);
impl ISideShowNotificationManager {
    pub unsafe fn Show<P0>(&self, in_pinotification: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ISideShowNotification>,
    {
        (::windows_core::Interface::vtable(self).Show)(::windows_core::Interface::as_raw(self), in_pinotification.into_param().abi()).ok()
    }
    pub unsafe fn Revoke(&self, in_notificationid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Revoke)(::windows_core::Interface::as_raw(self), in_notificationid).ok()
    }
    pub unsafe fn RevokeAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevokeAll)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowNotificationManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowNotificationManager {
    type Vtable = ISideShowNotificationManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowNotificationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63cea909_f2b9_4302_b5e1_c68e6d9ab833);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotificationManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pinotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows_core::HRESULT,
    pub RevokeAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowPropVariantCollection(::windows_core::IUnknown);
impl ISideShowPropVariantCollection {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pvalue).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), dwindex, pvalue).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), pcelems).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), dwindex).ok()
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowPropVariantCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowPropVariantCollection {
    type Vtable = ISideShowPropVariantCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowPropVariantCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ea7a549_7bff_4aae_bab0_22d43111de49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowPropVariantCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant")))]
    GetAt: usize,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISideShowSession(::windows_core::IUnknown);
impl ISideShowSession {
    pub unsafe fn RegisterContent(&self, in_applicationid: *const ::windows_core::GUID, in_endpointid: *const ::windows_core::GUID) -> ::windows_core::Result<ISideShowContentManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterContent)(::windows_core::Interface::as_raw(self), in_applicationid, in_endpointid, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterNotifications(&self, in_applicationid: *const ::windows_core::GUID) -> ::windows_core::Result<ISideShowNotificationManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RegisterNotifications)(::windows_core::Interface::as_raw(self), in_applicationid, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ISideShowSession, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISideShowSession {
    type Vtable = ISideShowSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISideShowSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe22331ee_9e7d_4922_9fc2_ab7aa41ce491);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowSession_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows_core::GUID, in_endpointid: *const ::windows_core::GUID, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows_core::GUID, out_ppinotification: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const CONTENT_ID_GLANCE: u32 = 0u32;
pub const CONTENT_ID_HOME: u32 = 1u32;
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x152e5811_feb9_4b00_90f4_d32947ae1681);
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = SCF_BUTTON_IDS(65280i32);
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = SCF_BUTTON_IDS(4i32);
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = SCF_BUTTON_IDS(9i32);
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(5i32);
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = SCF_BUTTON_IDS(1i32);
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = SCF_BUTTON_IDS(8i32);
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = SCF_BUTTON_IDS(7i32);
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = SCF_BUTTON_IDS(10i32);
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(6i32);
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = SCF_BUTTON_IDS(2i32);
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(11i32);
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = SCF_BUTTON_IDS(3i32);
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = SCF_EVENT_IDS(3i32);
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = SCF_EVENT_IDS(2i32);
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = SCF_EVENT_IDS(1i32);
pub const SIDESHOW_APPLICATION_EVENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cb572fa_1d3b_49b3_a17a_2e6bff052854);
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 16 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 15 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 5 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 6 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 9 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 7 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 1 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99);
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 4 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 2 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 3 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 14 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 8 };
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 10 };
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(2i32);
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(0i32);
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = SIDESHOW_COLOR_TYPE(1i32);
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5007fba8_d313_439f_bea2_a50201d3e9a8);
pub const SIDESHOW_ENDPOINT_ICAL: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4dff36b5_9dde_4f76_9a2a_96435047063d);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9a5353f_2d4b_47ce_93ee_759f3a7dda4f);
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57813854_2fc1_411c_a59f_f24927608804);
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(0i32);
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = SIDESHOW_SCREEN_TYPE(1i32);
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5009673c_3f7d_4c7e_9971_eaa2e91f1575);
pub const SideShowKeyCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfbbdbf8_18de_49b8_83dc_ebc727c62d94);
pub const SideShowNotification: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ce3e86f_d5cd_4525_a766_1abab1a752f5);
pub const SideShowPropVariantCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe640f415_539e_4923_96cd_5f093bc250cd);
pub const SideShowSession: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe20543b9_f785_4ea2_981e_c4ffa76bbc7c);
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCF_BUTTON_IDS(pub i32);
impl ::core::marker::Copy for SCF_BUTTON_IDS {}
impl ::core::clone::Clone for SCF_BUTTON_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCF_BUTTON_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SCF_BUTTON_IDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SCF_BUTTON_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCF_BUTTON_IDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SCF_EVENT_IDS(pub i32);
impl ::core::marker::Copy for SCF_EVENT_IDS {}
impl ::core::clone::Clone for SCF_EVENT_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SCF_EVENT_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SCF_EVENT_IDS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SCF_EVENT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCF_EVENT_IDS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SIDESHOW_COLOR_TYPE(pub i32);
impl ::core::marker::Copy for SIDESHOW_COLOR_TYPE {}
impl ::core::clone::Clone for SIDESHOW_COLOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SIDESHOW_COLOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SIDESHOW_COLOR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SIDESHOW_COLOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIDESHOW_COLOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SIDESHOW_SCREEN_TYPE(pub i32);
impl ::core::marker::Copy for SIDESHOW_SCREEN_TYPE {}
impl ::core::clone::Clone for SIDESHOW_SCREEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SIDESHOW_SCREEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SIDESHOW_SCREEN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SIDESHOW_SCREEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SIDESHOW_SCREEN_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: ::windows_core::GUID,
    pub EndpointId: ::windows_core::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl ::core::marker::Copy for APPLICATION_EVENT_DATA {}
impl ::core::clone::Clone for APPLICATION_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for APPLICATION_EVENT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: ::windows_core::GUID,
    pub EndpointId: ::windows_core::GUID,
    pub ContentId: u32,
}
impl ::core::marker::Copy for CONTENT_MISSING_EVENT_DATA {}
impl ::core::clone::Clone for CONTENT_MISSING_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for CONTENT_MISSING_EVENT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for CONTENT_MISSING_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
impl ::core::marker::Copy for DEVICE_USER_CHANGE_EVENT_DATA {}
impl ::core::clone::Clone for DEVICE_USER_CHANGE_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DEVICE_USER_CHANGE_EVENT_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DEVICE_USER_CHANGE_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: ::windows_core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl ::core::marker::Copy for EVENT_DATA_HEADER {}
impl ::core::clone::Clone for EVENT_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for EVENT_DATA_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for EVENT_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
impl ::core::marker::Copy for NEW_EVENT_DATA_AVAILABLE {}
impl ::core::clone::Clone for NEW_EVENT_DATA_AVAILABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NEW_EVENT_DATA_AVAILABLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NEW_EVENT_DATA_AVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
impl ::core::marker::Copy for SCF_CONTEXTMENU_EVENT {}
impl ::core::clone::Clone for SCF_CONTEXTMENU_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCF_CONTEXTMENU_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_CONTEXTMENU_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("PreviousItemId", &self.PreviousItemId).field("MenuPage", &self.MenuPage).field("MenuItemId", &self.MenuItemId).finish()
    }
}
impl ::windows_core::TypeKind for SCF_CONTEXTMENU_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCF_CONTEXTMENU_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.PreviousItemId == other.PreviousItemId && self.MenuPage == other.MenuPage && self.MenuItemId == other.MenuItemId
    }
}
impl ::core::cmp::Eq for SCF_CONTEXTMENU_EVENT {}
impl ::core::default::Default for SCF_CONTEXTMENU_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
impl ::core::marker::Copy for SCF_EVENT_HEADER {}
impl ::core::clone::Clone for SCF_EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCF_EVENT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_EVENT_HEADER").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).finish()
    }
}
impl ::windows_core::TypeKind for SCF_EVENT_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCF_EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage
    }
}
impl ::core::cmp::Eq for SCF_EVENT_HEADER {}
impl ::core::default::Default for SCF_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
impl ::core::marker::Copy for SCF_MENUACTION_EVENT {}
impl ::core::clone::Clone for SCF_MENUACTION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCF_MENUACTION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_MENUACTION_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("Button", &self.Button).field("ItemId", &self.ItemId).finish()
    }
}
impl ::windows_core::TypeKind for SCF_MENUACTION_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCF_MENUACTION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.Button == other.Button && self.ItemId == other.ItemId
    }
}
impl ::core::cmp::Eq for SCF_MENUACTION_EVENT {}
impl ::core::default::Default for SCF_MENUACTION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
impl ::core::marker::Copy for SCF_NAVIGATION_EVENT {}
impl ::core::clone::Clone for SCF_NAVIGATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCF_NAVIGATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCF_NAVIGATION_EVENT").field("PreviousPage", &self.PreviousPage).field("TargetPage", &self.TargetPage).field("Button", &self.Button).finish()
    }
}
impl ::windows_core::TypeKind for SCF_NAVIGATION_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SCF_NAVIGATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.PreviousPage == other.PreviousPage && self.TargetPage == other.TargetPage && self.Button == other.Button
    }
}
impl ::core::cmp::Eq for SCF_NAVIGATION_EVENT {}
impl ::core::default::Default for SCF_NAVIGATION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
