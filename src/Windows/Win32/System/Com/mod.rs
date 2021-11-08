#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ADVF(pub i32);
pub const ADVF_NODATA: ADVF = ADVF(1i32);
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::core::convert::From<i32> for ADVF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ADVF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APTTYPE(pub i32);
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::core::convert::From<i32> for APTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APTTYPEQUALIFIER(pub i32);
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
impl ::core::convert::From<i32> for APTTYPEQUALIFIER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APTTYPEQUALIFIER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationType(pub i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::core::convert::From<i32> for ApplicationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ApplicationType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIAdviseSink(pub ::windows::runtime::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnRename(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(336, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<AsyncIAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIAdviseSink2(pub ::windows::runtime::IUnknown);
impl AsyncIAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnRename(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(337, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<AsyncIAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AsyncIAdviseSink> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, AsyncIAdviseSink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AsyncIAdviseSink> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, AsyncIAdviseSink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIMultiQI(pub ::windows::runtime::IUnknown);
impl AsyncIMultiQI {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(917536, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<AsyncIMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmqis: u32, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeByte(pub ::windows::runtime::IUnknown);
impl AsyncIPipeByte {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const u8, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305547, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<AsyncIPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crequest: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut u8, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const u8, csent: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeDouble(pub ::windows::runtime::IUnknown);
impl AsyncIPipeDouble {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const f64, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305551, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<AsyncIPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crequest: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut f64, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const f64, csent: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeLong(pub ::windows::runtime::IUnknown);
impl AsyncIPipeLong {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const i32, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305549, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<AsyncIPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crequest: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut i32, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const i32, csent: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIUnknown(pub ::windows::runtime::IUnknown);
impl AsyncIUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_AddRef(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Release(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(917504, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<AsyncIUnknown> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AsyncIUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: super::super::Foundation::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: super::super::Foundation::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: ::windows::runtime::GUID,
    pub pUnk: ::core::option::Option<::windows::runtime::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for BINDINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for BINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDINFOF(pub i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::core::convert::From<i32> for BINDINFOF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BINDINFOF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BIND_FLAGS(pub i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::core::convert::From<i32> for BIND_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BIND_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl BIND_OPTS {}
impl ::core::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.grfFlags == other.grfFlags && self.grfMode == other.grfMode && self.dwTickCountDeadline == other.dwTickCountDeadline
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
unsafe impl ::windows::runtime::Abi for BIND_OPTS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BIND_OPTS2").field("__AnonymousBase_objidl_L9017_C36", &self.__AnonymousBase_objidl_L9017_C36).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9017_C36 == other.__AnonymousBase_objidl_L9017_C36 && self.dwTrackFlags == other.dwTrackFlags && self.dwClassContext == other.dwClassContext && self.locale == other.locale && self.pServerInfo == other.pServerInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BIND_OPTS2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BIND_OPTS3").field("__AnonymousBase_objidl_L9041_C36", &self.__AnonymousBase_objidl_L9041_C36).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9041_C36 == other.__AnonymousBase_objidl_L9041_C36 && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl BLOB {}
impl ::core::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pBlobData == other.pBlobData
    }
}
impl ::core::cmp::Eq for BLOB {}
unsafe impl ::windows::runtime::Abi for BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl BYTE_BLOB {}
impl ::core::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
unsafe impl ::windows::runtime::Abi for BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl BYTE_SIZEDARR {}
impl ::core::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn BindMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(pmk: Param0, grfopt: u32, iidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindMoniker(pmk: ::windows::runtime::RawPtr, grfopt: u32, iidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        BindMoniker(pmk.into_param().abi(), ::core::mem::transmute(grfopt), ::core::mem::transmute(iidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLTYPE(pub i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::core::convert::From<i32> for CALLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CATEGORYINFO {
    pub catid: ::windows::runtime::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl CATEGORYINFO {}
impl ::core::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.catid == other.catid && self.lcid == other.lcid && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
unsafe impl ::windows::runtime::Abi for CATEGORYINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLSCTX(pub u32);
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
impl ::core::convert::From<u32> for CLSCTX {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLSCTX {
    type Abi = Self;
}
impl ::core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl COAUTHIDENTITY {}
impl ::core::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
unsafe impl ::windows::runtime::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: super::super::Foundation::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COAUTHINFO")
            .field("dwAuthnSvc", &self.dwAuthnSvc)
            .field("dwAuthzSvc", &self.dwAuthzSvc)
            .field("pwszServerPrincName", &self.pwszServerPrincName)
            .field("dwAuthnLevel", &self.dwAuthnLevel)
            .field("dwImpersonationLevel", &self.dwImpersonationLevel)
            .field("pAuthIdentityData", &self.pAuthIdentityData)
            .field("dwCapabilities", &self.dwCapabilities)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pwszServerPrincName == other.pwszServerPrincName && self.dwAuthnLevel == other.dwAuthnLevel && self.dwImpersonationLevel == other.dwImpersonationLevel && self.pAuthIdentityData == other.pAuthIdentityData && self.dwCapabilities == other.dwCapabilities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COAUTHINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COINIT(pub u32);
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::core::convert::From<u32> for COINIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COINIT {
    type Abi = Self;
}
impl ::core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COINITBASE(pub i32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::convert::From<i32> for COINITBASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COINITBASE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMSD(pub i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::core::convert::From<i32> for COMSD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMSD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CONNECTDATA {
    pub pUnk: ::core::option::Option<::windows::runtime::IUnknown>,
    pub dwCookie: u32,
}
impl CONNECTDATA {}
impl ::core::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
unsafe impl ::windows::runtime::Abi for CONNECTDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1 && self.pwszName == other.pwszName && self.pAuthInfo == other.pAuthInfo && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COSERVERINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COWAIT_FLAGS(pub i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::core::convert::From<i32> for COWAIT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COWAIT_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl ::core::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CO_DEVICE_CATALOG_COOKIE {}
unsafe impl ::windows::runtime::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
impl ::core::convert::From<i32> for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl ::core::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CO_MTA_USAGE_COOKIE {}
unsafe impl ::windows::runtime::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl CSPLATFORM {}
impl ::core::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwPlatformId == other.dwPlatformId && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo && self.dwProcessorArch == other.dwProcessorArch
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
unsafe impl ::windows::runtime::Abi for CSPLATFORM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CWMO_FLAGS(pub i32);
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::core::convert::From<i32> for CWMO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CWMO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl CY {}
impl ::core::default::Default for CY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for CY {}
unsafe impl ::windows::runtime::Abi for CY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl CY_0 {}
impl ::core::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo == other.Lo && self.Hi == other.Hi
    }
}
impl ::core::cmp::Eq for CY_0 {}
unsafe impl ::windows::runtime::Abi for CY_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAddRefServerProcess() -> u32;
        }
        ::core::mem::transmute(CoAddRefServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, lpvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: ::windows::runtime::RawPtr, lpvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::core::mem::transmute(lpvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        CoAllowUnmarshalerCLSID(::core::mem::transmute(clsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoBuildVersion() -> u32;
        }
        ::core::mem::transmute(CoBuildVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::runtime::HRESULT;
        }
        CoCancelCall(::core::mem::transmute(dwthreadid), ::core::mem::transmute(ultimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCopyProxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pproxy: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCopyProxy(pproxy: ::windows::runtime::RawPtr, ppcopy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoCopyProxy(pproxy.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkouter: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateFreeThreadedMarshaler(punkouter: ::windows::runtime::RawPtr, ppunkmarshal: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateGuid(pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoCreateGuid(&mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCreateInstance<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(rclsid: *const ::windows::runtime::GUID, punkouter: Param1, dwclscontext: CLSCTX) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstance(rclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclscontext: CLSCTX, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclscontext), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoCreateInstanceEx(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pserverinfo), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoCreateInstanceFromApp(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(reserved), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoDecrementMTAUsage<'a, Param0: ::windows::runtime::IntoParam<'a, CO_MTA_USAGE_COOKIE>>(cookie: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows::runtime::HRESULT;
        }
        CoDecrementMTAUsage(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoDisableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectContext(dwtimeout: u32) -> ::windows::runtime::HRESULT;
        }
        CoDisconnectContext(::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoDisconnectObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, dwreserved: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectObject(punk: ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        CoDisconnectObject(punk.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoDosDateTimeToFileTime(::core::mem::transmute(ndosdate), ::core::mem::transmute(ndostime), ::core::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoEnableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoFileTimeNow(&mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoFileTimeToDosDateTime(::core::mem::transmute(lpfiletime), ::core::mem::transmute(lpdosdate), ::core::mem::transmute(lpdostime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeAllLibraries();
        }
        ::core::mem::transmute(CoFreeAllLibraries())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
        }
        ::core::mem::transmute(CoFreeLibrary(hinst.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibraries();
        }
        ::core::mem::transmute(CoFreeUnusedLibraries())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
        }
        ::core::mem::transmute(CoFreeUnusedLibrariesEx(::core::mem::transmute(dwunloaddelay), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::runtime::HRESULT;
        }
        CoGetApartmentType(::core::mem::transmute(papttype), ::core::mem::transmute(paptqualifier)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows::runtime::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallContext(riid: *const ::windows::runtime::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetCallContext(::core::mem::transmute(riid), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetCallerTID(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetCancelObject(::core::mem::transmute(dwthreadid), ::core::mem::transmute(iid), ::core::mem::transmute(ppunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows::runtime::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObject(rclsid: *const ::windows::runtime::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetClassObject(::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(pvreserved), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows::runtime::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetContextToken(ptoken: *mut usize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetContextToken(&mut result__).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetCurrentLogicalThreadId(&mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentProcess() -> u32;
        }
        ::core::mem::transmute(CoGetCurrentProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows::runtime::Result<IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMalloc as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetMalloc(::core::mem::transmute(dwmemcontext), &mut result__).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::core::mem::transmute(pbindoptions), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObjectContext(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetObjectContext(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetPSClsid(riid: *const ::windows::runtime::GUID, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetPSClsid(::core::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::HRESULT;
        }
        CoGetSystemSecurityPermissions(::core::mem::transmute(comsdtype), ::core::mem::transmute(ppsd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetTreatAsClass(clsidold: *const ::windows::runtime::GUID, pclsidnew: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoGetTreatAsClass(::core::mem::transmute(clsidold), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoImpersonateClient() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoImpersonateClient() -> ::windows::runtime::HRESULT;
        }
        CoImpersonateClient().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows::runtime::Result<CO_MTA_USAGE_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CO_MTA_USAGE_COOKIE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoIncrementMTAUsage(&mut result__).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoInitialize(::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::runtime::HRESULT;
        }
        CoInitializeEx(::core::mem::transmute(pvreserved), ::core::mem::transmute(dwcoinit)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoInitializeSecurity(
            ::core::mem::transmute(psecdesc),
            ::core::mem::transmute(cauthsvc),
            ::core::mem::transmute(asauthsvc),
            ::core::mem::transmute(preserved1),
            ::core::mem::transmute(dwauthnlevel),
            ::core::mem::transmute(dwimplevel),
            ::core::mem::transmute(pauthlist),
            ::core::mem::transmute(dwcapabilities),
            ::core::mem::transmute(preserved3),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows::runtime::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszmachinename: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        CoInvalidateRemoteMachineBindings(pszmachinename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsOle1Class(rclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsOle1Class(::core::mem::transmute(rclsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(CoLoadLibrary(lpszlibname.into_param().abi(), bautofree.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLockObjectExternal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(punk: Param0, flock: Param1, flastunlockreleases: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLockObjectExternal(punk: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        CoLockObjectExternal(punk.into_param().abi(), flock.into_param().abi(), flastunlockreleases.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::runtime::HRESULT;
        }
        CoQueryAuthenticationServices(::core::mem::transmute(pcauthsvc), ::core::mem::transmute(asauthsvc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CoQueryClientBlanket(::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: ::windows::runtime::RawPtr, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(pwauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterActivationFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IActivationFilter>>(pactivationfilter: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterActivationFilter(pactivationfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoRegisterActivationFilter(pactivationfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterChannelHook<'a, Param1: ::windows::runtime::IntoParam<'a, IChannelHook>>(extensionuuid: *const ::windows::runtime::GUID, pchannelhook: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterChannelHook(extensionuuid: *const ::windows::runtime::GUID, pchannelhook: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoRegisterChannelHook(::core::mem::transmute(extensionuuid), pchannelhook.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterClassObject<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(rclsid: *const ::windows::runtime::GUID, punk: Param1, dwclscontext: CLSCTX, flags: u32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterClassObject(rclsid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterClassObject(::core::mem::transmute(rclsid), punk.into_param().abi(), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(flags), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinstanceid: Param0) -> ::windows::runtime::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CO_DEVICE_CATALOG_COOKIE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.into_param().abi(), &mut result__).from_abi::<CO_DEVICE_CATALOG_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<'a, Param0: ::windows::runtime::IntoParam<'a, IInitializeSpy>>(pspy: Param0) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterInitializeSpy(pspy: ::windows::runtime::RawPtr, pulicookie: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterMallocSpy<'a, Param0: ::windows::runtime::IntoParam<'a, IMallocSpy>>(pmallocspy: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMallocSpy(pmallocspy: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoRegisterMallocSpy(pmallocspy.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const ::windows::runtime::GUID, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterPSClsid(riid: *const ::windows::runtime::GUID, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        CoRegisterPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRegisterSurrogate<'a, Param0: ::windows::runtime::IntoParam<'a, ISurrogate>>(psurrogate: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterSurrogate(psurrogate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoRegisterSurrogate(psurrogate.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseServerProcess() -> u32;
        }
        ::core::mem::transmute(CoReleaseServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoResumeClassObjects() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoResumeClassObjects() -> ::windows::runtime::HRESULT;
        }
        CoResumeClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRevertToSelf() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevertToSelf() -> ::windows::runtime::HRESULT;
        }
        CoRevertToSelf().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeClassObject(dwregister: u32) -> ::windows::runtime::HRESULT;
        }
        CoRevokeClassObject(::core::mem::transmute(dwregister)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<'a, Param0: ::windows::runtime::IntoParam<'a, CO_DEVICE_CATALOG_COOKIE>>(cookie: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows::runtime::HRESULT;
        }
        CoRevokeDeviceCatalog(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::runtime::HRESULT;
        }
        CoRevokeInitializeSpy(::core::mem::transmute(ulicookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeMallocSpy() -> ::windows::runtime::HRESULT;
        }
        CoRevokeMallocSpy().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoSetCancelObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetCancelObject(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoSetCancelObject(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: ::windows::runtime::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoSuspendClassObjects() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSuspendClassObjects() -> ::windows::runtime::HRESULT;
        }
        CoSuspendClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoSwitchCallContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pnewobject: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSwitchCallContext(pnewobject: ::windows::runtime::RawPtr, ppoldobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemAlloc(cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemAlloc(::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(CoTaskMemFree(::core::mem::transmute(pv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemRealloc(pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CoTaskMemRealloc(::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTestCancel() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTestCancel() -> ::windows::runtime::HRESULT;
        }
        CoTestCancel().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTreatAsClass(clsidold: *const ::windows::runtime::GUID, clsidnew: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        CoTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(clsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUninitialize();
        }
        ::core::mem::transmute(CoUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoWaitForMultipleHandles(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoWaitForMultipleObjects(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ComCallData {}
impl ::core::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        self.dwDispid == other.dwDispid && self.dwReserved == other.dwReserved && self.pUserDefined == other.pUserDefined
    }
}
impl ::core::cmp::Eq for ComCallData {}
unsafe impl ::windows::runtime::Abi for ComCallData {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAntiMoniker(ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateAntiMoniker(&mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows::runtime::Result<IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IBindCtx as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateBindCtx(::core::mem::transmute(reserved), &mut result__).from_abi::<IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateClassMoniker(rclsid: *const ::windows::runtime::GUID, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateClassMoniker(::core::mem::transmute(rclsid), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows::runtime::Result<IDataAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataAdviseHolder(ppdaholder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IDataAdviseHolder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkouter: Param0, rclsid: *const ::windows::runtime::GUID, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszpathname: Param0) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateFileMoniker(lpszpathname.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateGenericComposite<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(pmkfirst: Param0, pmkrest: Param1) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGenericComposite(pmkfirst: ::windows::runtime::RawPtr, pmkrest: ::windows::runtime::RawPtr, ppmkcomposite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateGenericComposite(pmkfirst.into_param().abi(), pmkrest.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateIUriBuilder<'a, Param0: ::windows::runtime::IntoParam<'a, IUri>>(piuri: Param0, dwflags: u32, dwreserved: usize) -> ::windows::runtime::Result<IUriBuilder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIUriBuilder(piuri: ::windows::runtime::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUriBuilder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateItemMoniker(lpszdelim.into_param().abi(), lpszitem.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateObjrefMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateObjrefMoniker(punk: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateObjrefMoniker(punk.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreatePointerMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePointerMoniker(punk: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows::runtime::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: ::windows::runtime::RawPtr, ppibsc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IBindStatusCallback as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), &mut result__).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::core::mem::transmute(dwencodingflags), ::core::mem::transmute(dwcodepage), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DATADIR(pub i32);
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::core::convert::From<i32> for DATADIR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DATADIR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOM_CALL_STATE(pub i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::core::convert::From<i32> for DCOM_CALL_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOM_CALL_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DVASPECT(pub i32);
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1i32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2i32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4i32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8i32);
impl ::core::convert::From<i32> for DVASPECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DVASPECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl DVTARGETDEVICE {}
impl ::core::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DVTARGETDEVICE")
            .field("tdSize", &self.tdSize)
            .field("tdDriverNameOffset", &self.tdDriverNameOffset)
            .field("tdDeviceNameOffset", &self.tdDeviceNameOffset)
            .field("tdPortNameOffset", &self.tdPortNameOffset)
            .field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset)
            .field("tdData", &self.tdData)
            .finish()
    }
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.tdSize == other.tdSize && self.tdDriverNameOffset == other.tdDriverNameOffset && self.tdDeviceNameOffset == other.tdDeviceNameOffset && self.tdPortNameOffset == other.tdPortNameOffset && self.tdExtDevmodeOffset == other.tdExtDevmodeOffset && self.tdData == other.tdData
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
unsafe impl ::windows::runtime::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl DWORD_BLOB {}
impl ::core::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.alData == other.alData
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
unsafe impl ::windows::runtime::Abi for DWORD_BLOB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        DcomChannelSetHResult(::core::mem::transmute(pvreserved), ::core::mem::transmute(pulreserved), ::core::mem::transmute(appshr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1i32);
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(32i32);
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(64i32);
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(128i32);
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(256i32);
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2i32);
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4i32);
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(512i32);
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
impl ::core::convert::From<i32> for EOLE_AUTHENTICATION_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EXTCONN(pub i32);
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::core::convert::From<i32> for EXTCONN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXTCONN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl FLAGGED_BYTE_BLOB {}
impl ::core::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
unsafe impl ::windows::runtime::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl FLAGGED_WORD_BLOB {}
impl ::core::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
unsafe impl ::windows::runtime::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for FLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl FORMATETC {}
impl ::core::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for FORMATETC {}
unsafe impl ::windows::runtime::Abi for FORMATETC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::core::convert::From<i32> for GLOBALOPT_EH_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
impl ::core::convert::From<i32> for GLOBALOPT_PROPERTIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(2i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(4i32);
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(128i32);
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
impl ::core::convert::From<i32> for GLOBALOPT_RO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
impl ::core::convert::From<i32> for GLOBALOPT_RPCTP_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::core::convert::From<i32> for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        GetClassFile(szfilename.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows::runtime::Result<IRunningObjectTable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRunningObjectTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        GetRunningObjectTable(::core::mem::transmute(reserved), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl HYPER_SIZEDARR {}
impl ::core::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivationFilter(pub ::windows::runtime::IUnknown);
impl IActivationFilter {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwactivationtype), ::core::mem::transmute(rclsid), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IActivationFilter {
    type Vtable = IActivationFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(23, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IActivationFilter> for ::windows::runtime::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivationFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActivationFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwactivationtype: u32, rclsid: *const ::windows::runtime::GUID, preplacementclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAddrExclusionControl(pub ::windows::runtime::IUnknown);
impl IAddrExclusionControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::runtime::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppenumerator)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, penumerator: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), penumerator.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(328, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IAddrExclusionControl> for ::windows::runtime::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAddrExclusionControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAddrExclusionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumerator: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAddrTrackingControl(pub ::windows::runtime::IUnknown);
impl IAddrTrackingControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(327, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IAddrTrackingControl> for ::windows::runtime::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAddrTrackingControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAddrTrackingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAdviseSink(pub ::windows::runtime::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IAdviseSink {
    type Vtable = IAdviseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(271, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAdviseSink2(pub ::windows::runtime::IUnknown);
impl IAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnSave(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnClose(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(293, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAdviseSink2> for IAdviseSink {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for IAdviseSink {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdviseSink> for IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdviseSink> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdviseSink> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdviseSink> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAgileObject(pub ::windows::runtime::IUnknown);
impl IAgileObject {}
unsafe impl ::windows::runtime::Interface for IAgileObject {
    type Vtable = IAgileObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2498374548, 59852, 18912, [192, 255, 238, 100, 202, 143, 91, 144]);
}
impl ::core::convert::From<IAgileObject> for ::windows::runtime::IUnknown {
    fn from(value: IAgileObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows::runtime::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAgileObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAsyncManager(pub ::windows::runtime::IUnknown);
impl IAsyncManager {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CompleteCall(&self, result: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(result)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAsyncManager {
    type Vtable = IAsyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(42, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IAsyncManager> for ::windows::runtime::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncManager> for ::windows::runtime::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAsyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulstateflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAsyncRpcChannelBuffer(pub ::windows::runtime::IUnknown);
impl IAsyncRpcChannelBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Send<'a, Param1: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), psync.into_param().abi(), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768412598, 15412, 4561, [156, 153, 0, 192, 79, 185, 152, 170]);
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::runtime::RawPtr, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindCtx(pub ::windows::runtime::IUnknown);
impl IBindCtx {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::runtime::Result<IRunningObjectTable> {
        let mut result__: <IRunningObjectTable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumObjectParam(&self) -> ::windows::runtime::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszkey.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindCtx {
    type Vtable = IBindCtx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(14, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IBindCtx> for ::windows::runtime::IUnknown {
    fn from(value: IBindCtx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindCtx> for ::windows::runtime::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBindCtx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbindopts: *const BIND_OPTS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbindopts: *mut BIND_OPTS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprot: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszkey: super::super::Foundation::PWSTR, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszkey: super::super::Foundation::PWSTR, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszkey: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindHost(pub ::windows::runtime::IUnknown);
impl IBindHost {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn CreateMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindHost {
    type Vtable = IBindHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4232577441, 11177, 4559, [162, 41, 0, 170, 0, 61, 115, 82]);
}
impl ::core::convert::From<IBindHost> for ::windows::runtime::IUnknown {
    fn from(value: IBindHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindHost> for ::windows::runtime::IUnknown {
    fn from(value: &IBindHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBindHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szname: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pbsc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pbsc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindStatusCallback(pub ::windows::runtime::IUnknown);
impl IBindStatusCallback {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::runtime::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::runtime::HRESULT, szerror: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045430209, 47865, 4558, [140, 130, 0, 170, 0, 75, 169, 11]);
}
impl ::core::convert::From<IBindStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBindStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32, pib: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnpriority: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: ::windows::runtime::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindStatusCallbackEx(pub ::windows::runtime::IUnknown);
impl IBindStatusCallbackEx {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::runtime::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::runtime::HRESULT, szerror: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2863091449, 36583, 18009, [136, 217, 248, 197, 4, 218, 115, 204]);
}
impl ::core::convert::From<IBindStatusCallbackEx> for ::windows::runtime::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for ::windows::runtime::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindStatusCallback> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindStatusCallback> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindStatusCallback> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindStatusCallback> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32, pib: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnpriority: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: ::windows::runtime::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBinding(pub ::windows::runtime::IUnknown);
impl IBinding {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Suspend(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(npriority)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::runtime::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsidprotocol), ::core::mem::transmute(pdwresult), ::core::mem::transmute(pszresult), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBinding {
    type Vtable = IBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045430208, 47865, 4558, [140, 130, 0, 170, 0, 75, 169, 11]);
}
impl ::core::convert::From<IBinding> for ::windows::runtime::IUnknown {
    fn from(value: IBinding) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBinding> for ::windows::runtime::IUnknown {
    fn from(value: &IBinding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, npriority: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnpriority: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclsidprotocol: *mut ::windows::runtime::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBlockingLock(pub ::windows::runtime::IUnknown);
impl IBlockingLock {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unlock(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBlockingLock {
    type Vtable = IBlockingLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(821286010, 25671, 4561, [142, 60, 0, 192, 79, 185, 56, 109]);
}
impl ::core::convert::From<IBlockingLock> for ::windows::runtime::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBlockingLock> for ::windows::runtime::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBlockingLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwtimeout: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallFactory(pub ::windows::runtime::IUnknown);
impl ICallFactory {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateCall<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, pctrlunk: Param1, riid2: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pctrlunk.into_param().abi(), ::core::mem::transmute(riid2), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICallFactory {
    type Vtable = ICallFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(477313584, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
impl ::core::convert::From<ICallFactory> for ::windows::runtime::IUnknown {
    fn from(value: ICallFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICallFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pctrlunk: ::windows::runtime::RawPtr, riid2: *const ::windows::runtime::GUID, ppv: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICancelMethodCalls(pub ::windows::runtime::IUnknown);
impl ICancelMethodCalls {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn TestCancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(41, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ICancelMethodCalls> for ::windows::runtime::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICancelMethodCalls> for ::windows::runtime::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICancelMethodCalls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulseconds: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatInformation(pub ::windows::runtime::IUnknown);
impl ICatInformation {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::runtime::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::runtime::GUID, lcid: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::runtime::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICatInformation {
    type Vtable = ICatInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188435, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ICatInformation> for ::windows::runtime::IUnknown {
    fn from(value: ICatInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICatInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcid: u32, ppenumcategoryinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rcatid: *const ::windows::runtime::GUID, lcid: u32, pszdesc: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID, ppenumclsid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ppenumcatid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ppenumcatid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatRegister(pub ::windows::runtime::IUnknown);
impl ICatRegister {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcategoryinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICatRegister {
    type Vtable = ICatRegister_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188434, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ICatRegister> for ::windows::runtime::IUnknown {
    fn from(value: ICatRegister) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatRegister> for ::windows::runtime::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICatRegister {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IChannelHook(pub ::windows::runtime::IUnknown);
impl IChannelHook {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::runtime::HRESULT) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep), ::core::mem::transmute(hrfault)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, hrfault: ::windows::runtime::HRESULT, pdatasize: *mut u32) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(hrfault), ::core::mem::transmute(pdatasize)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::runtime::HRESULT) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(hrfault)))
    }
}
unsafe impl ::windows::runtime::Interface for IChannelHook {
    type Vtable = IChannelHook_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(269010080, 30227, 4559, [154, 241, 0, 32, 175, 110, 114, 244]);
}
impl ::core::convert::From<IChannelHook> for ::windows::runtime::IUnknown {
    fn from(value: IChannelHook) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IChannelHook> for ::windows::runtime::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IChannelHook {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::runtime::HRESULT),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, hrfault: ::windows::runtime::HRESULT, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::runtime::HRESULT),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClassActivator(pub ::windows::runtime::IUnknown);
impl IClassActivator {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassObject<T: ::windows::runtime::Interface>(&self, rclsid: *const ::windows::runtime::GUID, dwclasscontext: u32, locale: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(locale), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IClassActivator {
    type Vtable = IClassActivator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(320, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IClassActivator> for ::windows::runtime::IUnknown {
    fn from(value: IClassActivator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClassActivator> for ::windows::runtime::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClassActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClassFactory(pub ::windows::runtime::IUnknown);
impl IClassFactory {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, punkouter: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn LockServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClassFactory {
    type Vtable = IClassFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IClassFactory> for ::windows::runtime::IUnknown {
    fn from(value: IClassFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClassFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClassFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClientSecurity(pub ::windows::runtime::IUnknown);
impl IClientSecurity {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            pproxy.into_param().abi(),
            ::core::mem::transmute(pauthnsvc),
            ::core::mem::transmute(pauthzsvc),
            ::core::mem::transmute(pserverprincname),
            ::core::mem::transmute(pauthnlevel),
            ::core::mem::transmute(pimplevel),
            ::core::mem::transmute(pauthinfo),
            ::core::mem::transmute(pcapabilites),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::core::mem::transmute_copy(self),
            pproxy.into_param().abi(),
            ::core::mem::transmute(dwauthnsvc),
            ::core::mem::transmute(dwauthzsvc),
            pserverprincname.into_param().abi(),
            ::core::mem::transmute(dwauthnlevel),
            ::core::mem::transmute(dwimplevel),
            ::core::mem::transmute(pauthinfo),
            ::core::mem::transmute(dwcapabilities),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CopyProxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pproxy: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IClientSecurity {
    type Vtable = IClientSecurity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(317, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IClientSecurity> for ::windows::runtime::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClientSecurity> for ::windows::runtime::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClientSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, ppcopy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComThreadingInfo(pub ::windows::runtime::IUnknown);
impl IComThreadingInfo {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::runtime::Result<APTTYPE> {
        let mut result__: <APTTYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APTTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::runtime::Result<THDTYPE> {
        let mut result__: <THDTYPE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<THDTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(462, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IComThreadingInfo> for ::windows::runtime::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComThreadingInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IComThreadingInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, papttype: *mut APTTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pthreadtype: *mut THDTYPE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidlogicalthreadid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnectionPoint(pub ::windows::runtime::IUnknown);
impl IConnectionPoint {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::runtime::Result<IConnectionPointContainer> {
        let mut result__: <IConnectionPointContainer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IConnectionPointContainer>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punksink: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punksink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumConnections(&self) -> ::windows::runtime::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnectionPoint {
    type Vtable = IConnectionPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443334, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::core::convert::From<IConnectionPoint> for ::windows::runtime::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnectionPoint> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConnectionPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcpc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punksink: ::windows::runtime::RawPtr, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnectionPointContainer(pub ::windows::runtime::IUnknown);
impl IConnectionPointContainer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::runtime::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IConnectionPoint> {
        let mut result__: <IConnectionPoint as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<IConnectionPoint>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443332, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::core::convert::From<IConnectionPointContainer> for ::windows::runtime::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnectionPointContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConnectionPointContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppcp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IContext(pub u8);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContextCallback(pub ::windows::runtime::IUnknown);
impl IContextCallback {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ContextCallback<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfncallback: ::core::option::Option<PFNCONTEXTCALL>, pparam: *const ComCallData, riid: *const ::windows::runtime::GUID, imethod: i32, punk: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pparam), ::core::mem::transmute(riid), ::core::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextCallback {
    type Vtable = IContextCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(474, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IContextCallback> for ::windows::runtime::IUnknown {
    fn from(value: IContextCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContextCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContextCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfncallback: ::windows::runtime::RawPtr, pparam: *const ComCallData, riid: *const ::windows::runtime::GUID, imethod: i32, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataAdviseHolder(pub ::windows::runtime::IUnknown);
impl IDataAdviseHolder {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IDataObject>, Param3: ::windows::runtime::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(pfetc), ::core::mem::transmute(advf), padvise.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows::runtime::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(advf)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(272, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IDataAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDataAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::runtime::RawPtr, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumadvise: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataobject: ::windows::runtime::RawPtr, dwreserved: u32, advf: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataObject(pub ::windows::runtime::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::runtime::Result<STGMEDIUM> {
        let mut result__: <STGMEDIUM as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetcin), &mut result__).from_abi::<STGMEDIUM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC) -> ::windows::runtime::Result<FORMATETC> {
        let mut result__: <FORMATETC as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatectin), &mut result__).from_abi::<FORMATETC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetData<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::runtime::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdirection), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DAdvise<'a, Param2: ::windows::runtime::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(advf), padvsink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumDAdvise(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDataObject {
    type Vtable = IDataObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(270, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IDataObject> for ::windows::runtime::IUnknown {
    fn from(value: IDataObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataObject> for ::windows::runtime::IUnknown {
    fn from(value: &IDataObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDataObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetcin: *const FORMATETC, pmedium: *mut ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pmedium: *mut ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pmedium: *const ::core::mem::ManuallyDrop<STGMEDIUM>, frelease: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdirection: u32, ppenumformatetc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::runtime::RawPtr, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumadvise: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumCATEGORYINFO(pub ::windows::runtime::IUnknown);
impl IEnumCATEGORYINFO {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188433, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumCATEGORYINFO> for ::windows::runtime::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumCATEGORYINFO> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumConnectionPoints(pub ::windows::runtime::IUnknown);
impl IEnumConnectionPoints {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, cconnections: u32, ppcp: *mut ::core::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(ppcp), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443333, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::core::convert::From<IEnumConnectionPoints> for ::windows::runtime::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumConnectionPoints> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumConnectionPoints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32, ppcp: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumConnections(pub ::windows::runtime::IUnknown);
impl IEnumConnections {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(rgcd), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumConnections {
    type Vtable = IEnumConnections_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443335, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::core::convert::From<IEnumConnections> for ::windows::runtime::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumConnections> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32, rgcd: *mut ::core::mem::ManuallyDrop<CONNECTDATA>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IEnumContextProps(pub u8);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumFORMATETC(pub ::windows::runtime::IUnknown);
impl IEnumFORMATETC {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(259, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumFORMATETC> for ::windows::runtime::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumFORMATETC> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumFORMATETC {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumGUID(pub ::windows::runtime::IUnknown);
impl IEnumGUID {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::runtime::GUID, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumGUID {
    type Vtable = IEnumGUID_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188416, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumGUID> for ::windows::runtime::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumGUID> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumGUID {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::GUID, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumMoniker(pub ::windows::runtime::IUnknown);
impl IEnumMoniker {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumMoniker {
    type Vtable = IEnumMoniker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumMoniker> for ::windows::runtime::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumMoniker> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumSTATDATA(pub ::windows::runtime::IUnknown);
impl IEnumSTATDATA {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(261, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumSTATDATA> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumSTATDATA> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSTATDATA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::core::mem::ManuallyDrop<STATDATA>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumString(pub ::windows::runtime::IUnknown);
impl IEnumString {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumString {
    type Vtable = IEnumString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(257, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumString> for ::windows::runtime::IUnknown {
    fn from(value: IEnumString) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumString> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumString) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumUnknown(pub ::windows::runtime::IUnknown);
impl IEnumUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<::windows::runtime::IUnknown>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumUnknown> {
        let mut result__: <IEnumUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumUnknown {
    type Vtable = IEnumUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(256, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IEnumUnknown> for ::windows::runtime::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::windows::runtime::RawPtr, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IExternalConnection(pub ::windows::runtime::IUnknown);
impl IExternalConnection {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IExternalConnection {
    type Vtable = IExternalConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(25, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IExternalConnection> for ::windows::runtime::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IExternalConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IExternalConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extconn: u32, reserved: u32) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFastRundown(pub ::windows::runtime::IUnknown);
impl IFastRundown {}
unsafe impl ::windows::runtime::Interface for IFastRundown {
    type Vtable = IFastRundown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(64, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IFastRundown> for ::windows::runtime::IUnknown {
    fn from(value: IFastRundown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFastRundown> for ::windows::runtime::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFastRundown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IForegroundTransfer(pub ::windows::runtime::IUnknown);
impl IForegroundTransfer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransfer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(325, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IForegroundTransfer> for ::windows::runtime::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IForegroundTransfer> for ::windows::runtime::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IForegroundTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGlobalInterfaceTable(pub ::windows::runtime::IUnknown);
impl IGlobalInterfaceTable {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(326, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IGlobalInterfaceTable> for ::windows::runtime::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGlobalInterfaceTable> for ::windows::runtime::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGlobalOptions(pub ::windows::runtime::IUnknown);
impl IGlobalOptions {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGlobalOptions {
    type Vtable = IGlobalOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IGlobalOptions> for ::windows::runtime::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGlobalOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGlobalOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        IIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInitializeSpy(pub ::windows::runtime::IUnknown);
impl IInitializeSpy {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::runtime::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcoinit), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInitializeSpy {
    type Vtable = IInitializeSpy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(52, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IInitializeSpy> for ::windows::runtime::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInitializeSpy> for ::windows::runtime::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInitializeSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrcoinit: ::windows::runtime::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcurthreadaptrefs: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwnewthreadaptrefs: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternalUnknown(pub ::windows::runtime::IUnknown);
impl IInternalUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInternalUnknown {
    type Vtable = IInternalUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(33, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IInternalUnknown> for ::windows::runtime::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternalUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInternalUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMachineGlobalObjectTable(pub ::windows::runtime::IUnknown);
impl IMachineGlobalObjectTable {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RegisterObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, clsid: *const ::windows::runtime::GUID, identifier: Param1, object: Param2) -> ::windows::runtime::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__: <*mut MachineGlobalObjectTableRegistrationToken__ as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), &mut result__).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID, identifier: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(token)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(651626924, 63243, 17441, [169, 111, 210, 135, 143, 175, 176, 13]);
}
impl ::core::convert::From<IMachineGlobalObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMachineGlobalObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, identifier: super::super::Foundation::PWSTR, object: ::windows::runtime::RawPtr, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMalloc(pub ::windows::runtime::IUnknown);
impl IMalloc {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Free(&self, pv: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn HeapMinimize(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IMalloc {
    type Vtable = IMalloc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IMalloc> for ::windows::runtime::IUnknown {
    fn from(value: IMalloc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMalloc> for ::windows::runtime::IUnknown {
    fn from(value: &IMalloc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMalloc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMallocSpy(pub ::windows::runtime::IUnknown);
impl IMallocSpy {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbrequest)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreFree<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostFree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fspyed: Param0) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreRealloc<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: Param3) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), ::core::mem::transmute(cbrequest), ::core::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostRealloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pactual: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreGetSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostGetSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi(), ::core::mem::transmute(factual)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreHeapMinimize(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostHeapMinimize(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IMallocSpy {
    type Vtable = IMallocSpy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IMallocSpy> for ::windows::runtime::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMallocSpy> for ::windows::runtime::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMallocSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbrequest: usize) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMoniker(pub ::windows::runtime::IUnknown);
impl IMoniker {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn BindToObject<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn BindToStorage<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reduce<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pbc.into_param().abi(), ::core::mem::transmute(dwreducehowfar), ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ComposeWith<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Enum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforward: Param0) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fforward.into_param().abi(), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmkothermoniker.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Hash(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param2: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Inverse(&self) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RelativePathTo<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMoniker {
    type Vtable = IMoniker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IMoniker> for ::windows::runtime::IUnknown {
    fn from(value: IMoniker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMoniker> for ::windows::runtime::IUnknown {
    fn from(value: &IMoniker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IMoniker> for IPersistStream {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersistStream {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistStream> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistStream> for &IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMoniker> for IPersist {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for IPersist {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbsize: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, riidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::runtime::RawPtr, ppmkreduced: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkright: ::windows::runtime::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkothermoniker: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhash: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, pmknewlyrunning: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkother: ::windows::runtime::RawPtr, ppmkprefix: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkother: ::windows::runtime::RawPtr, ppmkrelpath: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwmksys: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultiQI(pub ::windows::runtime::IUnknown);
impl IMultiQI {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMultiQI {
    type Vtable = IMultiQI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(32, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: IMultiQI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmqis: u32, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct INTERFACEINFO {
    pub pUnk: ::core::option::Option<::windows::runtime::IUnknown>,
    pub iid: ::windows::runtime::GUID,
    pub wMethod: u16,
}
impl INTERFACEINFO {}
impl ::core::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
unsafe impl ::windows::runtime::Abi for INTERFACEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INoMarshal(pub ::windows::runtime::IUnknown);
impl INoMarshal {}
unsafe impl ::windows::runtime::Interface for INoMarshal {
    type Vtable = INoMarshal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3972557083, 49627, 19904, [133, 94, 101, 246, 197, 81, 175, 73]);
}
impl ::core::convert::From<INoMarshal> for ::windows::runtime::IUnknown {
    fn from(value: INoMarshal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INoMarshal> for ::windows::runtime::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a INoMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOplockStorage(pub ::windows::runtime::IUnknown);
impl IOplockStorage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOplockStorage {
    type Vtable = IOplockStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2367277108, 34937, 4561, [131, 233, 0, 192, 79, 194, 198, 212]);
}
impl ::core::convert::From<IOplockStorage> for ::windows::runtime::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOplockStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IOplockStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::runtime::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::runtime::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPSFactoryBuffer(pub ::windows::runtime::IUnknown);
impl IPSFactoryBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateProxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows::runtime::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppproxy), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateStub<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punkserver: Param1) -> ::windows::runtime::Result<IRpcStubBuffer> {
        let mut result__: <IRpcStubBuffer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punkserver.into_param().abi(), &mut result__).from_abi::<IRpcStubBuffer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630416, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::core::convert::From<IPSFactoryBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPSFactoryBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPSFactoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppproxy: *mut ::windows::runtime::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punkserver: ::windows::runtime::RawPtr, ppstub: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersist(pub ::windows::runtime::IUnknown);
impl IPersist {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersist {
    type Vtable = IPersist_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(268, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IPersist> for ::windows::runtime::IUnknown {
    fn from(value: IPersist) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersist> for ::windows::runtime::IUnknown {
    fn from(value: &IPersist) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistFile(pub ::windows::runtime::IUnknown);
impl IPersistFile {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetCurFile(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersistFile {
    type Vtable = IPersistFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(267, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: IPersistFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPersistFile> for IPersist {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for IPersist {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilename: super::super::Foundation::PWSTR, dwmode: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilename: super::super::Foundation::PWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszfilename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistStream(pub ::windows::runtime::IUnknown);
impl IPersistStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersistStream {
    type Vtable = IPersistStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IPersistStream> for ::windows::runtime::IUnknown {
    fn from(value: IPersistStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistStream> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPersistStream> for IPersist {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for IPersist {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclassid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcbsize: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeByte(pub ::windows::runtime::IUnknown);
impl IPipeByte {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const u8, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeByte {
    type Vtable = IPipeByte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305546, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<IPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: IPipeByte) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const u8, csent: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeDouble(pub ::windows::runtime::IUnknown);
impl IPipeDouble {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const f64, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeDouble {
    type Vtable = IPipeDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305550, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<IPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const f64, csent: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeLong(pub ::windows::runtime::IUnknown);
impl IPipeLong {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const i32, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeLong {
    type Vtable = IPipeLong_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305548, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::core::convert::From<IPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: IPipeLong) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buf: *const i32, csent: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProcessInitControl(pub ::windows::runtime::IUnknown);
impl IProcessInitControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsecondsremaining)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProcessInitControl {
    type Vtable = IProcessInitControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1916276053, 36139, 17315, [133, 19, 43, 110, 243, 20, 52, 233]);
}
impl ::core::convert::From<IProcessInitControl> for ::windows::runtime::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProcessInitControl> for ::windows::runtime::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProcessInitControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsecondsremaining: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProcessLock(pub ::windows::runtime::IUnknown);
impl IProcessLock {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IProcessLock {
    type Vtable = IProcessLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IProcessLock> for ::windows::runtime::IUnknown {
    fn from(value: IProcessLock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProcessLock> for ::windows::runtime::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProcessLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProgressNotify(pub ::windows::runtime::IUnknown);
impl IProgressNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprogresscurrent), ::core::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProgressNotify {
    type Vtable = IProgressNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2849462432, 17943, 4559, [149, 252, 0, 170, 0, 104, 13, 180]);
}
impl ::core::convert::From<IProgressNotify> for ::windows::runtime::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProgressNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProgressNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IROTData(pub ::windows::runtime::IUnknown);
impl IROTData {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbdata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IROTData {
    type Vtable = IROTData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4070534080, 20513, 4558, [170, 21, 0, 0, 105, 1, 41, 63]);
}
impl ::core::convert::From<IROTData> for ::windows::runtime::IUnknown {
    fn from(value: IROTData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IROTData> for ::windows::runtime::IUnknown {
    fn from(value: &IROTData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IROTData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IROTData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReleaseMarshalBuffers(pub ::windows::runtime::IUnknown);
impl IReleaseMarshalBuffers {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3943479784, 31126, 4562, [135, 46, 0, 0, 248, 8, 8, 89]);
}
impl ::core::convert::From<IReleaseMarshalBuffers> for ::windows::runtime::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReleaseMarshalBuffers> for ::windows::runtime::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer(pub ::windows::runtime::IUnknown);
impl IRpcChannelBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630816, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::core::convert::From<IRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer2(pub ::windows::runtime::IUnknown);
impl IRpcChannelBuffer2 {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1498362320, 32537, 4560, [177, 148, 0, 160, 201, 13, 200, 191]);
}
impl ::core::convert::From<IRpcChannelBuffer2> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer3(pub ::windows::runtime::IUnknown);
impl IRpcChannelBuffer3 {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(ulsize), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterAsync<'a, Param1: ::windows::runtime::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(632378880, 277, 4560, [191, 13, 0, 170, 0, 184, 223, 210]);
}
impl ::core::convert::From<IRpcChannelBuffer3> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcHelper(pub ::windows::runtime::IUnknown);
impl IRpcHelper {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::runtime::Result<*mut ::windows::runtime::GUID> {
        let mut result__: <*mut ::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobjref), &mut result__).from_abi::<*mut ::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcHelper {
    type Vtable = IRpcHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(329, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IRpcHelper> for ::windows::runtime::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcOptions(pub ::windows::runtime::IUnknown);
impl IRpcOptions {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Set<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcOptions {
    type Vtable = IRpcOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(324, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IRpcOptions> for ::windows::runtime::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprx: ::windows::runtime::RawPtr, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprx: ::windows::runtime::RawPtr, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcProxyBuffer(pub ::windows::runtime::IUnknown);
impl IRpcProxyBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Disconnect(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630516, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::core::convert::From<IRpcProxyBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcProxyBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcProxyBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prpcchannelbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcStubBuffer(pub ::windows::runtime::IUnknown);
impl IRpcStubBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Disconnect(&self) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::runtime::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::runtime::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CountRefs(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
}
unsafe impl ::windows::runtime::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630716, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::core::convert::From<IRpcStubBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcStubBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcStubBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkserver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::RawPtr,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcSyntaxNegotiate(pub ::windows::runtime::IUnknown);
impl IRpcSyntaxNegotiate {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1486914841, 9416, 18741, [180, 130, 63, 216, 35, 51, 58, 79]);
}
impl ::core::convert::From<IRpcSyntaxNegotiate> for ::windows::runtime::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcSyntaxNegotiate> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRunnableObject(pub ::windows::runtime::IUnknown);
impl IRunnableObject {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetRunningClass(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Run<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn LockRunning<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetContainedObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcontained: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), fcontained.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRunnableObject {
    type Vtable = IRunnableObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(294, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IRunnableObject> for ::windows::runtime::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRunnableObject> for ::windows::runtime::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRunnableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcontained: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRunningObjectTable(pub ::windows::runtime::IUnknown);
impl IRunningObjectTable {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Register<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister), ::core::mem::transmute(pfiletime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumRunning(&self) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(16, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IRunningObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRunningObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRunningObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfflags: u32, punkobject: ::windows::runtime::RawPtr, pmkobjectname: ::windows::runtime::RawPtr, pdwregister: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregister: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkobjectname: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkobjectname: ::windows::runtime::RawPtr, ppunkobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmkobjectname: ::windows::runtime::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenummoniker: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISequentialStream(pub ::windows::runtime::IUnknown);
impl ISequentialStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(208878128, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
impl ::core::convert::From<ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServerSecurity(pub ::windows::runtime::IUnknown);
impl IServerSecurity {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ImpersonateClient(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevertToSelf(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IServerSecurity {
    type Vtable = IServerSecurity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(318, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IServerSecurity> for ::windows::runtime::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServerSecurity> for ::windows::runtime::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServerSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceProvider(pub ::windows::runtime::IUnknown);
impl IServiceProvider {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceProvider {
    type Vtable = IServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1834041537, 29750, 4558, [128, 52, 0, 170, 0, 96, 9, 250]);
}
impl ::core::convert::From<IServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStdMarshalInfo(pub ::windows::runtime::IUnknown);
impl IStdMarshalInfo {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(pclsid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IStdMarshalInfo> for ::windows::runtime::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStdMarshalInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStdMarshalInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStream(pub ::windows::runtime::IUnknown);
impl IStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IStream> {
        let mut result__: <IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IStream> for ::windows::runtime::IUnknown {
    fn from(value: IStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStream> for ::windows::runtime::IUnknown {
    fn from(value: &IStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for &IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurrogate(pub ::windows::runtime::IUnknown);
impl ISurrogate {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeSurrogate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurrogate {
    type Vtable = ISurrogate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(34, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISurrogate> for ::windows::runtime::IUnknown {
    fn from(value: ISurrogate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurrogate> for ::windows::runtime::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISurrogate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurrogateService(pub ::windows::runtime::IUnknown);
impl ISurrogateService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Init<'a, Param1: ::windows::runtime::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows::runtime::GUID, pprocesslock: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::runtime::GUID, apptype: ApplicationType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid), ::core::mem::transmute(apptype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulreserved)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(shutdowntype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurrogateService {
    type Vtable = ISurrogateService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(468, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISurrogateService> for ::windows::runtime::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurrogateService> for ::windows::runtime::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISurrogateService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidprocessid: *const ::windows::runtime::GUID, pprocesslock: ::windows::runtime::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidapplid: *const ::windows::runtime::GUID, apptype: ApplicationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rguidapplid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulreserved: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shutdowntype: ShutdownType) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronize(pub ::windows::runtime::IUnknown);
impl ISynchronize {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Signal(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronize {
    type Vtable = ISynchronize_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(48, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISynchronize> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronize) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronize> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronize {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeContainer(pub ::windows::runtime::IUnknown);
impl ISynchronizeContainer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::runtime::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(51, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISynchronizeContainer> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeContainer> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronizeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psync: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeEvent(pub ::windows::runtime::IUnknown);
impl ISynchronizeEvent {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ph)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISynchronizeEvent> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronizeHandle> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronizeHandle> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronizeHandle> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronizeHandle> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ph: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ph: *const super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeHandle(pub ::windows::runtime::IUnknown);
impl ISynchronizeHandle {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(49, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISynchronizeHandle> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeHandle> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronizeHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ph: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeMutex(pub ::windows::runtime::IUnknown);
impl ISynchronizeMutex {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Signal(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseMutex(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<ISynchronizeMutex> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ISynchronize {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ISynchronize {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronize> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronize> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronize> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronize> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITimeAndNoticeControl(pub ::windows::runtime::IUnknown);
impl ITimeAndNoticeControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(res1), ::core::mem::transmute(res2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154900654, 34936, 4561, [131, 233, 0, 192, 79, 194, 198, 212]);
}
impl ::core::convert::From<ITimeAndNoticeControl> for ::windows::runtime::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITimeAndNoticeControl> for ::windows::runtime::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, res1: u32, res2: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUri(pub ::windows::runtime::IUnknown);
impl IUri {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetAuthority(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetExtension(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetRawUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserInfo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetHostType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetScheme(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetZone(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUri {
    type Vtable = IUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2745100104, 27175, 18455, [166, 242, 19, 145, 75, 239, 88, 144]);
}
impl ::core::convert::From<IUri> for ::windows::runtime::IUnknown {
    fn from(value: IUri) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUri> for ::windows::runtime::IUnknown {
    fn from(value: &IUri) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrabsoluteuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrauthority: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdisplaystring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdomain: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextension: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfragment: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhost: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpassword: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpathandquery: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrquery: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrrawuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrschemename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruserinfo: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwhosttype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwscheme: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwzone: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puri: ::windows::runtime::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUriBuilder(pub ::windows::runtime::IUnknown);
impl IUriBuilder {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwuribuilderflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetIUri(&self) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetIUri<'a, Param0: ::windows::runtime::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), piuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchfragment), ::core::mem::transmute(ppwzfragment)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchhost), ::core::mem::transmute(ppwzhost)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpassword), ::core::mem::transmute(ppwzpassword)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpath), ::core::mem::transmute(ppwzpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfhasport), ::core::mem::transmute(pdwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchquery), ::core::mem::transmute(ppwzquery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchschemename), ::core::mem::transmute(ppwzschemename)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchusername), ::core::mem::transmute(ppwzusername)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetFragment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetHost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPassword<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), fhasport.into_param().abi(), ::core::mem::transmute(dwnewvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemeName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetUserName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpropertymask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn HasBeenModified(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUriBuilder {
    type Vtable = IUriBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1109504737, 35157, 18112, [189, 91, 222, 152, 151, 86, 93, 231]);
}
impl ::core::convert::From<IUriBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUriBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUriBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwpropertymask: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUrlMon(pub ::windows::runtime::IUnknown);
impl IUrlMon {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn AsyncGetClassBits<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, IBindCtx>>(
        &self,
        rclsid: *const ::windows::runtime::GUID,
        psztype: Param1,
        pszext: Param2,
        dwfileversionms: u32,
        dwfileversionls: u32,
        pszcodebase: Param5,
        pbc: Param6,
        dwclasscontext: u32,
        riid: *const ::windows::runtime::GUID,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(rclsid),
            psztype.into_param().abi(),
            pszext.into_param().abi(),
            ::core::mem::transmute(dwfileversionms),
            ::core::mem::transmute(dwfileversionls),
            pszcodebase.into_param().abi(),
            pbc.into_param().abi(),
            ::core::mem::transmute(dwclasscontext),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(flags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUrlMon {
    type Vtable = IUrlMon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(38, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IUrlMon> for ::windows::runtime::IUnknown {
    fn from(value: IUrlMon) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUrlMon> for ::windows::runtime::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUrlMon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, psztype: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, dwclasscontext: u32, riid: *const ::windows::runtime::GUID, flags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWaitMultiple(pub ::windows::runtime::IUnknown);
impl IWaitMultiple {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::runtime::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWaitMultiple {
    type Vtable = IWaitMultiple_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(43, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IWaitMultiple> for ::windows::runtime::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWaitMultiple> for ::windows::runtime::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWaitMultiple {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeout: u32, psync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psync: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl LONG_SIZEDARR {}
impl ::core::default::Default for LONG_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LONG_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for LONG_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub type LPFNCANUNLOADNOW = unsafe extern "system" fn() -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_Com`*"]
pub type LPFNGETCLASSOBJECT = unsafe extern "system" fn(param0: *const ::windows::runtime::GUID, param1: *const ::windows::runtime::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MEMCTX(pub i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::core::convert::From<i32> for MEMCTX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEMCTX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MKREDUCE(pub i32);
pub const MKRREDUCE_ONE: MKREDUCE = MKREDUCE(196608i32);
pub const MKRREDUCE_TOUSER: MKREDUCE = MKREDUCE(131072i32);
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = MKREDUCE(65536i32);
pub const MKRREDUCE_ALL: MKREDUCE = MKREDUCE(0i32);
impl ::core::convert::From<i32> for MKREDUCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MKREDUCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MKSYS(pub i32);
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
impl ::core::convert::From<i32> for MKSYS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MKSYS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MSHCTX(pub i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::core::convert::From<i32> for MSHCTX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSHCTX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MSHLFLAGS(pub i32);
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
impl ::core::convert::From<i32> for MSHLFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSHLFLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct MULTI_QI {
    pub pIID: *mut ::windows::runtime::GUID,
    pub pItf: ::core::option::Option<::windows::runtime::IUnknown>,
    pub hr: ::windows::runtime::HRESULT,
}
impl MULTI_QI {}
impl ::core::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
unsafe impl ::windows::runtime::Abi for MULTI_QI {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
unsafe impl ::windows::runtime::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows::runtime::RawPtr, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(pmkthis: Param0, pmkother: Param1) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerCommonPrefixWith(pmkthis: ::windows::runtime::RawPtr, pmkother: ::windows::runtime::RawPtr, ppmkcommon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonikerRelativePathTo<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pmksrc: Param0, pmkdest: Param1, ppmkrelpath: *mut ::core::option::Option<IMoniker>, dwreserved: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerRelativePathTo(pmksrc: ::windows::runtime::RawPtr, pmkdest: ::windows::runtime::RawPtr, ppmkrelpath: *mut ::windows::runtime::RawPtr, dwreserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        MonikerRelativePathTo(pmksrc.into_param().abi(), pmkdest.into_param().abi(), ::core::mem::transmute(ppmkrelpath), dwreserved.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PENDINGMSG(pub i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::core::convert::From<i32> for PENDINGMSG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PENDINGMSG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PENDINGTYPE(pub i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::core::convert::From<i32> for PENDINGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PENDINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub type PFNCONTEXTCALL = unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows::runtime::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        ProgIDFromCLSID(::core::mem::transmute(clsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl QUERYCONTEXT {}
impl ::core::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwContext == other.dwContext && self.Platform == other.Platform && self.Locale == other.Locale && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
unsafe impl ::windows::runtime::Abi for QUERYCONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGCLS(pub i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::core::convert::From<i32> for REGCLS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGCLS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl RPCOLEMESSAGE {}
impl ::core::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.dataRepresentation == other.dataRepresentation && self.Buffer == other.Buffer && self.cbBuffer == other.cbBuffer && self.iMethod == other.iMethod && self.reserved2 == other.reserved2 && self.rpcFlags == other.rpcFlags
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
unsafe impl ::windows::runtime::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPCOPT_PROPERTIES(pub i32);
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::core::convert::From<i32> for RPCOPT_PROPERTIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::core::convert::From<i32> for RPCOPT_SERVER_LOCALITY_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::core::convert::From<u32> for RPC_C_AUTHN_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_AUTHN_LEVEL {
    type Abi = Self;
}
impl ::core::ops::BitOr for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_AUTHN_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_AUTHN_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_IMP_LEVEL(pub u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::core::convert::From<u32> for RPC_C_IMP_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_IMP_LEVEL {
    type Abi = Self;
}
impl ::core::ops::BitOr for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_IMP_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_IMP_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl RemSTGMEDIUM {}
impl ::core::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.dwHandleType == other.dwHandleType && self.pData == other.pData && self.pUnkForRelease == other.pUnkForRelease && self.cbData == other.cbData && self.data == other.data
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
unsafe impl ::windows::runtime::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl SAFEARRAY {}
impl ::core::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cDims == other.cDims && self.fFeatures == other.fFeatures && self.cbElements == other.cbElements && self.cLocks == other.cLocks && self.pvData == other.pvData && self.rgsabound == other.rgsabound
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
unsafe impl ::windows::runtime::Abi for SAFEARRAY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl SAFEARRAYBOUND {}
impl ::core::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        self.cElements == other.cElements && self.lLbound == other.lLbound
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
unsafe impl ::windows::runtime::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::runtime::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::runtime::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl SChannelHookCallInfo {}
impl ::core::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.cbSize == other.cbSize && self.uCausality == other.uCausality && self.dwServerPid == other.dwServerPid && self.iMethod == other.iMethod && self.pObject == other.pObject
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
unsafe impl ::windows::runtime::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVERCALL(pub i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::core::convert::From<i32> for SERVERCALL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVERCALL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl SHORT_SIZEDARR {}
impl ::core::default::Default for SHORT_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SHORT_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::core::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for SHORT_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl SOLE_AUTHENTICATION_INFO {}
impl ::core::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pAuthInfo == other.pAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl SOLE_AUTHENTICATION_LIST {}
impl ::core::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cAuthInfo == other.cAuthInfo && self.aAuthInfo == other.aAuthInfo
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: super::super::Foundation::PWSTR,
    pub hr: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pPrincipalName == other.pPrincipalName && self.hr == other.hr
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::core::option::Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl STATDATA {}
impl ::core::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::core::cmp::Eq for STATDATA {}
unsafe impl ::windows::runtime::Abi for STATDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct STATSTG {
    pub pwcsName: super::super::Foundation::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows::runtime::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("STATSTG")
            .field("pwcsName", &self.pwcsName)
            .field("r#type", &self.r#type)
            .field("cbSize", &self.cbSize)
            .field("mtime", &self.mtime)
            .field("ctime", &self.ctime)
            .field("atime", &self.atime)
            .field("grfMode", &self.grfMode)
            .field("grfLocksSupported", &self.grfLocksSupported)
            .field("clsid", &self.clsid)
            .field("grfStateBits", &self.grfStateBits)
            .field("reserved", &self.reserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName && self.r#type == other.r#type && self.cbSize == other.cbSize && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.grfMode == other.grfMode && self.grfLocksSupported == other.grfLocksSupported && self.clsid == other.clsid && self.grfStateBits == other.grfStateBits && self.reserved == other.reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATSTG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::core::option::Option<::windows::runtime::IUnknown>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: ::windows::runtime::RawPtr,
    pub pstg: ::windows::runtime::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for STGMEDIUM_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct STGTY(pub i32);
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::core::convert::From<i32> for STGTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STGTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const STG_TOEND: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::core::convert::From<u32> for STREAM_SEEK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STREAM_SEEK {
    type Abi = Self;
}
impl ::core::ops::BitOr for STREAM_SEEK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for STREAM_SEEK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for STREAM_SEEK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for STREAM_SEEK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for STREAM_SEEK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShutdownType(pub i32);
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::core::convert::From<i32> for ShutdownType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ShutdownType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: super::super::Foundation::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        self.LayoutType == other.LayoutType && self.pwcsElementName == other.pwcsElementName && self.cOffset == other.cOffset && self.cBytes == other.cBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for StorageLayout {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        StringFromCLSID(::core::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::runtime::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows::runtime::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
        }
        ::core::mem::transmute(StringFromGUID2(::core::mem::transmute(rguid), ::core::mem::transmute(lpsz), ::core::mem::transmute(cchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        StringFromIID(::core::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct THDTYPE(pub i32);
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::core::convert::From<i32> for THDTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THDTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TYMED(pub i32);
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
pub const TYMED_FILE: TYMED = TYMED(2i32);
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
pub const TYMED_GDI: TYMED = TYMED(16i32);
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
pub const TYMED_NULL: TYMED = TYMED(0i32);
impl ::core::convert::From<i32> for TYMED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYMED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TYSPEC(pub i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::core::convert::From<i32> for TYSPEC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYSPEC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URI_CREATE_FLAGS(pub u32);
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
impl ::core::convert::From<u32> for URI_CREATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for URI_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Uri_PROPERTY(pub i32);
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
impl ::core::convert::From<i32> for Uri_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Uri_PROPERTY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub union VARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: i16,
    pub __OBSOLETE__VARIANT_BOOL: i16,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub punkVal: ::windows::runtime::RawPtr,
    pub pdispVal: ::windows::runtime::RawPtr,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut i16,
    pub __OBSOLETE__VARIANT_PBOOL: *mut i16,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut ::core::mem::ManuallyDrop<VARIANT>,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: super::super::Foundation::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::core::option::Option<super::Ole::Automation::IRecordInfo>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl WORD_BLOB {}
impl ::core::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
unsafe impl ::windows::runtime::Abi for WORD_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
#[cfg(feature = "Win32_Foundation")]
impl uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union uCLSSPEC_0 {
    pub clsid: ::windows::runtime::GUID,
    pub pFileExt: super::super::Foundation::PWSTR,
    pub pMimeType: super::super::Foundation::PWSTR,
    pub pProgId: super::super::Foundation::PWSTR,
    pub pFileName: super::super::Foundation::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::super::Foundation::PWSTR,
    pub PolicyId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ByName_e__Struct").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pPackageName == other.pPackageName && self.PolicyId == other.PolicyId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::runtime::GUID,
    pub PolicyId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl uCLSSPEC_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_ByObjectId_e__Struct").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectId == other.ObjectId && self.PolicyId == other.PolicyId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl userFLAG_STGMEDIUM {}
impl ::core::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
unsafe impl ::windows::runtime::Abi for userFLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::core::option::Option<::windows::runtime::IUnknown>,
}
impl userSTGMEDIUM {}
impl ::core::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
