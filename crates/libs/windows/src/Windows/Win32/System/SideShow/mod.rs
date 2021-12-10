#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C, packed(1))]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: ::windows::core::GUID,
    pub EndpointId: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for APPLICATION_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPLICATION_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APPLICATION_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for APPLICATION_EVENT_DATA {}
impl ::core::default::Default for APPLICATION_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CONTENT_ID_GLANCE: u32 = 0u32;
pub const CONTENT_ID_HOME: u32 = 1u32;
#[repr(C, packed(1))]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: ::windows::core::GUID,
    pub EndpointId: ::windows::core::GUID,
    pub ContentId: u32,
}
impl ::core::marker::Copy for CONTENT_MISSING_EVENT_DATA {}
impl ::core::clone::Clone for CONTENT_MISSING_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTENT_MISSING_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTENT_MISSING_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTENT_MISSING_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTENT_MISSING_EVENT_DATA {}
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
unsafe impl ::windows::core::Abi for DEVICE_USER_CHANGE_EVENT_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DEVICE_USER_CHANGE_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVICE_USER_CHANGE_EVENT_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DEVICE_USER_CHANGE_EVENT_DATA {}
impl ::core::default::Default for DEVICE_USER_CHANGE_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: ::windows::core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl ::core::marker::Copy for EVENT_DATA_HEADER {}
impl ::core::clone::Clone for EVENT_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EVENT_DATA_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EVENT_DATA_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EVENT_DATA_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for EVENT_DATA_HEADER {}
impl ::core::default::Default for EVENT_DATA_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152e5811_feb9_4b00_90f4_d32947ae1681);
#[repr(transparent)]
pub struct ISideShowBulkCapabilities(::windows::core::IUnknown);
impl ISideShowBulkCapabilities {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_keycapability), ::core::mem::transmute(inout_pvalue)).ok()
    }
    pub unsafe fn GetCapabilities<'a, Param0: ::windows::core::IntoParam<'a, ISideShowKeyCollection>>(&self, in_keycollection: Param0, inout_pvalues: *mut ::core::option::Option<ISideShowPropVariantCollection>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), in_keycollection.into_param().abi(), ::core::mem::transmute(inout_pvalues)).ok()
    }
}
impl ::core::convert::From<ISideShowBulkCapabilities> for ISideShowCapabilities {
    fn from(value: ISideShowBulkCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowBulkCapabilities> for ISideShowCapabilities {
    fn from(value: &ISideShowBulkCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISideShowCapabilities> for ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ISideShowCapabilities> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISideShowCapabilities> for &ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ISideShowCapabilities> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISideShowBulkCapabilities> for ::windows::core::IUnknown {
    fn from(value: ISideShowBulkCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowBulkCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ISideShowBulkCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowBulkCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowBulkCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowBulkCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowBulkCapabilities {}
unsafe impl ::windows::core::Interface for ISideShowBulkCapabilities {
    type Vtable = ISideShowBulkCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a2b7fbc_3ad5_48bd_bbf1_0e6cfbd10807);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowBulkCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_keycollection: ::windows::core::RawPtr, inout_pvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowCapabilities(::windows::core::IUnknown);
impl ISideShowCapabilities {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_keycapability), ::core::mem::transmute(inout_pvalue)).ok()
    }
}
impl ::core::convert::From<ISideShowCapabilities> for ::windows::core::IUnknown {
    fn from(value: ISideShowCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ISideShowCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowCapabilities {}
unsafe impl ::windows::core::Interface for ISideShowCapabilities {
    type Vtable = ISideShowCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x535e1379_c09e_4a54_a511_597bab3a72b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[repr(transparent)]
pub struct ISideShowCapabilitiesCollection(::windows::core::IUnknown);
impl ISideShowCapabilitiesCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn GetAt(&self, in_dwindex: u32) -> ::windows::core::Result<ISideShowCapabilities> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_dwindex), ::core::mem::transmute(&mut result__)).from_abi::<ISideShowCapabilities>(result__)
    }
}
impl ::core::convert::From<ISideShowCapabilitiesCollection> for ::windows::core::IUnknown {
    fn from(value: ISideShowCapabilitiesCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowCapabilitiesCollection> for ::windows::core::IUnknown {
    fn from(value: &ISideShowCapabilitiesCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowCapabilitiesCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowCapabilitiesCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowCapabilitiesCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowCapabilitiesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowCapabilitiesCollection {}
unsafe impl ::windows::core::Interface for ISideShowCapabilitiesCollection {
    type Vtable = ISideShowCapabilitiesCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50305597_5e0d_4ff7_b3af_33d0d9bd52dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowCapabilitiesCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowContent(::windows::core::IUnknown);
impl ISideShowContent {
    pub unsafe fn GetContent<'a, Param0: ::windows::core::IntoParam<'a, ISideShowCapabilities>>(&self, in_picapabilities: Param0, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), in_picapabilities.into_param().abi(), ::core::mem::transmute(out_pdwsize), ::core::mem::transmute(out_ppbdata)).ok()
    }
    pub unsafe fn ContentId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DifferentiateContent(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ISideShowContent> for ::windows::core::IUnknown {
    fn from(value: ISideShowContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowContent> for ::windows::core::IUnknown {
    fn from(value: &ISideShowContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowContent {}
unsafe impl ::windows::core::Interface for ISideShowContent {
    type Vtable = ISideShowContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18552ed_74ff_4fec_be07_4cfed29d4887);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ISideShowContentManager(::windows::core::IUnknown);
impl ISideShowContentManager {
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, ISideShowContent>>(&self, in_picontent: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), in_picontent.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, in_contentid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_contentid)).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetEventSink<'a, Param0: ::windows::core::IntoParam<'a, ISideShowEvents>>(&self, in_pievents: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), in_pievents.into_param().abi()).ok()
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<ISideShowCapabilitiesCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISideShowCapabilitiesCollection>(result__)
    }
}
impl ::core::convert::From<ISideShowContentManager> for ::windows::core::IUnknown {
    fn from(value: ISideShowContentManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowContentManager> for ::windows::core::IUnknown {
    fn from(value: &ISideShowContentManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowContentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowContentManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowContentManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowContentManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowContentManager {}
unsafe impl ::windows::core::Interface for ISideShowContentManager {
    type Vtable = ISideShowContentManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5d5b66b_eef9_41db_8d7e_e17c33ab10b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowContentManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pievents: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowEvents(::windows::core::IUnknown);
impl ISideShowEvents {
    pub unsafe fn ContentMissing(&self, in_contentid: u32) -> ::windows::core::Result<ISideShowContent> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_contentid), ::core::mem::transmute(&mut result__)).from_abi::<ISideShowContent>(result__)
    }
    pub unsafe fn ApplicationEvent<'a, Param0: ::windows::core::IntoParam<'a, ISideShowCapabilities>>(&self, in_picapabilities: Param0, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), in_picapabilities.into_param().abi(), ::core::mem::transmute(in_dweventid), ::core::mem::transmute(in_dweventsize), ::core::mem::transmute(in_pbeventdata)).ok()
    }
    pub unsafe fn DeviceAdded<'a, Param0: ::windows::core::IntoParam<'a, ISideShowCapabilities>>(&self, in_pidevice: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), in_pidevice.into_param().abi()).ok()
    }
    pub unsafe fn DeviceRemoved<'a, Param0: ::windows::core::IntoParam<'a, ISideShowCapabilities>>(&self, in_pidevice: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), in_pidevice.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISideShowEvents> for ::windows::core::IUnknown {
    fn from(value: ISideShowEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowEvents> for ::windows::core::IUnknown {
    fn from(value: &ISideShowEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowEvents {}
unsafe impl ::windows::core::Interface for ISideShowEvents {
    type Vtable = ISideShowEventsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61feca4c_deb4_4a7e_8d75_51f1132d615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowEventsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowKeyCollection(::windows::core::IUnknown);
impl ISideShowKeyCollection {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey)).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<ISideShowKeyCollection> for ::windows::core::IUnknown {
    fn from(value: ISideShowKeyCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowKeyCollection> for ::windows::core::IUnknown {
    fn from(value: &ISideShowKeyCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowKeyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowKeyCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowKeyCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowKeyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowKeyCollection {}
unsafe impl ::windows::core::Interface for ISideShowKeyCollection {
    type Vtable = ISideShowKeyCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x045473bc_a37b_4957_b144_68105411ed8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowKeyCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowNotification(::windows::core::IUnknown);
impl ISideShowNotification {
    pub unsafe fn NotificationId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    pub unsafe fn SetNotificationId(&self, in_notificationid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_notificationid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, in_pwsztitle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), in_pwsztitle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, in_pwszmessage: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), in_pwszmessage.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Image(&self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__: super::super::UI::WindowsAndMessaging::HICON = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::WindowsAndMessaging::HICON>>(&self, in_hicon: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), in_hicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__: super::super::Foundation::SYSTEMTIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationTime(&self, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_ptime)).ok()
    }
}
impl ::core::convert::From<ISideShowNotification> for ::windows::core::IUnknown {
    fn from(value: ISideShowNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowNotification> for ::windows::core::IUnknown {
    fn from(value: &ISideShowNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowNotification {}
unsafe impl ::windows::core::Interface for ISideShowNotification {
    type Vtable = ISideShowNotificationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03c93300_8ab2_41c5_9b79_46127a30e148);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotificationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pwszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
pub struct ISideShowNotificationManager(::windows::core::IUnknown);
impl ISideShowNotificationManager {
    pub unsafe fn Show<'a, Param0: ::windows::core::IntoParam<'a, ISideShowNotification>>(&self, in_pinotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), in_pinotification.into_param().abi()).ok()
    }
    pub unsafe fn Revoke(&self, in_notificationid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_notificationid)).ok()
    }
    pub unsafe fn RevokeAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISideShowNotificationManager> for ::windows::core::IUnknown {
    fn from(value: ISideShowNotificationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowNotificationManager> for ::windows::core::IUnknown {
    fn from(value: &ISideShowNotificationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowNotificationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowNotificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowNotificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowNotificationManager {}
unsafe impl ::windows::core::Interface for ISideShowNotificationManager {
    type Vtable = ISideShowNotificationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63cea909_f2b9_4302_b5e1_c68e6d9ab833);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowNotificationManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_pinotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowPropVariantCollection(::windows::core::IUnknown);
impl ISideShowPropVariantCollection {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Add(&self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
impl ::core::convert::From<ISideShowPropVariantCollection> for ::windows::core::IUnknown {
    fn from(value: ISideShowPropVariantCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowPropVariantCollection> for ::windows::core::IUnknown {
    fn from(value: &ISideShowPropVariantCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowPropVariantCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowPropVariantCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowPropVariantCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowPropVariantCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowPropVariantCollection {}
unsafe impl ::windows::core::Interface for ISideShowPropVariantCollection {
    type Vtable = ISideShowPropVariantCollectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ea7a549_7bff_4aae_bab0_22d43111de49);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowPropVariantCollectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct ISideShowSession(::windows::core::IUnknown);
impl ISideShowSession {
    pub unsafe fn RegisterContent(&self, in_applicationid: *const ::windows::core::GUID, in_endpointid: *const ::windows::core::GUID) -> ::windows::core::Result<ISideShowContentManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_applicationid), ::core::mem::transmute(in_endpointid), ::core::mem::transmute(&mut result__)).from_abi::<ISideShowContentManager>(result__)
    }
    pub unsafe fn RegisterNotifications(&self, in_applicationid: *const ::windows::core::GUID) -> ::windows::core::Result<ISideShowNotificationManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(in_applicationid), ::core::mem::transmute(&mut result__)).from_abi::<ISideShowNotificationManager>(result__)
    }
}
impl ::core::convert::From<ISideShowSession> for ::windows::core::IUnknown {
    fn from(value: ISideShowSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISideShowSession> for ::windows::core::IUnknown {
    fn from(value: &ISideShowSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISideShowSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISideShowSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISideShowSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISideShowSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISideShowSession {}
unsafe impl ::windows::core::Interface for ISideShowSession {
    type Vtable = ISideShowSessionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe22331ee_9e7d_4922_9fc2_ab7aa41ce491);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISideShowSessionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, in_endpointid: *const ::windows::core::GUID, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, out_ppinotification: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
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
unsafe impl ::windows::core::Abi for NEW_EVENT_DATA_AVAILABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NEW_EVENT_DATA_AVAILABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NEW_EVENT_DATA_AVAILABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NEW_EVENT_DATA_AVAILABLE {}
impl ::core::default::Default for NEW_EVENT_DATA_AVAILABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type SCF_BUTTON_IDS = i32;
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = 1i32;
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = 2i32;
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = 3i32;
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = 4i32;
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = 5i32;
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = 6i32;
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = 7i32;
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = 8i32;
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = 9i32;
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = 10i32;
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = 11i32;
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = 65280i32;
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
unsafe impl ::windows::core::Abi for SCF_CONTEXTMENU_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCF_CONTEXTMENU_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCF_CONTEXTMENU_EVENT>()) == 0 }
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
unsafe impl ::windows::core::Abi for SCF_EVENT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCF_EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCF_EVENT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCF_EVENT_HEADER {}
impl ::core::default::Default for SCF_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type SCF_EVENT_IDS = i32;
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = 1i32;
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = 2i32;
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = 3i32;
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
unsafe impl ::windows::core::Abi for SCF_MENUACTION_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCF_MENUACTION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCF_MENUACTION_EVENT>()) == 0 }
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
unsafe impl ::windows::core::Abi for SCF_NAVIGATION_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCF_NAVIGATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCF_NAVIGATION_EVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCF_NAVIGATION_EVENT {}
impl ::core::default::Default for SCF_NAVIGATION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SIDESHOW_APPLICATION_EVENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cb572fa_1d3b_49b3_a17a_2e6bff052854);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 1u32 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 10u32 };
pub type SIDESHOW_COLOR_TYPE = i32;
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = 0i32;
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = 1i32;
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = 2i32;
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5007fba8_d313_439f_bea2_a50201d3e9a8);
pub const SIDESHOW_ENDPOINT_ICAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dff36b5_9dde_4f76_9a2a_96435047063d);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9a5353f_2d4b_47ce_93ee_759f3a7dda4f);
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57813854_2fc1_411c_a59f_f24927608804);
pub type SIDESHOW_SCREEN_TYPE = i32;
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = 0i32;
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = 1i32;
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5009673c_3f7d_4c7e_9971_eaa2e91f1575);
pub const SideShowKeyCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfbbdbf8_18de_49b8_83dc_ebc727c62d94);
pub const SideShowNotification: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce3e86f_d5cd_4525_a766_1abab1a752f5);
pub const SideShowPropVariantCollection: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe640f415_539e_4923_96cd_5f093bc250cd);
pub const SideShowSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe20543b9_f785_4ea2_981e_c4ffa76bbc7c);
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
