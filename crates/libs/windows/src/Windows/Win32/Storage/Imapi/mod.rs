pub const BlockRange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
pub const CATID_SMTP_DNSRESOLVERRECORDSINK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd0b4366_8e03_11d2_94f6_00c04f79f1d6);
pub const CATID_SMTP_DSN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22b55731_f5f8_4d23_bd8f_87b52371a73a);
pub const CATID_SMTP_GET_AUX_DOMAIN_INFO_FLAGS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84ff368a_fab3_43d7_bcdf_692c5b46e6b1);
pub const CATID_SMTP_LOG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93d0a538_2c1e_4b68_a7c9_d73a8aa6ee97);
pub const CATID_SMTP_MAXMSGSIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebf159de_a67e_11d2_94f7_00c04f79f1d6);
pub const CATID_SMTP_MSGTRACKLOG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6df52aa_7db0_11d2_94f4_00c04f79f1d6);
pub const CATID_SMTP_ON_BEFORE_DATA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c92_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_INBOUND_COMMAND: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c8d_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_MESSAGE_START: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c90_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_PER_RECIPIENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c91_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SERVER_RESPONSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c8e_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_END: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c93_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_ON_SESSION_START: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6628c8f_0d5e_11d2_aa68_00c04fa35b82);
pub const CATID_SMTP_STORE_DRIVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59175850_e533_11d1_aa67_00c04fa345f6);
pub const CATID_SMTP_TRANSPORT_CATEGORIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x960252a3_0a3a_11d2_9e00_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_POSTCATEGORIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76719654_05a6_11d2_9dfd_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_PRECATEGORIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3acfb0d_83ff_11d2_9e14_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_ROUTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x283430c9_1850_11d2_9e03_00c04fa322ba);
pub const CATID_SMTP_TRANSPORT_SUBMISSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff3caa23_00b9_11d2_9dfb_00c04fa322ba);
pub const CLSID_SmtpCat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23c35b7_9219_11d2_9e17_00c04fa322ba);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[inline]
pub unsafe fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseIMsgSession(lpmsgsess: *mut _MSGSESS);
    }
    CloseIMsgSession(::core::mem::transmute(lpmsgsess))
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DDiscFormat2DataEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2DataEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<'a, P0, P1>(&self, object: P0, progress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), progress.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2DataEvents> for ::windows::core::IUnknown {
    fn from(value: DDiscFormat2DataEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2DataEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DDiscFormat2DataEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2DataEvents> for ::windows::core::IUnknown {
    fn from(value: &DDiscFormat2DataEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2DataEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2DataEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2DataEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DDiscFormat2DataEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2DataEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2DataEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DDiscFormat2DataEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DDiscFormat2DataEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DDiscFormat2DataEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DDiscFormat2DataEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDiscFormat2DataEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DDiscFormat2DataEvents {
    type Vtable = DDiscFormat2DataEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DDiscFormat2EraseEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2EraseEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<'a, P0>(&self, object: P0, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), elapsedseconds, estimatedtotalseconds).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2EraseEvents> for ::windows::core::IUnknown {
    fn from(value: DDiscFormat2EraseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2EraseEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DDiscFormat2EraseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2EraseEvents> for ::windows::core::IUnknown {
    fn from(value: &DDiscFormat2EraseEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2EraseEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2EraseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2EraseEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DDiscFormat2EraseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2EraseEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2EraseEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DDiscFormat2EraseEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DDiscFormat2EraseEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DDiscFormat2EraseEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DDiscFormat2EraseEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDiscFormat2EraseEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DDiscFormat2EraseEvents {
    type Vtable = DDiscFormat2EraseEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DDiscFormat2RawCDEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2RawCDEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<'a, P0, P1>(&self, object: P0, progress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), progress.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2RawCDEvents> for ::windows::core::IUnknown {
    fn from(value: DDiscFormat2RawCDEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2RawCDEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DDiscFormat2RawCDEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2RawCDEvents> for ::windows::core::IUnknown {
    fn from(value: &DDiscFormat2RawCDEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2RawCDEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2RawCDEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2RawCDEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DDiscFormat2RawCDEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2RawCDEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2RawCDEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DDiscFormat2RawCDEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DDiscFormat2RawCDEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DDiscFormat2RawCDEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DDiscFormat2RawCDEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDiscFormat2RawCDEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DDiscFormat2RawCDEvents {
    type Vtable = DDiscFormat2RawCDEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354142_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DDiscFormat2TrackAtOnceEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2TrackAtOnceEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<'a, P0, P1>(&self, object: P0, progress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), progress.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2TrackAtOnceEvents> for ::windows::core::IUnknown {
    fn from(value: DDiscFormat2TrackAtOnceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2TrackAtOnceEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DDiscFormat2TrackAtOnceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2TrackAtOnceEvents> for ::windows::core::IUnknown {
    fn from(value: &DDiscFormat2TrackAtOnceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscFormat2TrackAtOnceEvents> for super::super::System::Com::IDispatch {
    fn from(value: DDiscFormat2TrackAtOnceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscFormat2TrackAtOnceEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DDiscFormat2TrackAtOnceEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscFormat2TrackAtOnceEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscFormat2TrackAtOnceEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DDiscFormat2TrackAtOnceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DDiscFormat2TrackAtOnceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DDiscFormat2TrackAtOnceEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DDiscFormat2TrackAtOnceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDiscFormat2TrackAtOnceEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DDiscFormat2TrackAtOnceEvents {
    type Vtable = DDiscFormat2TrackAtOnceEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DDiscMaster2Events(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DDiscMaster2Events {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn NotifyDeviceAdded<'a, P0, P1>(&self, object: P0, uniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).NotifyDeviceAdded)(::windows::core::Interface::as_raw(self), object.into().abi(), uniqueid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn NotifyDeviceRemoved<'a, P0, P1>(&self, object: P0, uniqueid: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).NotifyDeviceRemoved)(::windows::core::Interface::as_raw(self), object.into().abi(), uniqueid.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscMaster2Events> for ::windows::core::IUnknown {
    fn from(value: DDiscMaster2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscMaster2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DDiscMaster2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscMaster2Events> for ::windows::core::IUnknown {
    fn from(value: &DDiscMaster2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DDiscMaster2Events> for super::super::System::Com::IDispatch {
    fn from(value: DDiscMaster2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DDiscMaster2Events> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DDiscMaster2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DDiscMaster2Events> for super::super::System::Com::IDispatch {
    fn from(value: &DDiscMaster2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DDiscMaster2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DDiscMaster2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DDiscMaster2Events {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DDiscMaster2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDiscMaster2Events").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DDiscMaster2Events {
    type Vtable = DDiscMaster2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354131_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyDeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyDeviceAdded: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub NotifyDeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    NotifyDeviceRemoved: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DFileSystemImageEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Update<'a, P0, P1>(&self, object: P0, currentfile: P1, copiedsectors: i32, totalsectors: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), currentfile.into().abi(), copiedsectors, totalsectors).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageEvents> for ::windows::core::IUnknown {
    fn from(value: DFileSystemImageEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DFileSystemImageEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DFileSystemImageEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageEvents> for ::windows::core::IUnknown {
    fn from(value: &DFileSystemImageEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageEvents> for super::super::System::Com::IDispatch {
    fn from(value: DFileSystemImageEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DFileSystemImageEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DFileSystemImageEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DFileSystemImageEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DFileSystemImageEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DFileSystemImageEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DFileSystemImageEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DFileSystemImageEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFileSystemImageEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DFileSystemImageEvents {
    type Vtable = DFileSystemImageEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fdf_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DFileSystemImageImportEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageImportEvents {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateImport<'a, P0, P1>(&self, object: P0, filesystem: FsiFileSystems, currentitem: P1, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).UpdateImport)(::windows::core::Interface::as_raw(self), object.into().abi(), filesystem, currentitem.into().abi(), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageImportEvents> for ::windows::core::IUnknown {
    fn from(value: DFileSystemImageImportEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DFileSystemImageImportEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DFileSystemImageImportEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageImportEvents> for ::windows::core::IUnknown {
    fn from(value: &DFileSystemImageImportEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DFileSystemImageImportEvents> for super::super::System::Com::IDispatch {
    fn from(value: DFileSystemImageImportEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DFileSystemImageImportEvents> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DFileSystemImageImportEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DFileSystemImageImportEvents> for super::super::System::Com::IDispatch {
    fn from(value: &DFileSystemImageImportEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DFileSystemImageImportEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DFileSystemImageImportEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DFileSystemImageImportEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DFileSystemImageImportEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFileSystemImageImportEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DFileSystemImageImportEvents {
    type Vtable = DFileSystemImageImportEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd25c30f9_4087_4366_9e24_e55be286424b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdateImport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdateImport: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISC_RECORDER_STATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_BURNING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_DOING_NOTHING: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_OPENED: DISC_RECORDER_STATE_FLAGS = DISC_RECORDER_STATE_FLAGS(1u32);
impl ::core::marker::Copy for DISC_RECORDER_STATE_FLAGS {}
impl ::core::clone::Clone for DISC_RECORDER_STATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISC_RECORDER_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISC_RECORDER_STATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISC_RECORDER_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISC_RECORDER_STATE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_INUSE: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DWriteEngine2Events(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DWriteEngine2Events {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<'a, P0, P1>(&self, object: P0, progress: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IDispatch>>,
    {
        (::windows::core::Interface::vtable(self).Update)(::windows::core::Interface::as_raw(self), object.into().abi(), progress.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DWriteEngine2Events> for ::windows::core::IUnknown {
    fn from(value: DWriteEngine2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DWriteEngine2Events> for &'a ::windows::core::IUnknown {
    fn from(value: &'a DWriteEngine2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DWriteEngine2Events> for ::windows::core::IUnknown {
    fn from(value: &DWriteEngine2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<DWriteEngine2Events> for super::super::System::Com::IDispatch {
    fn from(value: DWriteEngine2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a DWriteEngine2Events> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a DWriteEngine2Events) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&DWriteEngine2Events> for super::super::System::Com::IDispatch {
    fn from(value: &DWriteEngine2Events) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DWriteEngine2Events {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DWriteEngine2Events {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DWriteEngine2Events {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DWriteEngine2Events {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWriteEngine2Events").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DWriteEngine2Events {
    type Vtable = DWriteEngine2Events_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354137_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmulationType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const EmulationNone: EmulationType = EmulationType(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation12MFloppy: EmulationType = EmulationType(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation144MFloppy: EmulationType = EmulationType(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const Emulation288MFloppy: EmulationType = EmulationType(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const EmulationHardDisk: EmulationType = EmulationType(4i32);
impl ::core::marker::Copy for EmulationType {}
impl ::core::clone::Clone for EmulationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmulationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EmulationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EmulationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmulationType").field(&self.0).finish()
    }
}
pub const EnumFsiItems: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fc6_975b_59be_a960_9a2a262853a5);
pub const EnumProgressItems: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fca_975b_59be_a960_9a2a262853a5);
pub const FileSystemImageResult: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fcc_975b_59be_a960_9a2a262853a5);
pub const FsiDirectoryItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fc8_975b_59be_a960_9a2a262853a5);
pub const FsiFileItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fc7_975b_59be_a960_9a2a262853a5);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsiFileSystems(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemNone: FsiFileSystems = FsiFileSystems(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemISO9660: FsiFileSystems = FsiFileSystems(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemJoliet: FsiFileSystems = FsiFileSystems(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemUDF: FsiFileSystems = FsiFileSystems(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiFileSystemUnknown: FsiFileSystems = FsiFileSystems(1073741824i32);
impl ::core::marker::Copy for FsiFileSystems {}
impl ::core::clone::Clone for FsiFileSystems {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsiFileSystems {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsiFileSystems {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsiFileSystems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsiFileSystems").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsiItemType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemNotFound: FsiItemType = FsiItemType(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemDirectory: FsiItemType = FsiItemType(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const FsiItemFile: FsiItemType = FsiItemType(2i32);
impl ::core::marker::Copy for FsiItemType {}
impl ::core::clone::Clone for FsiItemType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsiItemType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsiItemType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsiItemType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsiItemType").field(&self.0).finish()
    }
}
pub const FsiNamedStreams: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6b6f8ed_6d19_44b4_b539_b159b793a32d);
pub const FsiStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fcd_975b_59be_a960_9a2a262853a5);
pub const GUID_SMTPSVC_SOURCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b3c0666_e470_11d1_aa67_00c04fa345f6);
pub const GUID_SMTP_SOURCE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb65c4dc_e468_11d1_aa67_00c04fa345f6);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptagarray: *mut super::super::System::AddressBook::SPropTagArray, lpppropattrarray: *mut *mut SPropAttrArray) -> ::windows::core::HRESULT;
    }
    GetAttribIMsgOnIStg(::core::mem::transmute(lpobject), ::core::mem::transmute(lpproptagarray), ::core::mem::transmute(lpppropattrarray)).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBlockRange(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRange {
    pub unsafe fn StartLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EndLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EndLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRange> for ::windows::core::IUnknown {
    fn from(value: IBlockRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBlockRange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBlockRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRange> for ::windows::core::IUnknown {
    fn from(value: &IBlockRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRange> for super::super::System::Com::IDispatch {
    fn from(value: IBlockRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBlockRange> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IBlockRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRange> for super::super::System::Com::IDispatch {
    fn from(value: &IBlockRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IBlockRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBlockRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBlockRange {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBlockRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockRange").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBlockRange {
    type Vtable = IBlockRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca25_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub EndLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBlockRangeList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeList {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockRanges(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BlockRanges)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRangeList> for ::windows::core::IUnknown {
    fn from(value: IBlockRangeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBlockRangeList> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBlockRangeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRangeList> for ::windows::core::IUnknown {
    fn from(value: &IBlockRangeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBlockRangeList> for super::super::System::Com::IDispatch {
    fn from(value: IBlockRangeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBlockRangeList> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IBlockRangeList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBlockRangeList> for super::super::System::Com::IDispatch {
    fn from(value: &IBlockRangeList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IBlockRangeList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBlockRangeList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBlockRangeList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBlockRangeList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockRangeList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBlockRangeList {
    type Vtable = IBlockRangeList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca26_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockRanges: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBootOptions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBootOptions {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImage(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BootImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Manufacturer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Manufacturer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManufacturer<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetManufacturer)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn PlatformId(&self) -> ::windows::core::Result<PlatformId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PlatformId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PlatformId>(result__)
    }
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPlatformId)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Emulation(&self) -> ::windows::core::Result<EmulationType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Emulation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<EmulationType>(result__)
    }
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEmulation)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ImageSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ImageSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AssignBootImage<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AssignBootImage)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootOptions> for ::windows::core::IUnknown {
    fn from(value: IBootOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBootOptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBootOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootOptions> for ::windows::core::IUnknown {
    fn from(value: &IBootOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootOptions> for super::super::System::Com::IDispatch {
    fn from(value: IBootOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBootOptions> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IBootOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootOptions> for super::super::System::Com::IDispatch {
    fn from(value: &IBootOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IBootOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBootOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBootOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBootOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBootOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBootOptions {
    type Vtable = IBootOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd4_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Manufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetManufacturer: usize,
    pub PlatformId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows::core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows::core::HRESULT,
    pub Emulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows::core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows::core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AssignBootImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AssignBootImage: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IBurnVerification(::windows::core::IUnknown);
impl IBurnVerification {
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBurnVerificationLevel)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BurnVerificationLevel(&self) -> ::windows::core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BurnVerificationLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_BURN_VERIFICATION_LEVEL>(result__)
    }
}
impl ::core::convert::From<IBurnVerification> for ::windows::core::IUnknown {
    fn from(value: IBurnVerification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBurnVerification> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBurnVerification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBurnVerification> for ::windows::core::IUnknown {
    fn from(value: &IBurnVerification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IBurnVerification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBurnVerification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBurnVerification {}
impl ::core::fmt::Debug for IBurnVerification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBurnVerification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBurnVerification {
    type Vtable = IBurnVerification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ffd834_958b_426d_8470_2a13879c6a91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsRecorderSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsCurrentMediaSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MediaPhysicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MediaHeuristicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedMediaTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2 {
    type Vtable = IDiscFormat2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354152_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub IsRecorderSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsRecorderSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IsCurrentMediaSupported: usize,
    pub MediaPhysicallyBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedMediaTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedMediaTypes: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2Data(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Data {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsRecorderSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorder)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Recorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetPostgapAlreadyInImage(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPostgapAlreadyInImage)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn PostgapAlreadyInImage(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PostgapAlreadyInImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentMediaStatus(&self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentMediaStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_DATA_MEDIA_STATE>(result__)
    }
    pub unsafe fn WriteProtectStatus(&self) -> ::windows::core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteProtectStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_WRITE_PROTECT_STATE>(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FreeSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextWritableAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetForceMediaToBeClosed(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForceMediaToBeClosed)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ForceMediaToBeClosed(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ForceMediaToBeClosed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisableConsumerDvdCompatibilityMode)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DisableConsumerDvdCompatibilityMode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetClientName)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ClientName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeeds)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn SetForceOverwrite(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForceOverwrite)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ForceOverwrite(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ForceOverwrite)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWriteSpeed)(::windows::core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Data> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Data> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Data> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2Data) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Data> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Data> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Data> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2Data) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Data> for IDiscFormat2 {
    fn from(value: IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Data> for &'a IDiscFormat2 {
    fn from(value: &'a IDiscFormat2Data) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Data> for IDiscFormat2 {
    fn from(value: &IDiscFormat2Data) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2Data {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2Data {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2Data {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2Data").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2Data {
    type Vtable = IDiscFormat2Data_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354153_9f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub PostgapAlreadyInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentMediaStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub ForceMediaToBeClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
    pub SetForceOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub ForceOverwrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2DataEventArgs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StartLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SectorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastReadLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastWrittenLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ElapsedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RemainingTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentAction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_DATA_WRITE_ACTION>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2DataEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2DataEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2DataEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2DataEventArgs> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2DataEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2DataEventArgs> for &'a IWriteEngine2EventArgs {
    fn from(value: &'a IDiscFormat2DataEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2DataEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2DataEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2DataEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2DataEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2DataEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2DataEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2DataEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2DataEventArgs {
    type Vtable = IDiscFormat2DataEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735413d_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2Erase(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Erase {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsRecorderSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorder)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Recorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn SetFullErase(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFullErase)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn FullErase(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FullErase)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetClientName)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ClientName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn EraseMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EraseMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Erase> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Erase> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Erase> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2Erase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Erase> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Erase> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Erase> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2Erase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2Erase> for IDiscFormat2 {
    fn from(value: IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2Erase> for &'a IDiscFormat2 {
    fn from(value: &'a IDiscFormat2Erase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2Erase> for IDiscFormat2 {
    fn from(value: &IDiscFormat2Erase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2Erase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2Erase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2Erase {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2Erase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2Erase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2Erase {
    type Vtable = IDiscFormat2Erase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354156_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Erase_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetFullErase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub FullErase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub EraseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2RawCD(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCD {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsRecorderSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PrepareMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).WriteMedia)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteMedia2<'a, P0>(&self, data: P0, streamleadinsectors: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).WriteMedia2)(::windows::core::Interface::as_raw(self), data.into().abi(), streamleadinsectors).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWriteSpeed)(::windows::core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorder)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Recorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn StartOfNextSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartOfNextSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastPossibleStartOfLeadout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedSectorTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedSectorTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRequestedSectorType)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn RequestedSectorType(&self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedSectorType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetClientName)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ClientName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeeds)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCD> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCD> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCD> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCD> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCD> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCD> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCD> for IDiscFormat2 {
    fn from(value: IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCD> for &'a IDiscFormat2 {
    fn from(value: &'a IDiscFormat2RawCD) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCD> for IDiscFormat2 {
    fn from(value: &IDiscFormat2RawCD) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2RawCD {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2RawCD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2RawCD {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2RawCD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2RawCD").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2RawCD {
    type Vtable = IDiscFormat2RawCD_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354155_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCD_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteMedia2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, streamleadinsectors: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub StartOfNextSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedSectorTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedSectorTypes: usize,
    pub SetRequestedSectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2RawCDEventArgs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StartLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SectorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastReadLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastWrittenLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentAction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ElapsedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RemainingTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCDEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCDEventArgs> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2RawCDEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2RawCDEventArgs> for &'a IWriteEngine2EventArgs {
    fn from(value: &'a IDiscFormat2RawCDEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2RawCDEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2RawCDEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2RawCDEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2RawCDEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2RawCDEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2RawCDEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2RawCDEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2RawCDEventArgs {
    type Vtable = IDiscFormat2RawCDEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354143_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnce(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnce {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsRecorderSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsRecorderSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsCurrentMediaSupported<'a, P0>(&self, recorder: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsCurrentMediaSupported)(::windows::core::Interface::as_raw(self), recorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaPhysicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaPhysicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MediaHeuristicallyBlank(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MediaHeuristicallyBlank)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedMediaTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SupportedMediaTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PrepareMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAudioTrack<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AddAudioTrack)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    pub unsafe fn CancelAddTrack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelAddTrack)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWriteSpeed)(::windows::core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRecorder<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorder)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Recorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BufferUnderrunFreeDisabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NumberOfExistingTracks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FreeSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UsedSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UsedSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDoNotFinalizeMedia(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDoNotFinalizeMedia)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn DoNotFinalizeMedia(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DoNotFinalizeMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ExpectedTableOfContents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentPhysicalMediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetClientName)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ClientName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentWriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeeds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeeds)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnce> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnce> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnce> for IDiscFormat2 {
    fn from(value: IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnce> for &'a IDiscFormat2 {
    fn from(value: &'a IDiscFormat2TrackAtOnce) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnce> for IDiscFormat2 {
    fn from(value: &IDiscFormat2TrackAtOnce) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2TrackAtOnce {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2TrackAtOnce {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2TrackAtOnce {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2TrackAtOnce {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2TrackAtOnce").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2TrackAtOnce {
    type Vtable = IDiscFormat2TrackAtOnce_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354154_8f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnce_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAudioTrack: usize,
    pub CancelAddTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRecorder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recorder: usize,
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub DoNotFinalizeMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientName: usize,
    pub RequestedWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub CurrentWriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeeds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeeds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedWriteSpeedDescriptors: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscFormat2TrackAtOnceEventArgs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceEventArgs {
    pub unsafe fn StartLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StartLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SectorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastReadLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastWrittenLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TotalSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UsedSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FreeSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentTrackNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentTrackNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CurrentAction(&self) -> ::windows::core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentAction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_TAO_WRITE_ACTION>(result__)
    }
    pub unsafe fn ElapsedTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ElapsedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RemainingTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RemainingTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnceEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnceEventArgs> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscFormat2TrackAtOnceEventArgs> for IWriteEngine2EventArgs {
    fn from(value: IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscFormat2TrackAtOnceEventArgs> for &'a IWriteEngine2EventArgs {
    fn from(value: &'a IDiscFormat2TrackAtOnceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscFormat2TrackAtOnceEventArgs> for IWriteEngine2EventArgs {
    fn from(value: &IDiscFormat2TrackAtOnceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscFormat2TrackAtOnceEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscFormat2TrackAtOnceEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscFormat2TrackAtOnceEventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscFormat2TrackAtOnceEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscFormat2TrackAtOnceEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscFormat2TrackAtOnceEventArgs {
    type Vtable = IDiscFormat2TrackAtOnceEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354140_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IDiscMaster(::windows::core::IUnknown);
impl IDiscMaster {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDiscMasterFormats(&self) -> ::windows::core::Result<IEnumDiscMasterFormats> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumDiscMasterFormats)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDiscMasterFormats>(result__)
    }
    pub unsafe fn GetActiveDiscMasterFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetActiveDiscMasterFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetActiveDiscMasterFormat(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActiveDiscMasterFormat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn EnumDiscRecorders(&self) -> ::windows::core::Result<IEnumDiscRecorders> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumDiscRecorders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDiscRecorders>(result__)
    }
    pub unsafe fn GetActiveDiscRecorder(&self) -> ::windows::core::Result<IDiscRecorder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetActiveDiscRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder>(result__)
    }
    pub unsafe fn SetActiveDiscRecorder<'a, P0>(&self, precorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder>>,
    {
        (::windows::core::Interface::vtable(self).SetActiveDiscRecorder)(::windows::core::Interface::as_raw(self), precorder.into().abi()).ok()
    }
    pub unsafe fn ClearFormatContent(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFormatContent)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ProgressAdvise<'a, P0>(&self, pevents: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscMasterProgressEvents>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProgressAdvise)(::windows::core::Interface::as_raw(self), pevents.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn ProgressUnadvise(&self, vcookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProgressUnadvise)(::windows::core::Interface::as_raw(self), vcookie).ok()
    }
    pub unsafe fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RecordDisc)(::windows::core::Interface::as_raw(self), bsimulate, bejectafterburn).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDiscMaster> for ::windows::core::IUnknown {
    fn from(value: IDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDiscMaster> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscMaster> for ::windows::core::IUnknown {
    fn from(value: &IDiscMaster) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDiscMaster {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDiscMaster {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiscMaster {}
impl ::core::fmt::Debug for IDiscMaster {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscMaster").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiscMaster {
    type Vtable = IDiscMaster_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520cca62_51a5_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDiscMasterFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetActiveDiscMasterFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDiscRecorders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecorder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precorder: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearFormatContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProgressAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, pvcookie: *mut usize) -> ::windows::core::HRESULT,
    pub ProgressUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows::core::HRESULT,
    pub RecordDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscMaster2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscMaster2 {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn IsSupportedEnvironment(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSupportedEnvironment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscMaster2> for ::windows::core::IUnknown {
    fn from(value: IDiscMaster2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscMaster2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscMaster2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscMaster2> for ::windows::core::IUnknown {
    fn from(value: &IDiscMaster2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscMaster2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscMaster2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscMaster2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscMaster2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscMaster2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscMaster2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscMaster2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscMaster2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscMaster2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscMaster2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscMaster2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscMaster2 {
    type Vtable = IDiscMaster2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354130_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub IsSupportedEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IDiscMasterProgressEvents(::windows::core::IUnknown);
impl IDiscMasterProgressEvents {
    pub unsafe fn QueryCancel(&self) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryCancel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u8>(result__)
    }
    pub unsafe fn NotifyPnPActivity(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyPnPActivity)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyAddProgress)(::windows::core::Interface::as_raw(self), ncompletedsteps, ntotalsteps).ok()
    }
    pub unsafe fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyBlockProgress)(::windows::core::Interface::as_raw(self), ncompleted, ntotal).ok()
    }
    pub unsafe fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyTrackProgress)(::windows::core::Interface::as_raw(self), ncurrenttrack, ntotaltracks).ok()
    }
    pub unsafe fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyPreparingBurn)(::windows::core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyClosingDisc)(::windows::core::Interface::as_raw(self), nestimatedseconds).ok()
    }
    pub unsafe fn NotifyBurnComplete(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyBurnComplete)(::windows::core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn NotifyEraseComplete(&self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyEraseComplete)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
impl ::core::convert::From<IDiscMasterProgressEvents> for ::windows::core::IUnknown {
    fn from(value: IDiscMasterProgressEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDiscMasterProgressEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscMasterProgressEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscMasterProgressEvents> for ::windows::core::IUnknown {
    fn from(value: &IDiscMasterProgressEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDiscMasterProgressEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDiscMasterProgressEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiscMasterProgressEvents {}
impl ::core::fmt::Debug for IDiscMasterProgressEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscMasterProgressEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiscMasterProgressEvents {
    type Vtable = IDiscMasterProgressEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec9e51c1_4e5d_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMasterProgressEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub QueryCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows::core::HRESULT,
    pub NotifyPnPActivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NotifyAddProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::HRESULT,
    pub NotifyBlockProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows::core::HRESULT,
    pub NotifyTrackProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::HRESULT,
    pub NotifyPreparingBurn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT,
    pub NotifyClosingDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT,
    pub NotifyBurnComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub NotifyEraseComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IDiscRecorder(::windows::core::IUnknown);
impl IDiscRecorder {
    pub unsafe fn Init(&self, pbyuniqueid: &[u8], nuldrivenumber: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pbyuniqueid)), pbyuniqueid.len() as _, nuldrivenumber).ok()
    }
    pub unsafe fn GetRecorderGUID(&self, pbyuniqueid: &mut [u8], pulreturnsizerequired: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecorderGUID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pbyuniqueid)), pbyuniqueid.len() as _, ::core::mem::transmute(pulreturnsizerequired)).ok()
    }
    pub unsafe fn GetRecorderType(&self) -> ::windows::core::Result<RECORDER_TYPES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecorderType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<RECORDER_TYPES>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayNames(&self, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrvendorid), ::core::mem::transmute(pbstrproductid), ::core::mem::transmute(pbstrrevision)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBasePnPID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBasePnPID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetRecorderProperties(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecorderProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::StructuredStorage::IPropertyStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetRecorderProperties<'a, P0>(&self, ppropstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::StructuredStorage::IPropertyStorage>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorderProperties)(::windows::core::Interface::as_raw(self), ppropstg.into().abi()).ok()
    }
    pub unsafe fn GetRecorderState(&self) -> ::windows::core::Result<DISC_RECORDER_STATE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecorderState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DISC_RECORDER_STATE_FLAGS>(result__)
    }
    pub unsafe fn OpenExclusive(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OpenExclusive)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryMediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(fmediatype), ::core::mem::transmute(fmediaflags)).ok()
    }
    pub unsafe fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryMediaInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsessions), ::core::mem::transmute(pblasttrack), ::core::mem::transmute(ulstartaddress), ::core::mem::transmute(ulnextwritable), ::core::mem::transmute(ulfreeblocks)).ok()
    }
    pub unsafe fn Eject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Eject)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Erase(&self, bfullerase: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Erase)(::windows::core::Interface::as_raw(self), bfullerase).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IDiscRecorder> for ::windows::core::IUnknown {
    fn from(value: IDiscRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDiscRecorder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscRecorder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscRecorder> for ::windows::core::IUnknown {
    fn from(value: &IDiscRecorder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDiscRecorder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDiscRecorder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiscRecorder {}
impl ::core::fmt::Debug for IDiscRecorder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscRecorder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiscRecorder {
    type Vtable = IDiscRecorder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85ac9776_ca88_4cf2_894e_09598c078a41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::HRESULT,
    pub GetRecorderGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::HRESULT,
    pub GetRecorderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayNames: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBasePnPID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBasePnPID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetRecorderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetRecorderProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetRecorderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetRecorderProperties: usize,
    pub GetRecorderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::core::HRESULT,
    pub OpenExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::HRESULT,
    pub QueryMediaInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::HRESULT,
    pub Eject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Erase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDiscRecorder2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EjectMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseTray(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseTray)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireExclusiveAccess<'a, P0>(&self, force: i16, __midl__idiscrecorder20000: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AcquireExclusiveAccess)(::windows::core::Interface::as_raw(self), force, __midl__idiscrecorder20000.into().abi()).ok()
    }
    pub unsafe fn ReleaseExclusiveAccess(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseExclusiveAccess)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableMcn(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableMcn)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableMcn(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableMcn)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeDiscRecorder<'a, P0>(&self, recorderuniqueid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).InitializeDiscRecorder)(::windows::core::Interface::as_raw(self), recorderuniqueid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ActiveDiscRecorder(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ActiveDiscRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VendorId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VendorId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductRevision(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProductRevision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VolumePathNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumePathNames)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn DeviceCanLoadMedia(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DeviceCanLoadMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn LegacyDeviceNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LegacyDeviceNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedFeaturePages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedFeaturePages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentFeaturePages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentFeaturePages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedProfiles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedProfiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentProfiles(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentProfiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedModePages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SupportedModePages)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExclusiveAccessOwner(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ExclusiveAccessOwner)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscRecorder2> for ::windows::core::IUnknown {
    fn from(value: IDiscRecorder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscRecorder2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscRecorder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscRecorder2> for ::windows::core::IUnknown {
    fn from(value: &IDiscRecorder2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDiscRecorder2> for super::super::System::Com::IDispatch {
    fn from(value: IDiscRecorder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDiscRecorder2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IDiscRecorder2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDiscRecorder2> for super::super::System::Com::IDispatch {
    fn from(value: &IDiscRecorder2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDiscRecorder2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDiscRecorder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDiscRecorder2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDiscRecorder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscRecorder2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDiscRecorder2 {
    type Vtable = IDiscRecorder2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354133_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireExclusiveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireExclusiveAccess: usize,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeDiscRecorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ActiveDiscRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ActiveDiscRecorder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VendorId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductRevision: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VolumePathNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VolumePathNames: usize,
    pub DeviceCanLoadMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub LegacyDeviceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentFeaturePages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CurrentProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CurrentProfiles: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedModePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedModePages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExclusiveAccessOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExclusiveAccessOwner: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IDiscRecorder2Ex(::windows::core::IUnknown);
impl IDiscRecorder2Ex {
    pub unsafe fn SendCommandNoData(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendCommandNoData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(cdb)), cdb.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(sensebuffer)), timeout).ok()
    }
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendCommandSendDataToDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(cdb)), cdb.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(sensebuffer)), timeout, ::core::mem::transmute(::windows::core::as_ptr_or_null(buffer)), buffer.len() as _).ok()
    }
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: &[u8], sensebuffer: &mut [u8; 18], timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendCommandGetDataFromDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(cdb)), cdb.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(sensebuffer)), timeout, ::core::mem::transmute(buffer), buffersize, ::core::mem::transmute(bufferfetched)).ok()
    }
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadDvdStructure)(::windows::core::Interface::as_raw(self), format, address, layer, agid, ::core::mem::transmute(data), ::core::mem::transmute(count)).ok()
    }
    pub unsafe fn SendDvdStructure(&self, format: u32, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendDvdStructure)(::windows::core::Interface::as_raw(self), format, ::core::mem::transmute(::windows::core::as_ptr_or_null(data)), data.len() as _).ok()
    }
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdapterDescriptor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data), ::core::mem::transmute(bytesize)).ok()
    }
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceDescriptor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data), ::core::mem::transmute(bytesize)).ok()
    }
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDiscInformation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(discinformation), ::core::mem::transmute(bytesize)).ok()
    }
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTrackInformation)(::windows::core::Interface::as_raw(self), address, addresstype, ::core::mem::transmute(trackinformation), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFeaturePage<'a, P0>(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: P0, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).GetFeaturePage)(::windows::core::Interface::as_raw(self), requestedfeature, currentfeatureonly.into(), ::core::mem::transmute(featuredata), ::core::mem::transmute(bytesize)).ok()
    }
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetModePage)(::windows::core::Interface::as_raw(self), requestedmodepage, requesttype, ::core::mem::transmute(modepagedata), ::core::mem::transmute(bytesize)).ok()
    }
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetModePage)(::windows::core::Interface::as_raw(self), requesttype, ::core::mem::transmute(::windows::core::as_ptr_or_null(data)), data.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedFeaturePages<'a, P0>(&self, currentfeatureonly: P0, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).GetSupportedFeaturePages)(::windows::core::Interface::as_raw(self), currentfeatureonly.into(), ::core::mem::transmute(featuredata), ::core::mem::transmute(bytesize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSupportedProfiles<'a, P0>(&self, currentonly: P0, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).GetSupportedProfiles)(::windows::core::Interface::as_raw(self), currentonly.into(), ::core::mem::transmute(profiletypes), ::core::mem::transmute(validprofiles)).ok()
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSupportedModePages)(::windows::core::Interface::as_raw(self), requesttype, ::core::mem::transmute(modepagetypes), ::core::mem::transmute(validpages)).ok()
    }
    pub unsafe fn GetByteAlignmentMask(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetByteAlignmentMask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaximumPageAlignedTransferSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDiscRecorder2Ex> for ::windows::core::IUnknown {
    fn from(value: IDiscRecorder2Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDiscRecorder2Ex> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDiscRecorder2Ex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDiscRecorder2Ex> for ::windows::core::IUnknown {
    fn from(value: &IDiscRecorder2Ex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDiscRecorder2Ex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDiscRecorder2Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDiscRecorder2Ex {}
impl ::core::fmt::Debug for IDiscRecorder2Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDiscRecorder2Ex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDiscRecorder2Ex {
    type Vtable = IDiscRecorder2Ex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354132_7f64_5b0f_8f00_5d77afbe261e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2Ex_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SendCommandNoData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows::core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFeaturePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFeaturePage: usize,
    pub GetModePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedFeaturePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedFeaturePages: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSupportedProfiles: usize,
    pub GetSupportedModePages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IEnumDiscMasterFormats(::windows::core::IUnknown);
impl IEnumDiscMasterFormats {
    pub unsafe fn Next(&self, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cformats, ::core::mem::transmute(lpiidformatid), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cformats: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cformats).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDiscMasterFormats> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDiscMasterFormats>(result__)
    }
}
impl ::core::convert::From<IEnumDiscMasterFormats> for ::windows::core::IUnknown {
    fn from(value: IEnumDiscMasterFormats) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumDiscMasterFormats> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumDiscMasterFormats) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDiscMasterFormats> for ::windows::core::IUnknown {
    fn from(value: &IEnumDiscMasterFormats) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumDiscMasterFormats {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDiscMasterFormats {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDiscMasterFormats {}
impl ::core::fmt::Debug for IEnumDiscMasterFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDiscMasterFormats").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDiscMasterFormats {
    type Vtable = IEnumDiscMasterFormats_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddf445e1_54ba_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscMasterFormats_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IEnumDiscRecorders(::windows::core::IUnknown);
impl IEnumDiscRecorders {
    pub unsafe fn Next(&self, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), crecorders, ::core::mem::transmute(pprecorder), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, crecorders: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), crecorders).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDiscRecorders> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumDiscRecorders>(result__)
    }
}
impl ::core::convert::From<IEnumDiscRecorders> for ::windows::core::IUnknown {
    fn from(value: IEnumDiscRecorders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumDiscRecorders> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumDiscRecorders) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDiscRecorders> for ::windows::core::IUnknown {
    fn from(value: &IEnumDiscRecorders) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumDiscRecorders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDiscRecorders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDiscRecorders {}
impl ::core::fmt::Debug for IEnumDiscRecorders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDiscRecorders").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDiscRecorders {
    type Vtable = IEnumDiscRecorders_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b1921e1_54ac_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDiscRecorders_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IEnumFsiItems(::windows::core::IUnknown);
impl IEnumFsiItems {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IFsiItem>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFsiItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFsiItems>(result__)
    }
}
impl ::core::convert::From<IEnumFsiItems> for ::windows::core::IUnknown {
    fn from(value: IEnumFsiItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumFsiItems> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumFsiItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFsiItems> for ::windows::core::IUnknown {
    fn from(value: &IEnumFsiItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumFsiItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFsiItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFsiItems {}
impl ::core::fmt::Debug for IEnumFsiItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFsiItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumFsiItems {
    type Vtable = IEnumFsiItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fda_975b_59be_a960_9a2a262853a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IEnumProgressItems(::windows::core::IUnknown);
impl IEnumProgressItems {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IProgressItem>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumProgressItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumProgressItems>(result__)
    }
}
impl ::core::convert::From<IEnumProgressItems> for ::windows::core::IUnknown {
    fn from(value: IEnumProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumProgressItems> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumProgressItems> for ::windows::core::IUnknown {
    fn from(value: &IEnumProgressItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumProgressItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumProgressItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumProgressItems {}
impl ::core::fmt::Debug for IEnumProgressItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumProgressItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumProgressItems {
    type Vtable = IEnumProgressItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd6_975b_59be_a960_9a2a262853a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFileSystemImage(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows::core::Result<IFsiDirectoryItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Root)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SessionStartBlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSessionStartBlock)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FreeMediaBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFreeMediaBlocks)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UsedBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVolumeName<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetVolumeName)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ImportedVolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows::core::Result<IBootOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BootImageOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBootOptions>>,
    {
        (::windows::core::Interface::vtable(self).SetBootImageOptions)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DirectoryCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetWorkingDirectory)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ChangePoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FileSystemsToCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileSystemsToCreate)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FileSystemsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUDFRevision)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UDFRevision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UDFRevisionsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).ChooseImageDefaults)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ISO9660InterchangeLevelsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows::core::Result<IFileSystemImageResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateResultImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFileSystemImageResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Exists<'a, P0>(&self, fullpath: P0) -> ::windows::core::Result<FsiItemType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Exists)(::windows::core::Interface::as_raw(self), fullpath.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiItemType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CalculateDiscIdentifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<FsiFileSystems>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IdentifyFileSystemsOnDisc)(::windows::core::Interface::as_raw(self), discrecorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultFileSystemForImport)(::windows::core::Interface::as_raw(self), filesystems, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ImportFileSystem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImportSpecificFileSystem)(::windows::core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RollbackToChangePoint)(::windows::core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockInChangePoint)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDirectoryItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiDirectoryItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDirectoryItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFileItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiFileItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateFileItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiFileItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumeNameUDF)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumeNameJoliet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VolumeNameISO9660)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn StageFiles(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StageFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStageFiles)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage> for ::windows::core::IUnknown {
    fn from(value: IFileSystemImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSystemImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage> for ::windows::core::IUnknown {
    fn from(value: &IFileSystemImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFileSystemImage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFileSystemImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSystemImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSystemImage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSystemImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemImage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFileSystemImage {
    type Vtable = IFileSystemImage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fe1_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Root: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Root: usize,
    pub SessionStartBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaxMediaBlocksFromDevice: usize,
    pub UsedBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVolumeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportedVolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportedVolumeName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptions: usize,
    pub FileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
    pub ChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub StrictFileSystemCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    pub FileSystemsToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows::core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UDFRevisionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UDFRevisionsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ChooseImageDefaults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ChooseImageDefaults: usize,
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ISO9660InterchangeLevelsSupported: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Exists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Exists: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CalculateDiscIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CalculateDiscIdentifier: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void, filesystems: *mut FsiFileSystems) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows::core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows::core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows::core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateDirectoryItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateDirectoryItem: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateFileItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateFileItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameUDF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameUDF: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameJoliet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameJoliet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub VolumeNameISO9660: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VolumeNameISO9660: usize,
    pub StageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetStageFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MultisessionInterfaces: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMultisessionInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMultisessionInterfaces: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFileSystemImage2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows::core::Result<IFsiDirectoryItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Root)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.SessionStartBlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSessionStartBlock)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FreeMediaBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFreeMediaBlocks)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetMaxMediaBlocksFromDevice)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UsedBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.VolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVolumeName<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetVolumeName)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ImportedVolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows::core::Result<IBootOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BootImageOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBootOptions>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBootImageOptions)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.DirectoryCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.WorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetWorkingDirectory)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ChangePoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetUseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemsToCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFileSystemsToCreate)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetUDFRevision)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UDFRevision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.UDFRevisionsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).base__.ChooseImageDefaults)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ChooseImageDefaultsForMediaType)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ISO9660InterchangeLevelsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows::core::Result<IFileSystemImageResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateResultImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFileSystemImageResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Exists<'a, P0>(&self, fullpath: P0) -> ::windows::core::Result<FsiItemType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Exists)(::windows::core::Interface::as_raw(self), fullpath.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiItemType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CalculateDiscIdentifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<FsiFileSystems>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IdentifyFileSystemsOnDisc)(::windows::core::Interface::as_raw(self), discrecorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDefaultFileSystemForImport)(::windows::core::Interface::as_raw(self), filesystems, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ImportFileSystem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ImportSpecificFileSystem)(::windows::core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RollbackToChangePoint)(::windows::core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockInChangePoint)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDirectoryItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiDirectoryItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDirectoryItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFileItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiFileItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateFileItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiFileItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.VolumeNameUDF)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.VolumeNameJoliet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.VolumeNameISO9660)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn StageFiles(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StageFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStageFiles)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BootImageOptionsArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBootImageOptionsArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage2> for ::windows::core::IUnknown {
    fn from(value: IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage2> for ::windows::core::IUnknown {
    fn from(value: &IFileSystemImage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage2> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage2> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage2> for IFileSystemImage {
    fn from(value: IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage2> for &'a IFileSystemImage {
    fn from(value: &'a IFileSystemImage2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage2> for IFileSystemImage {
    fn from(value: &IFileSystemImage2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFileSystemImage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSystemImage2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSystemImage2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSystemImage2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemImage2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFileSystemImage2 {
    type Vtable = IFileSystemImage2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BootImageOptionsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BootImageOptionsArray: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBootImageOptionsArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBootImageOptionsArray: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFileSystemImage3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Root(&self) -> ::windows::core::Result<IFsiDirectoryItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Root)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    pub unsafe fn SessionStartBlock(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.SessionStartBlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSessionStartBlock)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FreeMediaBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FreeMediaBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFreeMediaBlocks)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetMaxMediaBlocksFromDevice)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn UsedBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.UsedBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.VolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVolumeName<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetVolumeName)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportedVolumeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ImportedVolumeName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptions(&self) -> ::windows::core::Result<IBootOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.BootImageOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBootOptions>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptions<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IBootOptions>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetBootImageOptions)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn FileCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DirectoryCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.DirectoryCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.WorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetWorkingDirectory)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    pub unsafe fn ChangePoint(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ChangePoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn StrictFileSystemCompliance(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.StrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetStrictFileSystemCompliance)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UseRestrictedCharacterSet(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.UseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetUseRestrictedCharacterSet)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsToCreate(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemsToCreate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetFileSystemsToCreate)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn FileSystemsSupported(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetUDFRevision)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn UDFRevision(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.UDFRevision)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UDFRevisionsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.UDFRevisionsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ChooseImageDefaults<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ChooseImageDefaults)(::windows::core::Interface::as_raw(self), discrecorder.into().abi()).ok()
    }
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ChooseImageDefaultsForMediaType)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ISO9660InterchangeLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ISO9660InterchangeLevelsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows::core::Result<IFileSystemImageResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreateResultImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFileSystemImageResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Exists<'a, P0>(&self, fullpath: P0) -> ::windows::core::Result<FsiItemType>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Exists)(::windows::core::Interface::as_raw(self), fullpath.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiItemType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CalculateDiscIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CalculateDiscIdentifier)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdentifyFileSystemsOnDisc<'a, P0>(&self, discrecorder: P0) -> ::windows::core::Result<FsiFileSystems>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.IdentifyFileSystemsOnDisc)(::windows::core::Interface::as_raw(self), discrecorder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetDefaultFileSystemForImport)(::windows::core::Interface::as_raw(self), filesystems, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportFileSystem(&self) -> ::windows::core::Result<FsiFileSystems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ImportFileSystem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FsiFileSystems>(result__)
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ImportSpecificFileSystem)(::windows::core::Interface::as_raw(self), filesystemtouse).ok()
    }
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RollbackToChangePoint)(::windows::core::Interface::as_raw(self), changepoint).ok()
    }
    pub unsafe fn LockInChangePoint(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.LockInChangePoint)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDirectoryItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiDirectoryItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreateDirectoryItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiDirectoryItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFileItem<'a, P0>(&self, name: P0) -> ::windows::core::Result<IFsiFileItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreateFileItem)(::windows::core::Interface::as_raw(self), name.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiFileItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameUDF(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.VolumeNameUDF)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameJoliet(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.VolumeNameJoliet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeNameISO9660(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.VolumeNameISO9660)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn StageFiles(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.StageFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStageFiles(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetStageFiles)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MultisessionInterfaces(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.MultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetMultisessionInterfaces)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BootImageOptionsArray(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BootImageOptionsArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBootImageOptionsArray)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateRedundantUdfMetadataFiles)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetCreateRedundantUdfMetadataFiles(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreateRedundantUdfMetadataFiles)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProbeSpecificFileSystem)(::windows::core::Interface::as_raw(self), filesystemtoprobe, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage3> for ::windows::core::IUnknown {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage3> for ::windows::core::IUnknown {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage3> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage3> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage3> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage3> for IFileSystemImage {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage3> for &'a IFileSystemImage {
    fn from(value: &'a IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage3> for IFileSystemImage {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImage3> for IFileSystemImage2 {
    fn from(value: IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImage3> for &'a IFileSystemImage2 {
    fn from(value: &'a IFileSystemImage3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImage3> for IFileSystemImage2 {
    fn from(value: &IFileSystemImage3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFileSystemImage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSystemImage3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSystemImage3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSystemImage3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemImage3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFileSystemImage3 {
    type Vtable = IFileSystemImage3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cff842c_7e97_4807_8304_910dd8f7c051);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFileSystemImageResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImageStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ImageStream)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItems(&self) -> ::windows::core::Result<IProgressItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProgressItems)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProgressItems>(result__)
    }
    pub unsafe fn TotalBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BlockSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DiscId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult> for ::windows::core::IUnknown {
    fn from(value: IFileSystemImageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImageResult> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSystemImageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult> for ::windows::core::IUnknown {
    fn from(value: &IFileSystemImageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImageResult> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFileSystemImageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFileSystemImageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSystemImageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSystemImageResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSystemImageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemImageResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFileSystemImageResult {
    type Vtable = IFileSystemImageResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd8_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ImageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImageStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItems: usize,
    pub TotalBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DiscId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DiscId: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFileSystemImageResult2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImageStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ImageStream)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItems(&self) -> ::windows::core::Result<IProgressItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProgressItems)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProgressItems>(result__)
    }
    pub unsafe fn TotalBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.TotalBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn BlockSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.BlockSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.DiscId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModifiedBlocks(&self) -> ::windows::core::Result<IBlockRangeList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ModifiedBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBlockRangeList>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult2> for ::windows::core::IUnknown {
    fn from(value: IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImageResult2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult2> for ::windows::core::IUnknown {
    fn from(value: &IFileSystemImageResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult2> for super::super::System::Com::IDispatch {
    fn from(value: IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImageResult2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult2> for super::super::System::Com::IDispatch {
    fn from(value: &IFileSystemImageResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFileSystemImageResult2> for IFileSystemImageResult {
    fn from(value: IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFileSystemImageResult2> for &'a IFileSystemImageResult {
    fn from(value: &'a IFileSystemImageResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFileSystemImageResult2> for IFileSystemImageResult {
    fn from(value: &IFileSystemImageResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFileSystemImageResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFileSystemImageResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFileSystemImageResult2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFileSystemImageResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSystemImageResult2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFileSystemImageResult2 {
    type Vtable = IFileSystemImageResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca29_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModifiedBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModifiedBlocks: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiDirectoryItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FullPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCreationTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastAccessedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastAccessedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastModifiedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastModifiedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsHidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetIsHidden)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemName)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemPath)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_Item<'a, P0>(&self, path: P0) -> ::windows::core::Result<IFsiItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiItem>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EnumFsiItems(&self) -> ::windows::core::Result<IEnumFsiItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumFsiItems)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFsiItems>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectory<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddDirectory)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddFile<'a, P0, P1>(&self, path: P0, filedata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AddFile)(::windows::core::Interface::as_raw(self), path.into().abi(), filedata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTree<'a, P0>(&self, sourcedirectory: P0, includebasedirectory: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddTree)(::windows::core::Interface::as_raw(self), sourcedirectory.into().abi(), includebasedirectory).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, P0>(&self, item: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFsiItem>>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), item.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveTree<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).RemoveTree)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem> for ::windows::core::IUnknown {
    fn from(value: IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem> for ::windows::core::IUnknown {
    fn from(value: &IFsiDirectoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiDirectoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem> for IFsiItem {
    fn from(value: IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem> for &'a IFsiItem {
    fn from(value: &'a IFsiDirectoryItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem> for IFsiItem {
    fn from(value: &IFsiDirectoryItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiDirectoryItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiDirectoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiDirectoryItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiDirectoryItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiDirectoryItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiDirectoryItem {
    type Vtable = IFsiDirectoryItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fdc_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDirectory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTree: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveTree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveTree: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiDirectoryItem2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FullPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetCreationTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.LastAccessedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastAccessedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.LastModifiedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastModifiedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.IsHidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemName)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemPath)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn get_Item<'a, P0>(&self, path: P0) -> ::windows::core::Result<IFsiItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.get_Item)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiItem>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EnumFsiItems(&self) -> ::windows::core::Result<IEnumFsiItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.EnumFsiItems)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFsiItems>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDirectory<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDirectory)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddFile<'a, P0, P1>(&self, path: P0, filedata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.AddFile)(::windows::core::Interface::as_raw(self), path.into().abi(), filedata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTree<'a, P0>(&self, sourcedirectory: P0, includebasedirectory: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.AddTree)(::windows::core::Interface::as_raw(self), sourcedirectory.into().abi(), includebasedirectory).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, P0>(&self, item: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFsiItem>>,
    {
        (::windows::core::Interface::vtable(self).base__.Add)(::windows::core::Interface::as_raw(self), item.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.Remove)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveTree<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveTree)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddTreeWithNamedStreams<'a, P0>(&self, sourcedirectory: P0, includebasedirectory: i16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddTreeWithNamedStreams)(::windows::core::Interface::as_raw(self), sourcedirectory.into().abi(), includebasedirectory).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem2> for ::windows::core::IUnknown {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem2> for ::windows::core::IUnknown {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem2> for super::super::System::Com::IDispatch {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem2> for IFsiItem {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem2> for &'a IFsiItem {
    fn from(value: &'a IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem2> for IFsiItem {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiDirectoryItem2> for IFsiDirectoryItem {
    fn from(value: IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiDirectoryItem2> for &'a IFsiDirectoryItem {
    fn from(value: &'a IFsiDirectoryItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiDirectoryItem2> for IFsiDirectoryItem {
    fn from(value: &IFsiDirectoryItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiDirectoryItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiDirectoryItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiDirectoryItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiDirectoryItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiDirectoryItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiDirectoryItem2 {
    type Vtable = IFsiDirectoryItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddTreeWithNamedStreams: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiFileItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FullPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCreationTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastAccessedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastAccessedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastModifiedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastModifiedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsHidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetIsHidden)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemName)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FileSystemPath)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn DataSize(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DataSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn DataSize32BitLow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DataSize32BitLow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DataSize32BitHigh)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetData<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem> for ::windows::core::IUnknown {
    fn from(value: IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem> for ::windows::core::IUnknown {
    fn from(value: &IFsiFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem> for IFsiItem {
    fn from(value: IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem> for &'a IFsiItem {
    fn from(value: &'a IFsiFileItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem> for IFsiItem {
    fn from(value: &IFsiFileItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiFileItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiFileItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiFileItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiFileItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiFileItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiFileItem {
    type Vtable = IFsiFileItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fdb_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub DataSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows::core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Data: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetData: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiFileItem2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FullPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetCreationTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.LastAccessedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastAccessedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.LastModifiedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastModifiedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.IsHidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemName)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.FileSystemPath)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn DataSize(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.DataSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn DataSize32BitLow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.DataSize32BitLow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DataSize32BitHigh(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.DataSize32BitHigh)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Data(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetData<'a, P0>(&self, newval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), newval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FsiNamedStreams(&self) -> ::windows::core::Result<IFsiNamedStreams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FsiNamedStreams)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiNamedStreams>(result__)
    }
    pub unsafe fn IsNamedStream(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsNamedStream)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddStream<'a, P0, P1>(&self, name: P0, streamdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AddStream)(::windows::core::Interface::as_raw(self), name.into().abi(), streamdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveStream<'a, P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).RemoveStream)(::windows::core::Interface::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsRealTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsRealTime(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsRealTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem2> for ::windows::core::IUnknown {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem2> for ::windows::core::IUnknown {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem2> for super::super::System::Com::IDispatch {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem2> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem2> for IFsiItem {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem2> for &'a IFsiItem {
    fn from(value: &'a IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem2> for IFsiItem {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiFileItem2> for IFsiFileItem {
    fn from(value: IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiFileItem2> for &'a IFsiFileItem {
    fn from(value: &'a IFsiFileItem2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiFileItem2> for IFsiFileItem {
    fn from(value: &IFsiFileItem2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiFileItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiFileItem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiFileItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiFileItem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiFileItem2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiFileItem2 {
    type Vtable = IFsiFileItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FsiNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FsiNamedStreams: usize,
    pub IsNamedStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStream: usize,
    pub IsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetIsRealTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiItem {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FullPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreationTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCreationTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastAccessedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastAccessedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastAccessedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn LastModifiedTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastModifiedTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastModifiedTime)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn IsHidden(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsHidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetIsHidden(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIsHidden)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FileSystemName)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FileSystemPath)(::windows::core::Interface::as_raw(self), filesystem, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiItem> for ::windows::core::IUnknown {
    fn from(value: IFsiItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiItem> for ::windows::core::IUnknown {
    fn from(value: &IFsiItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiItem> for super::super::System::Com::IDispatch {
    fn from(value: IFsiItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiItem> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiItem> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiItem {
    type Vtable = IFsiItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd9_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FullPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FullPath: usize,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FileSystemName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileSystemName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FileSystemPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FileSystemPath: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsiNamedStreams(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsiNamedStreams {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<IFsiFileItem2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFsiFileItem2>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn EnumNamedStreams(&self) -> ::windows::core::Result<IEnumFsiItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumNamedStreams)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFsiItems>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiNamedStreams> for ::windows::core::IUnknown {
    fn from(value: IFsiNamedStreams) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiNamedStreams> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFsiNamedStreams) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiNamedStreams> for ::windows::core::IUnknown {
    fn from(value: &IFsiNamedStreams) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IFsiNamedStreams> for super::super::System::Com::IDispatch {
    fn from(value: IFsiNamedStreams) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IFsiNamedStreams> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IFsiNamedStreams) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IFsiNamedStreams> for super::super::System::Com::IDispatch {
    fn from(value: &IFsiNamedStreams) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsiNamedStreams {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsiNamedStreams {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsiNamedStreams {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsiNamedStreams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsiNamedStreams").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsiNamedStreams {
    type Vtable = IFsiNamedStreams_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed79ba56_5294_4250_8d46_f9aecee23459);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIsoImageManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIsoImageManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Stream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Stream)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, P0>(&self, val: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPath)(::windows::core::Interface::as_raw(self), val.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetStream<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).SetStream)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    pub unsafe fn Validate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIsoImageManager> for ::windows::core::IUnknown {
    fn from(value: IIsoImageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIsoImageManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IIsoImageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIsoImageManager> for ::windows::core::IUnknown {
    fn from(value: &IIsoImageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIsoImageManager> for super::super::System::Com::IDispatch {
    fn from(value: IIsoImageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIsoImageManager> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IIsoImageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIsoImageManager> for super::super::System::Com::IDispatch {
    fn from(value: &IIsoImageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIsoImageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIsoImageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIsoImageManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIsoImageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsoImageManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIsoImageManager {
    type Vtable = IIsoImageManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Stream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPath: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IJolietDiscMaster(::windows::core::IUnknown);
impl IJolietDiscMaster {
    pub unsafe fn GetTotalDataBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTotalDataBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetUsedDataBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUsedDataBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetDataBlockSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDataBlockSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn AddData<'a, P0>(&self, pstorage: P0, lfileoverwrite: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::StructuredStorage::IStorage>>,
    {
        (::windows::core::Interface::vtable(self).AddData)(::windows::core::Interface::as_raw(self), pstorage.into().abi(), lfileoverwrite).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetJolietProperties(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetJolietProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::StructuredStorage::IPropertyStorage>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetJolietProperties<'a, P0>(&self, ppropstg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::StructuredStorage::IPropertyStorage>>,
    {
        (::windows::core::Interface::vtable(self).SetJolietProperties)(::windows::core::Interface::as_raw(self), ppropstg.into().abi()).ok()
    }
}
impl ::core::convert::From<IJolietDiscMaster> for ::windows::core::IUnknown {
    fn from(value: IJolietDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IJolietDiscMaster> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IJolietDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IJolietDiscMaster> for ::windows::core::IUnknown {
    fn from(value: &IJolietDiscMaster) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IJolietDiscMaster {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IJolietDiscMaster {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJolietDiscMaster {}
impl ::core::fmt::Debug for IJolietDiscMaster {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJolietDiscMaster").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IJolietDiscMaster {
    type Vtable = IJolietDiscMaster_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bc42ce_4e5c_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJolietDiscMaster_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetTotalDataBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT,
    pub GetUsedDataBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT,
    pub GetDataBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub AddData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstorage: *mut ::core::ffi::c_void, lfileoverwrite: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    AddData: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetJolietProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetJolietProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetJolietProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetJolietProperties: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_FullVersion_STR: &str = "1.0";
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_FullVersion_WSTR: &str = "1.0";
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2FS_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPILib2_MajorVersion: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPILib2_MinorVersion: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_BURN_VERIFICATION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = IMAPI_BURN_VERIFICATION_LEVEL(2i32);
impl ::core::marker::Copy for IMAPI_BURN_VERIFICATION_LEVEL {}
impl ::core::clone::Clone for IMAPI_BURN_VERIFICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_BURN_VERIFICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_BURN_VERIFICATION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_BURN_VERIFICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_BURN_VERIFICATION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_CD_SECTOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = IMAPI_CD_SECTOR_TYPE(9i32);
impl ::core::marker::Copy for IMAPI_CD_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_CD_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_CD_SECTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_CD_SECTOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_CD_SECTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_CD_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = IMAPI_CD_TRACK_DIGITAL_COPY_SETTING(2i32);
impl ::core::marker::Copy for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {}
impl ::core::clone::Clone for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_CD_TRACK_DIGITAL_COPY_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_CD_TRACK_DIGITAL_COPY_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_ALREADYOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220958i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_BADJOLIETNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220963i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_BOOTIMAGE_AND_NONBLANK_DISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220946i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_CANNOT_WRITE_TO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220948i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_COMPRESSEDSTASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220952i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220972i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOPROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220975i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOTACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220974i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_NOTPRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220973i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DEVICE_STILL_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220954i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DISCFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220964i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_DISCINFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220967i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_ENCRYPTEDSTASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220951i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILEACCESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220968i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILEEXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220956i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_FILESYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220969i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_GENERIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220978i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INITIALIZE_ENDWRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220970i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INITIALIZE_WRITE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220971i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_INVALIDIMAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220962i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_LOSS_OF_STREAMING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220953i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_MEDIUM_INVALIDTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220976i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_MEDIUM_NOTPRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220977i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOACTIVEFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220961i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOACTIVERECORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220960i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTENOUGHDISKFORSTASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220950i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTINITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220980i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_NOTOPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220981i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_REMOVABLESTASH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220949i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_STASHINUSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220955i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACKNOTOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220966i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACKOPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220965i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_TRACK_NOT_BIG_ENOUGH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220947i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_USERABORT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220979i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_WRONGDISC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220957i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_E_WRONGFORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220959i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FEATURE_PAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(56i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(59i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(81i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(256i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(257i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(258i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(259i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(260i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(261i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(262i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(263i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(264i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(265i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(266i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(267i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(268i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(269i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = IMAPI_FEATURE_PAGE_TYPE(272i32);
impl ::core::marker::Copy for IMAPI_FEATURE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_FEATURE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FEATURE_PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FEATURE_PAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FEATURE_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FEATURE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_DATA_MEDIA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(64512i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(2048i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(4096i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(8192i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(16384i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = IMAPI_FORMAT2_DATA_MEDIA_STATE(32768i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_MEDIA_STATE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_DATA_MEDIA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_MEDIA_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_DATA_WRITE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(6i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = IMAPI_FORMAT2_DATA_WRITE_ACTION(7i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_DATA_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_DATA_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_DATA_WRITE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE(3i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = IMAPI_FORMAT2_RAW_CD_WRITE_ACTION(3i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_RAW_CD_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_RAW_CD_WRITE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_FORMAT2_TAO_WRITE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = IMAPI_FORMAT2_TAO_WRITE_ACTION(4i32);
impl ::core::marker::Copy for IMAPI_FORMAT2_TAO_WRITE_ACTION {}
impl ::core::clone::Clone for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_FORMAT2_TAO_WRITE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_FORMAT2_TAO_WRITE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MEDIA_PHYSICAL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = IMAPI_MEDIA_PHYSICAL_TYPE(19i32);
impl ::core::marker::Copy for IMAPI_MEDIA_PHYSICAL_TYPE {}
impl ::core::clone::Clone for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_MEDIA_PHYSICAL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_MEDIA_PHYSICAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_PHYSICAL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MEDIA_WRITE_PROTECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = IMAPI_MEDIA_WRITE_PROTECT_STATE(16384i32);
impl ::core::marker::Copy for IMAPI_MEDIA_WRITE_PROTECT_STATE {}
impl ::core::clone::Clone for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_MEDIA_WRITE_PROTECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MEDIA_WRITE_PROTECT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MODE_PAGE_REQUEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = IMAPI_MODE_PAGE_REQUEST_TYPE(3i32);
impl ::core::marker::Copy for IMAPI_MODE_PAGE_REQUEST_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_MODE_PAGE_REQUEST_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_MODE_PAGE_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_MODE_PAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = IMAPI_MODE_PAGE_TYPE(42i32);
impl ::core::marker::Copy for IMAPI_MODE_PAGE_TYPE {}
impl ::core::clone::Clone for IMAPI_MODE_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_MODE_PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_MODE_PAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_MODE_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_MODE_PAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_PROFILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(81i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(82i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = IMAPI_PROFILE_TYPE(65535i32);
impl ::core::marker::Copy for IMAPI_PROFILE_TYPE {}
impl ::core::clone::Clone for IMAPI_PROFILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_PROFILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_PROFILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_PROFILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_PROFILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAPI_READ_TRACK_ADDRESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = IMAPI_READ_TRACK_ADDRESS_TYPE(2i32);
impl ::core::marker::Copy for IMAPI_READ_TRACK_ADDRESS_TYPE {}
impl ::core::clone::Clone for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMAPI_READ_TRACK_ADDRESS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMAPI_READ_TRACK_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAPI_READ_TRACK_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_SECTOR_SIZE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_S_BUFFER_TO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(262657i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMAPI_S_PROPERTIESIGNORED: ::windows::core::HRESULT = ::windows::core::HRESULT(262656i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_CPV_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CPV_BEFORE__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32767i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CP_START: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32768i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_CPV_AFTER__: IMMPID_CPV_ENUM = IMMPID_CPV_ENUM(32769i32);
impl ::core::marker::Copy for IMMPID_CPV_ENUM {}
impl ::core::clone::Clone for IMMPID_CPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_CPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_CPV_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_CPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_CPV_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_MPV_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_BEFORE__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12287i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_STORE_DRIVER_HANDLE: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12288i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_MESSAGE_CREATION_FLAGS: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12289i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_MESSAGE_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12290i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12291i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_PROPERTY_STREAM_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12292i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_TOTAL_OPEN_CONTENT_HANDLES: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12293i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MPV_AFTER__: IMMPID_MPV_ENUM = IMMPID_MPV_ENUM(12294i32);
impl ::core::marker::Copy for IMMPID_MPV_ENUM {}
impl ::core::clone::Clone for IMMPID_MPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_MPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_MPV_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_MPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_MPV_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_MP_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_BEFORE__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4095i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RECIPIENT_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4096i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONTENT_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4097i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_SMTP: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4098i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_X500: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4099i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_X400: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4100i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_LEGACY_EX_DN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4101i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DOMAIN_LIST: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4102i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_PICKUP_FILE_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4103i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_AUTHENTICATED_USER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4104i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4105i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_HELO_DOMAIN: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4106i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EIGHTBIT_MIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4107i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CHUNKING_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4108i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_BINARYMIME_OPTION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4109i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_REMOTE_AUTHENTICATION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4110i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ERROR_CODE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4111i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DSN_ENVID_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4112i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DSN_RET_VALUE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4113i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_REMOTE_SERVER_DSN_CAPABLE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4114i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4115i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MESSAGE_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4116i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4117i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4118i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_LOCAL_EXPIRE_DELAY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4119i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_LOCAL_EXPIRE_NDR: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4120i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ARRIVAL_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4121i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_HR_CAT_STATUS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4122i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4123i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SUPERSEDES_MSG_GUID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4124i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SCANNED_FOR_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4125i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_FOUND_EMBEDDED_CRLF_DOT_CRLF: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4126i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSG_SIZE_HINT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4127i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_MSG_ID: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4128i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_MSG_SUBJECT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4129i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4130i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_TO_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4131i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_CC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4132i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_RFC822_BCC_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4133i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_SERVER_IP_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4134i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SERVER_NAME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4135i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SERVER_VERSION: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4136i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_NUM_RECIPIENTS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4137i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_X_PRIORITY: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4138i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_FROM_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4139i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4140i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_DEFERRED_DELIVERY_FILETIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4141i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_SENDER_ADDRESS_OTHER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4142i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ORIGINAL_ARRIVAL_TIME: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4143i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_MSGCLASS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4144i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONTENT_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4145i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_ENCRYPTION_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4146i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CONNECTION_SERVER_PORT: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4147i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CLIENT_AUTH_USER: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4148i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CLIENT_AUTH_TYPE: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4149i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CRC_GLOBAL: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4150i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_CRC_RECIPS: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4151i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_INBOUND_MAIL_FROM_AUTH: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4152i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_MP_AFTER__: IMMPID_MP_ENUM = IMMPID_MP_ENUM(4153i32);
impl ::core::marker::Copy for IMMPID_MP_ENUM {}
impl ::core::clone::Clone for IMMPID_MP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_MP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_MP_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_MP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_MP_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_NMP_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_BEFORE__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24575i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_SECONDARY_GROUPS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24576i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_SECONDARY_ARTNUM: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24577i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_PRIMARY_GROUP: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24578i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_PRIMARY_ARTID: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24579i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_POST_TOKEN: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24580i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NEWSGROUP_LIST: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24581i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_HEADERS: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24582i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NNTP_PROCESSING: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24583i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_NNTP_APPROVED_HEADER: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24584i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_NMP_AFTER__: IMMPID_NMP_ENUM = IMMPID_NMP_ENUM(24585i32);
impl ::core::marker::Copy for IMMPID_NMP_ENUM {}
impl ::core::clone::Clone for IMMPID_NMP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_NMP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_NMP_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_NMP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_NMP_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_RPV_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_BEFORE__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16383i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_DONT_DELIVER: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16384i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_NO_NAME_COLLISIONS: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16385i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RPV_AFTER__: IMMPID_RPV_ENUM = IMMPID_RPV_ENUM(16386i32);
impl ::core::marker::Copy for IMMPID_RPV_ENUM {}
impl ::core::clone::Clone for IMMPID_RPV_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_RPV_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_RPV_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_RPV_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_RPV_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMMPID_RP_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_BEFORE__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8191i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_SUCCESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8192i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_INVALID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8193i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_TYPE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8194i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8195i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_TYPE_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8196i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ERROR_CODE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8197i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ERROR_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8198i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_NOTIFY_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8199i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_ORCPT_VALUE: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8200i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_SMTP: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8201i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_X400: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8202i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_X500: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8203i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_LEGACY_EX_DN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8204i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_RECIPIENT_FLAGS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8205i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_SMTP_STATUS_STRING: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8206i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DSN_PRE_CAT_ADDRESS: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8207i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_MDB_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8208i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_USER_GUID: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8209i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DOMAIN: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8210i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_ADDRESS_OTHER: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8211i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_DISPLAY_NAME: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8212i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const IMMPID_RP_AFTER__: IMMPID_RP_ENUM = IMMPID_RP_ENUM(8213i32);
impl ::core::marker::Copy for IMMPID_RP_ENUM {}
impl ::core::clone::Clone for IMMPID_RP_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMMPID_RP_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IMMPID_RP_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for IMMPID_RP_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMPID_RP_ENUM").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct IMMP_MPV_STORE_DRIVER_HANDLE {
    pub guidSignature: ::windows::core::GUID,
}
impl ::core::marker::Copy for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::clone::Clone for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMMP_MPV_STORE_DRIVER_HANDLE").field("guidSignature", &self.guidSignature).finish()
    }
}
unsafe impl ::windows::core::Abi for IMMP_MPV_STORE_DRIVER_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IMMP_MPV_STORE_DRIVER_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for IMMP_MPV_STORE_DRIVER_HANDLE {}
impl ::core::default::Default for IMMP_MPV_STORE_DRIVER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMultisession(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisession {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsSupportedOnCurrentMediaState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInUse)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn InUse(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InUse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ImportRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisession> for ::windows::core::IUnknown {
    fn from(value: IMultisession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisession> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultisession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisession> for ::windows::core::IUnknown {
    fn from(value: &IMultisession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisession> for super::super::System::Com::IDispatch {
    fn from(value: IMultisession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisession> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IMultisession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisession> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMultisession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMultisession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMultisession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMultisession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultisession").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMultisession {
    type Vtable = IMultisession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354150_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub SetInUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub InUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ImportRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ImportRecorder: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMultisessionRandomWrite(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionRandomWrite {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsSupportedOnCurrentMediaState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetInUse)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn InUse(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.InUse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ImportRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn WriteUnitSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteUnitSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenAddress(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastWrittenAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionRandomWrite> for ::windows::core::IUnknown {
    fn from(value: IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionRandomWrite> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionRandomWrite> for ::windows::core::IUnknown {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionRandomWrite> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionRandomWrite> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionRandomWrite> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionRandomWrite> for IMultisession {
    fn from(value: IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionRandomWrite> for &'a IMultisession {
    fn from(value: &'a IMultisessionRandomWrite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionRandomWrite> for IMultisession {
    fn from(value: &IMultisessionRandomWrite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMultisessionRandomWrite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMultisessionRandomWrite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMultisessionRandomWrite {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMultisessionRandomWrite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultisessionRandomWrite").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMultisessionRandomWrite {
    type Vtable = IMultisessionRandomWrite_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca23_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMultisessionSequential(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsSupportedOnCurrentMediaState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetInUse)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn InUse(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.InUse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ImportRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn IsFirstDataSession(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsFirstDataSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextWritableAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FreeSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential> for ::windows::core::IUnknown {
    fn from(value: IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential> for ::windows::core::IUnknown {
    fn from(value: &IMultisessionSequential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionSequential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential> for IMultisession {
    fn from(value: IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential> for &'a IMultisession {
    fn from(value: &'a IMultisessionSequential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential> for IMultisession {
    fn from(value: &IMultisessionSequential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMultisessionSequential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMultisessionSequential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMultisessionSequential {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMultisessionSequential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultisessionSequential").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMultisessionSequential {
    type Vtable = IMultisessionSequential_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354151_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub IsFirstDataSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMultisessionSequential2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential2 {
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.IsSupportedOnCurrentMediaState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetInUse(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetInUse)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn InUse(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.InUse)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ImportRecorder(&self) -> ::windows::core::Result<IDiscRecorder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.ImportRecorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2>(result__)
    }
    pub unsafe fn IsFirstDataSession(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IsFirstDataSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.StartAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.LastWrittenAddressOfPreviousSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NextWritableAddress(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.NextWritableAddress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.FreeSectorsOnMedia)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn WriteUnitSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteUnitSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential2> for ::windows::core::IUnknown {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential2> for ::windows::core::IUnknown {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential2> for super::super::System::Com::IDispatch {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential2> for super::super::System::Com::IDispatch {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential2> for IMultisession {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential2> for &'a IMultisession {
    fn from(value: &'a IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential2> for IMultisession {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMultisessionSequential2> for IMultisessionSequential {
    fn from(value: IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMultisessionSequential2> for &'a IMultisessionSequential {
    fn from(value: &'a IMultisessionSequential2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMultisessionSequential2> for IMultisessionSequential {
    fn from(value: &IMultisessionSequential2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMultisessionSequential2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMultisessionSequential2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMultisessionSequential2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMultisessionSequential2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultisessionSequential2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMultisessionSequential2 {
    type Vtable = IMultisessionSequential2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca22_2204_11dd_966a_001aa01bbc58);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProgressItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItem {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn FirstBlock(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FirstBlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn LastBlock(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastBlock)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn BlockCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BlockCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItem> for ::windows::core::IUnknown {
    fn from(value: IProgressItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IProgressItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProgressItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItem> for ::windows::core::IUnknown {
    fn from(value: &IProgressItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItem> for super::super::System::Com::IDispatch {
    fn from(value: IProgressItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IProgressItem> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IProgressItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItem> for super::super::System::Com::IDispatch {
    fn from(value: &IProgressItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProgressItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IProgressItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IProgressItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IProgressItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IProgressItem {
    type Vtable = IProgressItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd5_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub FirstBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IProgressItems(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IProgressItems {
    #[doc = "*Required features: `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Ole::IEnumVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<IProgressItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProgressItem>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> ::windows::core::Result<IProgressItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProgressItemFromBlock)(::windows::core::Interface::as_raw(self), block, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProgressItem>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ProgressItemFromDescription<'a, P0>(&self, description: P0) -> ::windows::core::Result<IProgressItem>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProgressItemFromDescription)(::windows::core::Interface::as_raw(self), description.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IProgressItem>(result__)
    }
    pub unsafe fn EnumProgressItems(&self) -> ::windows::core::Result<IEnumProgressItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumProgressItems)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumProgressItems>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItems> for ::windows::core::IUnknown {
    fn from(value: IProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IProgressItems> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItems> for ::windows::core::IUnknown {
    fn from(value: &IProgressItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IProgressItems> for super::super::System::Com::IDispatch {
    fn from(value: IProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IProgressItems> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IProgressItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IProgressItems> for super::super::System::Com::IDispatch {
    fn from(value: &IProgressItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IProgressItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IProgressItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IProgressItems {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IProgressItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressItems").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IProgressItems {
    type Vtable = IProgressItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fd7_975b_59be_a960_9a2a262853a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ProgressItemFromBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ProgressItemFromBlock: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ProgressItemFromDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ProgressItemFromDescription: usize,
    pub EnumProgressItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRawCDImageCreator(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageCreator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResultImage(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateResultImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddTrack<'a, P0>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddTrack)(::windows::core::Interface::as_raw(self), datatype, data.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSpecialPregap<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AddSpecialPregap)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddSubcodeRWGenerator<'a, P0>(&self, subcode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).AddSubcodeRWGenerator)(::windows::core::Interface::as_raw(self), subcode.into().abi()).ok()
    }
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResultingImageType)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ResultingImageType(&self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResultingImageType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>(result__)
    }
    pub unsafe fn StartOfLeadout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartOfLeadout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartOfLeadoutLimit)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartOfLeadoutLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartOfLeadoutLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDisableGaplessAudio(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisableGaplessAudio)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn DisableGaplessAudio(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DisableGaplessAudio)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaCatalogNumber<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetMediaCatalogNumber)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MediaCatalogNumber(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MediaCatalogNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartingTrackNumber)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingTrackNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartingTrackNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_TrackInfo(&self, trackindex: i32) -> ::windows::core::Result<IRawCDImageTrackInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_TrackInfo)(::windows::core::Interface::as_raw(self), trackindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRawCDImageTrackInfo>(result__)
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NumberOfExistingTracks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastUsedUserSectorInImage(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastUsedUserSectorInImage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExpectedTableOfContents(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ExpectedTableOfContents)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageCreator> for ::windows::core::IUnknown {
    fn from(value: IRawCDImageCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRawCDImageCreator> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRawCDImageCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageCreator> for ::windows::core::IUnknown {
    fn from(value: &IRawCDImageCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageCreator> for super::super::System::Com::IDispatch {
    fn from(value: IRawCDImageCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRawCDImageCreator> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IRawCDImageCreator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageCreator> for super::super::System::Com::IDispatch {
    fn from(value: &IRawCDImageCreator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRawCDImageCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRawCDImageCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRawCDImageCreator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRawCDImageCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawCDImageCreator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRawCDImageCreator {
    type Vtable = IRawCDImageCreator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25983550_9d65_49ce_b335_40630d901227);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateResultImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateResultImage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut ::core::ffi::c_void, trackindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddTrack: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSpecialPregap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subcode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddSubcodeRWGenerator: usize,
    pub SetResultingImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetDisableGaplessAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub DisableGaplessAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMediaCatalogNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMediaCatalogNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MediaCatalogNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MediaCatalogNumber: usize,
    pub SetStartingTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_TrackInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_TrackInfo: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ExpectedTableOfContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExpectedTableOfContents: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRawCDImageTrackInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageTrackInfo {
    pub unsafe fn StartingLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartingLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SectorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TrackNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TrackNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorType(&self) -> ::windows::core::Result<IMAPI_CD_SECTOR_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SectorType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_CD_SECTOR_TYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ISRC(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ISRC)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetISRC<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetISRC)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    pub unsafe fn DigitalAudioCopySetting(&self) -> ::windows::core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DigitalAudioCopySetting)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>(result__)
    }
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDigitalAudioCopySetting)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn AudioHasPreemphasis(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AudioHasPreemphasis)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAudioHasPreemphasis(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioHasPreemphasis)(::windows::core::Interface::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TrackIndexes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TrackIndexes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTrackIndex)(::windows::core::Interface::as_raw(self), lbaoffset).ok()
    }
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearTrackIndex)(::windows::core::Interface::as_raw(self), lbaoffset).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageTrackInfo> for ::windows::core::IUnknown {
    fn from(value: IRawCDImageTrackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRawCDImageTrackInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRawCDImageTrackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageTrackInfo> for ::windows::core::IUnknown {
    fn from(value: &IRawCDImageTrackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRawCDImageTrackInfo> for super::super::System::Com::IDispatch {
    fn from(value: IRawCDImageTrackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRawCDImageTrackInfo> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IRawCDImageTrackInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRawCDImageTrackInfo> for super::super::System::Com::IDispatch {
    fn from(value: &IRawCDImageTrackInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRawCDImageTrackInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRawCDImageTrackInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRawCDImageTrackInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRawCDImageTrackInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawCDImageTrackInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRawCDImageTrackInfo {
    type Vtable = IRawCDImageTrackInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25983551_9d65_49ce_b335_40630d901227);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartingLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SectorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ISRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ISRC: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetISRC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetISRC: usize,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT,
    pub AudioHasPreemphasis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub TrackIndexes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TrackIndexes: usize,
    pub AddTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
pub struct IRedbookDiscMaster(::windows::core::IUnknown);
impl IRedbookDiscMaster {
    pub unsafe fn GetTotalAudioTracks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTotalAudioTracks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetTotalAudioBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTotalAudioBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetUsedAudioBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUsedAudioBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetAvailableAudioTrackBlocks(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailableAudioTrackBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn GetAudioBlockSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAudioBlockSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn CreateAudioTrack(&self, nblocks: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateAudioTrack)(::windows::core::Interface::as_raw(self), nblocks).ok()
    }
    pub unsafe fn AddAudioTrackBlocks(&self, pby: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddAudioTrackBlocks)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pby)), pby.len() as _).ok()
    }
    pub unsafe fn CloseAudioTrack(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseAudioTrack)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRedbookDiscMaster> for ::windows::core::IUnknown {
    fn from(value: IRedbookDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRedbookDiscMaster> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRedbookDiscMaster) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRedbookDiscMaster> for ::windows::core::IUnknown {
    fn from(value: &IRedbookDiscMaster) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRedbookDiscMaster {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRedbookDiscMaster {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRedbookDiscMaster {}
impl ::core::fmt::Debug for IRedbookDiscMaster {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRedbookDiscMaster").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRedbookDiscMaster {
    type Vtable = IRedbookDiscMaster_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3bc42cd_4e5c_11d3_9144_00104ba11c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRedbookDiscMaster_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetTotalAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows::core::HRESULT,
    pub GetTotalAudioBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT,
    pub GetUsedAudioBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT,
    pub GetAvailableAudioTrackBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT,
    pub GetAudioBlockSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT,
    pub CreateAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows::core::HRESULT,
    pub AddAudioTrackBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows::core::HRESULT,
    pub CloseAudioTrack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IStreamConcatenate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbread))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbwritten))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<'a, P0>(&self, pstm: P0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<'a, P0, P1>(&self, stream1: P0, stream2: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), stream1.into().abi(), stream2.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize2(&self, streams: &[::core::option::Option<super::super::System::Com::IStream>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(streams)), streams.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append<'a, P0>(&self, stream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).Append)(::windows::core::Interface::as_raw(self), stream.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Append2(&self, streams: &[::core::option::Option<super::super::System::Com::IStream>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Append2)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(streams)), streams.len() as _).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamConcatenate> for ::windows::core::IUnknown {
    fn from(value: IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamConcatenate> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamConcatenate> for ::windows::core::IUnknown {
    fn from(value: &IStreamConcatenate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamConcatenate> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamConcatenate> for &'a super::super::System::Com::ISequentialStream {
    fn from(value: &'a IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamConcatenate> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamConcatenate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamConcatenate> for super::super::System::Com::IStream {
    fn from(value: IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamConcatenate> for &'a super::super::System::Com::IStream {
    fn from(value: &'a IStreamConcatenate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamConcatenate> for super::super::System::Com::IStream {
    fn from(value: &IStreamConcatenate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IStreamConcatenate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IStreamConcatenate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IStreamConcatenate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IStreamConcatenate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamConcatenate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IStreamConcatenate {
    type Vtable = IStreamConcatenate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354146_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream1: *mut ::core::ffi::c_void, stream2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Append2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Append2: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IStreamInterleave(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbread))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbwritten))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<'a, P0>(&self, pstm: P0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(streams), ::core::mem::transmute(interleavesizes), ::core::mem::transmute(streamcount)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamInterleave> for ::windows::core::IUnknown {
    fn from(value: IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamInterleave> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamInterleave> for ::windows::core::IUnknown {
    fn from(value: &IStreamInterleave) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamInterleave> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamInterleave> for &'a super::super::System::Com::ISequentialStream {
    fn from(value: &'a IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamInterleave> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamInterleave) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamInterleave> for super::super::System::Com::IStream {
    fn from(value: IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamInterleave> for &'a super::super::System::Com::IStream {
    fn from(value: &'a IStreamInterleave) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamInterleave> for super::super::System::Com::IStream {
    fn from(value: &IStreamInterleave) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IStreamInterleave {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IStreamInterleave {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IStreamInterleave {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IStreamInterleave {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamInterleave").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IStreamInterleave {
    type Vtable = IStreamInterleave_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354147_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IStreamPseudoRandomBased(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbread))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pv), cb, ::core::mem::transmute(pcbwritten))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<'a, P0>(&self, pstm: P0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatstg), grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::System::Com::IStream>(result__)
    }
    pub unsafe fn SetSeed(&self, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSeed)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Seed(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Seed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn put_ExtendedSeed(&self, values: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).put_ExtendedSeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(values)), values.len() as _).ok()
    }
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_ExtendedSeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(values), ::core::mem::transmute(ecount)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamPseudoRandomBased> for ::windows::core::IUnknown {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamPseudoRandomBased> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamPseudoRandomBased> for ::windows::core::IUnknown {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamPseudoRandomBased> for super::super::System::Com::ISequentialStream {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamPseudoRandomBased> for &'a super::super::System::Com::ISequentialStream {
    fn from(value: &'a IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamPseudoRandomBased> for super::super::System::Com::ISequentialStream {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStreamPseudoRandomBased> for super::super::System::Com::IStream {
    fn from(value: IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IStreamPseudoRandomBased> for &'a super::super::System::Com::IStream {
    fn from(value: &'a IStreamPseudoRandomBased) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStreamPseudoRandomBased> for super::super::System::Com::IStream {
    fn from(value: &IStreamPseudoRandomBased) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IStreamPseudoRandomBased {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IStreamPseudoRandomBased {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IStreamPseudoRandomBased {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IStreamPseudoRandomBased {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStreamPseudoRandomBased").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IStreamPseudoRandomBased {
    type Vtable = IStreamPseudoRandomBased_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354145_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Seed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows::core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWriteEngine2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WriteSection<'a, P0>(&self, data: P0, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IStream>>,
    {
        (::windows::core::Interface::vtable(self).WriteSection)(::windows::core::Interface::as_raw(self), data.into().abi(), startingblockaddress, numberofblocks).ok()
    }
    pub unsafe fn CancelWrite(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelWrite)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetRecorder<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDiscRecorder2Ex>>,
    {
        (::windows::core::Interface::vtable(self).SetRecorder)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    pub unsafe fn Recorder(&self) -> ::windows::core::Result<IDiscRecorder2Ex> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Recorder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDiscRecorder2Ex>(result__)
    }
    pub unsafe fn SetUseStreamingWrite12(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseStreamingWrite12)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn UseStreamingWrite12(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UseStreamingWrite12)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartingSectorsPerSecond)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn StartingSectorsPerSecond(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartingSectorsPerSecond)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEndingSectorsPerSecond)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn EndingSectorsPerSecond(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EndingSectorsPerSecond)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBytesPerSector)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn BytesPerSector(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).BytesPerSector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn WriteInProgress(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteInProgress)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2> for ::windows::core::IUnknown {
    fn from(value: IWriteEngine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteEngine2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWriteEngine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2> for ::windows::core::IUnknown {
    fn from(value: &IWriteEngine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2> for super::super::System::Com::IDispatch {
    fn from(value: IWriteEngine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteEngine2> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IWriteEngine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteEngine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWriteEngine2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWriteEngine2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWriteEngine2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWriteEngine2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWriteEngine2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWriteEngine2 {
    type Vtable = IWriteEngine2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354135_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub WriteSection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Recorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetUseStreamingWrite12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub UseStreamingWrite12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub WriteInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWriteEngine2EventArgs(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2EventArgs {
    pub unsafe fn StartLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SectorCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SectorCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastReadLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastReadLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LastWrittenLba(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastWrittenLba)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn TotalSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TotalSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UsedSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).UsedSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn FreeSystemBuffer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FreeSystemBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2EventArgs> for ::windows::core::IUnknown {
    fn from(value: IWriteEngine2EventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteEngine2EventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWriteEngine2EventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2EventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWriteEngine2EventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteEngine2EventArgs> for super::super::System::Com::IDispatch {
    fn from(value: IWriteEngine2EventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteEngine2EventArgs> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IWriteEngine2EventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteEngine2EventArgs> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteEngine2EventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWriteEngine2EventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWriteEngine2EventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWriteEngine2EventArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWriteEngine2EventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWriteEngine2EventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWriteEngine2EventArgs {
    type Vtable = IWriteEngine2EventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354136_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWriteSpeedDescriptor(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWriteSpeedDescriptor {
    pub unsafe fn MediaType(&self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MediaType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMAPI_MEDIA_PHYSICAL_TYPE>(result__)
    }
    pub unsafe fn RotationTypeIsPureCAV(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RotationTypeIsPureCAV)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn WriteSpeed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).WriteSpeed)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteSpeedDescriptor> for ::windows::core::IUnknown {
    fn from(value: IWriteSpeedDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteSpeedDescriptor> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWriteSpeedDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteSpeedDescriptor> for ::windows::core::IUnknown {
    fn from(value: &IWriteSpeedDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWriteSpeedDescriptor> for super::super::System::Com::IDispatch {
    fn from(value: IWriteSpeedDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWriteSpeedDescriptor> for &'a super::super::System::Com::IDispatch {
    fn from(value: &'a IWriteSpeedDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWriteSpeedDescriptor> for super::super::System::Com::IDispatch {
    fn from(value: &IWriteSpeedDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWriteSpeedDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWriteSpeedDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWriteSpeedDescriptor {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWriteSpeedDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWriteSpeedDescriptor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWriteSpeedDescriptor {
    type Vtable = IWriteSpeedDescriptor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354144_7f64_5b0f_8f00_5d77afbe261e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT,
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT,
    pub WriteSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIA_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_BLANK: MEDIA_FLAGS = MEDIA_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_RW: MEDIA_FLAGS = MEDIA_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_WRITABLE: MEDIA_FLAGS = MEDIA_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_FORMAT_UNUSABLE_BY_IMAPI: MEDIA_FLAGS = MEDIA_FLAGS(8i32);
impl ::core::marker::Copy for MEDIA_FLAGS {}
impl ::core::clone::Clone for MEDIA_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEDIA_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEDIA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEDIA_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CDDA_CDROM: MEDIA_TYPES = MEDIA_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_ROM_XA: MEDIA_TYPES = MEDIA_TYPES(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_I: MEDIA_TYPES = MEDIA_TYPES(3i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_EXTRA: MEDIA_TYPES = MEDIA_TYPES(4i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_CD_OTHER: MEDIA_TYPES = MEDIA_TYPES(5i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MEDIA_SPECIAL: MEDIA_TYPES = MEDIA_TYPES(6i32);
impl ::core::marker::Copy for MEDIA_TYPES {}
impl ::core::clone::Clone for MEDIA_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEDIA_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEDIA_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEDIA_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MPV_INBOUND_CUTOFF_EXCEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MPV_WRITE_CONTENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_DELIVERY_REPORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_NONDELIVERY_REPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_REPLICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_MSGCLASS_SYSTEM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_ABANDON_DELIVERY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_ABORT_DELIVERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_BAD_MAIL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_CATEGORIZED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_RETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_SUBMITTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const MP_STATUS_SUCCESS: u32 = 0u32;
pub const MSDiscMasterObj: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520cca63_51a5_11d3_9144_00104ba11c5e);
pub const MSDiscRecorderObj: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520cca61_51a5_11d3_9144_00104ba11c5e);
pub const MSEnumDiscRecordersObj: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a03567a_63cb_4ba8_baf6_52119816d1ef);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
#[cfg(feature = "Win32_System_AddressBook")]
pub type MSGCALLRELEASE = ::core::option::Option<unsafe extern "system" fn(ulcallerdata: u32, lpmessage: ::core::option::Option<super::super::System::AddressBook::IMessage>)>;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[inline]
pub unsafe fn MapStorageSCode(stgscode: i32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MapStorageSCode(stgscode: i32) -> i32;
    }
    MapStorageSCode(stgscode)
}
pub const MsftDiscFormat2Data: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735412a_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2Erase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735412b_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2RawCD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354128_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2TrackAtOnce: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354129_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscMaster2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735412e_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscRecorder2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735412d_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftFileSystemImage: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fc5_975b_59be_a960_9a2a262853a5);
pub const MsftIsoImageManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceee3b62_8f56_4056_869b_ef16917e3efc);
pub const MsftMultisessionRandomWrite: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb507ca24_2204_11dd_966a_001aa01bbc58);
pub const MsftMultisessionSequential: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354122_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftRawCDImageCreator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25983561_9d65_49ce_b335_40630d901227);
pub const MsftStreamConcatenate: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354125_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamInterleave: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354124_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamPrng001: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354126_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamZero: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354127_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteEngine2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2735412c_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteSpeedDescriptor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27354123_7f64_5b0f_8f00_5d77afbe261e);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_MODERATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const NMP_PROCESS_POST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OpenIMsgOnIStg<'a, P0, P1>(lpmsgsess: *mut _MSGSESS, lpallocatebuffer: super::super::System::AddressBook::LPALLOCATEBUFFER, lpallocatemore: super::super::System::AddressBook::LPALLOCATEMORE, lpfreebuffer: super::super::System::AddressBook::LPFREEBUFFER, lpmalloc: P0, lpmapisup: *mut ::core::ffi::c_void, lpstg: P1, lpfmsgcallrelease: *mut MSGCALLRELEASE, ulcallerdata: u32, ulflags: u32, lppmsg: *mut ::core::option::Option<super::super::System::AddressBook::IMessage>) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IMalloc>>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::StructuredStorage::IStorage>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenIMsgOnIStg(lpmsgsess: *mut _MSGSESS, lpallocatebuffer: *mut ::core::ffi::c_void, lpallocatemore: *mut ::core::ffi::c_void, lpfreebuffer: *mut ::core::ffi::c_void, lpmalloc: *mut ::core::ffi::c_void, lpmapisup: *mut ::core::ffi::c_void, lpstg: *mut ::core::ffi::c_void, lpfmsgcallrelease: *mut *mut ::core::ffi::c_void, ulcallerdata: u32, ulflags: u32, lppmsg: *mut *mut ::core::ffi::c_void) -> i32;
    }
    OpenIMsgOnIStg(::core::mem::transmute(lpmsgsess), ::core::mem::transmute(lpallocatebuffer), ::core::mem::transmute(lpallocatemore), ::core::mem::transmute(lpfreebuffer), lpmalloc.into().abi(), ::core::mem::transmute(lpmapisup), lpstg.into().abi(), ::core::mem::transmute(lpfmsgcallrelease), ulcallerdata, ulflags, ::core::mem::transmute(lppmsg))
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OpenIMsgSession<'a, P0>(lpmalloc: P0, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::IMalloc>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenIMsgSession(lpmalloc: *mut ::core::ffi::c_void, ulflags: u32, lppmsgsess: *mut *mut _MSGSESS) -> i32;
    }
    OpenIMsgSession(lpmalloc.into().abi(), ulflags, ::core::mem::transmute(lppmsgsess))
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PlatformId(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformX86: PlatformId = PlatformId(0i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformPowerPC: PlatformId = PlatformId(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformMac: PlatformId = PlatformId(2i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const PlatformEFI: PlatformId = PlatformId(239i32);
impl ::core::marker::Copy for PlatformId {}
impl ::core::clone::Clone for PlatformId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlatformId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PlatformId {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlatformId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlatformId").field(&self.0).finish()
    }
}
pub const ProgressItem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fcb_975b_59be_a960_9a2a262853a5);
pub const ProgressItems: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c941fc9_975b_59be_a960_9a2a262853a5);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RECORDER_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_CDR: RECORDER_TYPES = RECORDER_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RECORDER_CDRW: RECORDER_TYPES = RECORDER_TYPES(2i32);
impl ::core::marker::Copy for RECORDER_TYPES {}
impl ::core::clone::Clone for RECORDER_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RECORDER_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RECORDER_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RECORDER_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECORDER_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DELIVERED: u32 = 272u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_HANDLED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_DELAY: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_FAILURE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_NEVER: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_NOTIFY_SUCCESS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_DELAYED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_DELIVERED: u32 = 131136u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_EXPANDED: u32 = 32832u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_NDR: u32 = 1104u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_DSN_SENT_RELAYED: u32 = 65600u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ENPANDED: u32 = 8208u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_CAT: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_MTA: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_ERROR_CONTEXT_STORE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_EXPANDED: u32 = 8208u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_FAILED: u32 = 2096u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_GENERAL_FAILURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_HANDLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_RECIP_FLAGS_RESERVED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_REMOTE_MTA_NO_DSN: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_UNRESOLVED: u32 = 4144u32;
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const RP_VOLATILE_FLAGS_MASK: u32 = 4026531840u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct SPropAttrArray {
    pub cValues: u32,
    pub aPropAttr: [u32; 1],
}
impl ::core::marker::Copy for SPropAttrArray {}
impl ::core::clone::Clone for SPropAttrArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPropAttrArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPropAttrArray").field("cValues", &self.cValues).field("aPropAttr", &self.aPropAttr).finish()
    }
}
unsafe impl ::windows::core::Abi for SPropAttrArray {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SPropAttrArray {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SPropAttrArray>()) == 0 }
    }
}
impl ::core::cmp::Eq for SPropAttrArray {}
impl ::core::default::Default for SPropAttrArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub const SZ_PROGID_SMTPCAT: &str = "Smtp.Cat";
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_AddressBook\"`*"]
#[cfg(feature = "Win32_System_AddressBook")]
#[inline]
pub unsafe fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetAttribIMsgOnIStg(lpobject: *mut ::core::ffi::c_void, lpproptags: *mut super::super::System::AddressBook::SPropTagArray, lppropattrs: *mut SPropAttrArray, lpppropproblems: *mut *mut super::super::System::AddressBook::SPropProblemArray) -> ::windows::core::HRESULT;
    }
    SetAttribIMsgOnIStg(::core::mem::transmute(lpobject), ::core::mem::transmute(lpproptags), ::core::mem::transmute(lppropattrs), ::core::mem::transmute(lpppropproblems)).ok()
}
#[repr(C)]
pub struct _MSGSESS(pub u8);
pub const tagIMMPID_CPV_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2a76b2a_e52d_11d1_aa64_00c04fa35b82);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`*"]
pub struct tagIMMPID_GUIDLIST_ITEM {
    pub pguid: *const ::windows::core::GUID,
    pub dwStart: u32,
    pub dwLast: u32,
}
impl ::core::marker::Copy for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::clone::Clone for tagIMMPID_GUIDLIST_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tagIMMPID_GUIDLIST_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tagIMMPID_GUIDLIST_ITEM").field("pguid", &self.pguid).field("dwStart", &self.dwStart).field("dwLast", &self.dwLast).finish()
    }
}
unsafe impl ::windows::core::Abi for tagIMMPID_GUIDLIST_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tagIMMPID_GUIDLIST_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tagIMMPID_GUIDLIST_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for tagIMMPID_GUIDLIST_ITEM {}
impl ::core::default::Default for tagIMMPID_GUIDLIST_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const tagIMMPID_MPV_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbe69706_c9bd_11d1_9ff2_00c04fa37348);
pub const tagIMMPID_MP_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13384cf0_b3c4_11d1_aa92_00aa006bc80b);
pub const tagIMMPID_NMP_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7433a9aa_20e2_11d2_94d6_00c04fa379f1);
pub const tagIMMPID_RPV_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79e82049_d320_11d1_9ff4_00c04fa37348);
pub const tagIMMPID_RP_STRUCT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79e82048_d320_11d1_9ff4_00c04fa37348);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
