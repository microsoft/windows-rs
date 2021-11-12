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
unsafe impl ::windows::core::Abi for ADVF {
    type Abi = Self;
}
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
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
unsafe impl ::windows::core::Abi for APTTYPE {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for APTTYPEQUALIFIER {
    type Abi = Self;
}
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl AUTHENTICATEINFO {}
impl ::core::default::Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUTHENTICATEINFO").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
unsafe impl ::windows::core::Abi for AUTHENTICATEINFO {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for ApplicationType {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIAdviseSink(pub ::windows::core::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    pub unsafe fn Finish_OnRename(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Finish_OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Finish_OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
impl ::core::convert::From<AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIAdviseSink2(pub ::windows::core::IUnknown);
impl AsyncIAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    pub unsafe fn Finish_OnRename(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Finish_OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Finish_OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
impl ::core::convert::From<AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, AsyncIAdviseSink> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, AsyncIAdviseSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, AsyncIAdviseSink> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, AsyncIAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIMultiQI(pub ::windows::core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::core::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
impl ::core::convert::From<AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cmqis: u32, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeByte(pub ::windows::core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeDouble(pub ::windows::core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIPipeLong(pub ::windows::core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, crequest: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncIUnknown(pub ::windows::core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn Begin_AddRef(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Begin_Release(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknown_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
impl ::core::convert::From<AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
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
    pub iid: ::windows::core::GUID,
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
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
unsafe impl ::windows::core::Abi for BINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
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
unsafe impl ::windows::core::Abi for BINDINFOF {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::windows::core::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for BINDPTR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for BINDPTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
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
unsafe impl ::windows::core::Abi for BIND_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for BIND_OPTS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for BIND_OPTS2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
#[inline]
pub unsafe fn BindMoniker<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(pmk: Param0, grfopt: u32, iidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindMoniker(pmk: ::windows::core::RawPtr, grfopt: u32, iidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        BindMoniker(pmk.into_param().abi(), ::core::mem::transmute(grfopt), ::core::mem::transmute(iidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLCONV(pub i32);
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::core::convert::From<i32> for CALLCONV {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CALLCONV {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for CALLTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CATEGORYINFO {
    pub catid: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for CATEGORYINFO {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for CLSCTX {
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for COAUTHINFO {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for COINIT {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COINITBASE(pub i32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::convert::From<i32> for COINITBASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COINITBASE {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for COMSD {
    type Abi = Self;
}
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct CONNECTDATA {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
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
unsafe impl ::windows::core::Abi for CONNECTDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for COSERVERINFO {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for COWAIT_FLAGS {
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
unsafe impl ::windows::core::Handle for CO_DEVICE_CATALOG_COOKIE {}
unsafe impl ::windows::core::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
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
unsafe impl ::windows::core::Handle for CO_MTA_USAGE_COOKIE {}
unsafe impl ::windows::core::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for CSPLATFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CUSTDATA").field("cCustData", &self.cCustData).field("prgCustData", &self.prgCustData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cCustData == other.cCustData && self.prgCustData == other.prgCustData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows::core::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATAITEM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
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
unsafe impl ::windows::core::Abi for CWMO_FLAGS {
    type Abi = Self;
}
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for CY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for CY_0 {
    type Abi = Self;
}
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
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: ::windows::core::RawPtr, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::core::mem::transmute(lpvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowUnmarshalerCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoAllowUnmarshalerCLSID(::core::mem::transmute(clsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> ::windows::core::HRESULT;
        }
        CoCancelCall(::core::mem::transmute(dwthreadid), ::core::mem::transmute(ultimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCopyProxy(pproxy: ::windows::core::RawPtr, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoCopyProxy(pproxy.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateFreeThreadedMarshaler(punkouter: ::windows::core::RawPtr, ppunkmarshal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateGuid(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoCreateGuid(&mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(rclsid: *const ::windows::core::GUID, punkouter: Param1, dwclscontext: CLSCTX) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstance(rclsid: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, dwclscontext: CLSCTX, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclscontext), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceEx(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pserverinfo), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows::core::GUID, punkouter: ::windows::core::RawPtr, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceFromApp(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(reserved), ::core::mem::transmute(dwcount), ::core::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDecrementMTAUsage<'a, Param0: ::windows::core::IntoParam<'a, CO_MTA_USAGE_COOKIE>>(cookie: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecrementMTAUsage(cookie: CO_MTA_USAGE_COOKIE) -> ::windows::core::HRESULT;
        }
        CoDecrementMTAUsage(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoDisableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectContext(dwtimeout: u32) -> ::windows::core::HRESULT;
        }
        CoDisconnectContext(::core::mem::transmute(dwtimeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoDisconnectObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectObject(punk: ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoDisconnectObject(punk.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnableCallCancellation(preserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoEnableCallCancellation(::core::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoFileTimeNow(&mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0) {
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
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> ::windows::core::HRESULT;
        }
        CoGetApartmentType(::core::mem::transmute(papttype), ::core::mem::transmute(paptqualifier)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallContext(riid: *const ::windows::core::GUID, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetCallContext(::core::mem::transmute(riid), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetCallerTID(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetCancelObject(::core::mem::transmute(dwthreadid), ::core::mem::transmute(iid), ::core::mem::transmute(ppunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObject(rclsid: *const ::windows::core::GUID, dwclscontext: CLSCTX, pvreserved: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetClassObject(::core::mem::transmute(rclsid), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(pvreserved), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows::core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetContextToken(ptoken: *mut usize) -> ::windows::core::HRESULT;
        }
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetContextToken(&mut result__).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetCurrentLogicalThreadId(&mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows::core::Result<IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMalloc as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetMalloc(::core::mem::transmute(dwmemcontext), &mut result__).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::core::mem::transmute(pbindoptions), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObjectContext(riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetObjectContext(::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetPSClsid(riid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetPSClsid(::core::mem::transmute(riid), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::HRESULT;
        }
        CoGetSystemSecurityPermissions(::core::mem::transmute(comsdtype), ::core::mem::transmute(ppsd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID, pclsidnew: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoGetTreatAsClass(::core::mem::transmute(clsidold), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoImpersonateClient() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoImpersonateClient() -> ::windows::core::HRESULT;
        }
        CoImpersonateClient().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows::core::Result<CO_MTA_USAGE_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: <CO_MTA_USAGE_COOKIE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoIncrementMTAUsage(&mut result__).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitialize(pvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoInitialize(::core::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeEx(pvreserved: *const ::core::ffi::c_void, dwcoinit: COINIT) -> ::windows::core::HRESULT;
        }
        CoInitializeEx(::core::mem::transmute(pvreserved), ::core::mem::transmute(dwcoinit)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows::core::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszmachinename: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInvalidateRemoteMachineBindings(pszmachinename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        CoInvalidateRemoteMachineBindings(pszmachinename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: ::windows::core::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsOle1Class(rclsid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsOle1Class(::core::mem::transmute(rclsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> super::super::Foundation::HINSTANCE {
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLockObjectExternal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(punk: Param0, flock: Param1, flastunlockreleases: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLockObjectExternal(punk: ::windows::core::RawPtr, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        CoLockObjectExternal(punk.into_param().abi(), flock.into_param().abi(), flastunlockreleases.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::core::HRESULT;
        }
        CoQueryAuthenticationServices(::core::mem::transmute(pcauthsvc), ::core::mem::transmute(asauthsvc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryClientBlanket(::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: ::windows::core::RawPtr, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(pwauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterActivationFilter<'a, Param0: ::windows::core::IntoParam<'a, IActivationFilter>>(pactivationfilter: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterActivationFilter(pactivationfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterActivationFilter(pactivationfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterChannelHook<'a, Param1: ::windows::core::IntoParam<'a, IChannelHook>>(extensionuuid: *const ::windows::core::GUID, pchannelhook: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterChannelHook(extensionuuid: *const ::windows::core::GUID, pchannelhook: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterChannelHook(::core::mem::transmute(extensionuuid), pchannelhook.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterClassObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(rclsid: *const ::windows::core::GUID, punk: Param1, dwclscontext: CLSCTX, flags: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterClassObject(rclsid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterClassObject(::core::mem::transmute(rclsid), punk.into_param().abi(), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(flags), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinstanceid: Param0) -> ::windows::core::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: <CO_DEVICE_CATALOG_COOKIE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.into_param().abi(), &mut result__).from_abi::<CO_DEVICE_CATALOG_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterInitializeSpy<'a, Param0: ::windows::core::IntoParam<'a, IInitializeSpy>>(pspy: Param0) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterInitializeSpy(pspy: ::windows::core::RawPtr, pulicookie: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterMallocSpy<'a, Param0: ::windows::core::IntoParam<'a, IMallocSpy>>(pmallocspy: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMallocSpy(pmallocspy: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterMallocSpy(pmallocspy.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const ::windows::core::GUID, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterPSClsid(riid: *const ::windows::core::GUID, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoRegisterPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(rclsid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRegisterSurrogate<'a, Param0: ::windows::core::IntoParam<'a, ISurrogate>>(psurrogate: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterSurrogate(psurrogate: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoRegisterSurrogate(psurrogate.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CoResumeClassObjects() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoResumeClassObjects() -> ::windows::core::HRESULT;
        }
        CoResumeClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevertToSelf() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevertToSelf() -> ::windows::core::HRESULT;
        }
        CoRevertToSelf().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeClassObject(dwregister: u32) -> ::windows::core::HRESULT;
        }
        CoRevokeClassObject(::core::mem::transmute(dwregister)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<'a, Param0: ::windows::core::IntoParam<'a, CO_DEVICE_CATALOG_COOKIE>>(cookie: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeDeviceCatalog(cookie: CO_DEVICE_CATALOG_COOKIE) -> ::windows::core::HRESULT;
        }
        CoRevokeDeviceCatalog(cookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeInitializeSpy(ulicookie: u64) -> ::windows::core::HRESULT;
        }
        CoRevokeInitializeSpy(::core::mem::transmute(ulicookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRevokeMallocSpy() -> ::windows::core::HRESULT;
        }
        CoRevokeMallocSpy().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSetCancelObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetCancelObject(punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoSetCancelObject(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: ::windows::core::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSuspendClassObjects() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSuspendClassObjects() -> ::windows::core::HRESULT;
        }
        CoSuspendClassObjects().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoSwitchCallContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pnewobject: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSwitchCallContext(pnewobject: ::windows::core::RawPtr, ppoldobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[inline]
pub unsafe fn CoTestCancel() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTestCancel() -> ::windows::core::HRESULT;
        }
        CoTestCancel().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTreatAsClass(clsidold: *const ::windows::core::GUID, clsidnew: *const ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        CoTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(clsidnew)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoWaitForMultipleHandles(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoWaitForMultipleObjects(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(chandles), ::core::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for ComCallData {
    type Abi = Self;
}
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAntiMoniker(ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateAntiMoniker(&mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows::core::Result<IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IBindCtx as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateBindCtx(::core::mem::transmute(reserved), &mut result__).from_abi::<IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateClassMoniker(rclsid: *const ::windows::core::GUID, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateClassMoniker(::core::mem::transmute(rclsid), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows::core::Result<IDataAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataAdviseHolder(ppdaholder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IDataAdviseHolder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszpathname: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateFileMoniker(lpszpathname.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateGenericComposite<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkfirst: Param0, pmkrest: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGenericComposite(pmkfirst: ::windows::core::RawPtr, pmkrest: ::windows::core::RawPtr, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateGenericComposite(pmkfirst.into_param().abi(), pmkrest.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateIUriBuilder<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(piuri: Param0, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUriBuilder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIUriBuilder(piuri: ::windows::core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IUriBuilder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateItemMoniker(lpszdelim.into_param().abi(), lpszitem.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateObjrefMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateObjrefMoniker(punk: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateObjrefMoniker(punk.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreatePointerMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePointerMoniker(punk: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows::core::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: ::windows::core::RawPtr, ppibsc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IBindStatusCallback as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), &mut result__).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::core::mem::transmute(dwencodingflags), ::core::mem::transmute(dwcodepage), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
unsafe impl ::windows::core::Abi for DATADIR {
    type Abi = Self;
}
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
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
unsafe impl ::windows::core::Abi for DCOM_CALL_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DESCKIND(pub i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::core::convert::From<i32> for DESCKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DESCKIND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPPARAMS").field("rgvarg", &self.rgvarg).field("rgdispidNamedArgs", &self.rgdispidNamedArgs).field("cArgs", &self.cArgs).field("cNamedArgs", &self.cNamedArgs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgvarg == other.rgvarg && self.rgdispidNamedArgs == other.rgdispidNamedArgs && self.cArgs == other.cArgs && self.cNamedArgs == other.cNamedArgs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DISPPARAMS {
    type Abi = Self;
}
pub const DMUS_ERRBASE: u32 = 4096u32;
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
unsafe impl ::windows::core::Abi for DVASPECT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for DWORD_BLOB {
    type Abi = Self;
}
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DcomChannelSetHResult(pvreserved: *const ::core::ffi::c_void, pulreserved: *const u32, appshr: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        DcomChannelSetHResult(::core::mem::transmute(pvreserved), ::core::mem::transmute(pulreserved), ::core::mem::transmute(appshr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC_0 {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: ::core::option::Option<LPEXCEPFINO_DEFERRED_FILLIN>,
    pub scode: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EXCEPINFO")
            .field("wCode", &self.wCode)
            .field("wReserved", &self.wReserved)
            .field("bstrSource", &self.bstrSource)
            .field("bstrDescription", &self.bstrDescription)
            .field("bstrHelpFile", &self.bstrHelpFile)
            .field("dwHelpContext", &self.dwHelpContext)
            .field("pvReserved", &self.pvReserved)
            .field("scode", &self.scode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXCEPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.wCode == other.wCode && self.wReserved == other.wReserved && self.bstrSource == other.bstrSource && self.bstrDescription == other.bstrDescription && self.bstrHelpFile == other.bstrHelpFile && self.dwHelpContext == other.dwHelpContext && self.pvReserved == other.pvReserved && self.pfnDeferredFillIn.map(|f| f as usize) == other.pfnDeferredFillIn.map(|f| f as usize) && self.scode == other.scode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXCEPINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXCEPINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
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
unsafe impl ::windows::core::Abi for EXTCONN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for FLAGGED_WORD_BLOB {
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
unsafe impl ::windows::core::Abi for FLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for FORMATETC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for FUNCDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for FUNCDESC {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FUNCKIND(pub i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::core::convert::From<i32> for FUNCKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FUNCKIND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
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
unsafe impl ::windows::core::Abi for GDI_OBJECT {
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
unsafe impl ::windows::core::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetClassFile(szfilename.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::core::Result<IErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IErrorInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetErrorInfo(::core::mem::transmute(dwreserved), &mut result__).from_abi::<IErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows::core::Result<IRunningObjectTable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IRunningObjectTable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetRunningObjectTable(::core::mem::transmute(reserved), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivationFilter(pub ::windows::core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwactivationtype), ::core::mem::transmute(rclsid), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IActivationFilter {
    type Vtable = IActivationFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAddrExclusionControl(pub ::windows::core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppenumerator)).ok()
    }
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, penumerator: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), penumerator.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, penumerator: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAddrTrackingControl(pub ::windows::core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAdviseSink(pub ::windows::core::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    pub unsafe fn OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink {
    type Vtable = IAdviseSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAdviseSink2(pub ::windows::core::IUnknown);
impl IAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)))
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex)))
    }
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    pub unsafe fn OnSave(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn OnClose(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmk.into_param().abi()))
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IAdviseSink> for IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAdviseSink> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAdviseSink> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAgileObject(pub ::windows::core::IUnknown);
impl IAgileObject {}
unsafe impl ::windows::core::Interface for IAgileObject {
    type Vtable = IAgileObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
impl ::core::convert::From<IAgileObject> for ::windows::core::IUnknown {
    fn from(value: IAgileObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows::core::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAsyncManager(pub ::windows::core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(result)).ok()
    }
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAsyncManager {
    type Vtable = IAsyncManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pulstateflags: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAsyncRpcChannelBuffer(pub ::windows::core::IUnknown);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Send<'a, Param1: ::windows::core::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), psync.into_param().abi(), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAuthenticate(pub ::windows::core::IUnknown);
impl IAuthenticate {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticate {
    type Vtable = IAuthenticate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: IAuthenticate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAuthenticateEx(pub ::windows::core::IUnknown);
impl IAuthenticateEx {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pauthinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticateEx {
    type Vtable = IAuthenticateEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
impl ::core::convert::From<IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: IAuthenticateEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticateEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAuthenticateEx> for IAuthenticate {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for IAuthenticate {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAuthenticate> for IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, IAuthenticate> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAuthenticate> for &IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, IAuthenticate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindCtx(pub ::windows::core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::core::Result<IRunningObjectTable> {
        let mut result__: <IRunningObjectTable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn EnumObjectParam(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszkey.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IBindCtx {
    type Vtable = IBindCtx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IBindCtx> for ::windows::core::IUnknown {
    fn from(value: IBindCtx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindCtx> for ::windows::core::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszkey: super::super::Foundation::PWSTR, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszkey: super::super::Foundation::PWSTR, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindHost(pub ::windows::core::IUnknown);
impl IBindHost {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateMoniker<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwreserved)).ok()
    }
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBindHost {
    type Vtable = IBindHost_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
impl ::core::convert::From<IBindHost> for ::windows::core::IUnknown {
    fn from(value: IBindHost) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindHost> for ::windows::core::IUnknown {
    fn from(value: &IBindHost) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szname: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindStatusCallback(pub ::windows::core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBindStatusCallbackEx(pub ::windows::core::IUnknown);
impl IBindStatusCallbackEx {
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
impl ::core::convert::From<IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IBindStatusCallback> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, IBindStatusCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBindStatusCallback> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, IBindStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::core::mem::ManuallyDrop<BINDINFO>, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBinding(pub ::windows::core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(npriority)).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsidprotocol), ::core::mem::transmute(pdwresult), ::core::mem::transmute(pszresult), ::core::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBinding {
    type Vtable = IBinding_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
impl ::core::convert::From<IBinding> for ::windows::core::IUnknown {
    fn from(value: IBinding) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBinding> for ::windows::core::IUnknown {
    fn from(value: &IBinding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, npriority: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBlockingLock(pub ::windows::core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IBlockingLock {
    type Vtable = IBlockingLock_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
impl ::core::convert::From<IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICallFactory(pub ::windows::core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, pctrlunk: Param1, riid2: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pctrlunk.into_param().abi(), ::core::mem::transmute(riid2), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for ICallFactory {
    type Vtable = ICallFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
impl ::core::convert::From<ICallFactory> for ::windows::core::IUnknown {
    fn from(value: ICallFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICallFactory> for ::windows::core::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pctrlunk: ::windows::core::RawPtr, riid2: *const ::windows::core::GUID, ppv: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICancelMethodCalls(pub ::windows::core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulseconds)).ok()
    }
    pub unsafe fn TestCancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatInformation(pub ::windows::core::IUnknown);
impl ICatInformation {
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::core::GUID, lcid: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(cimplemented), ::core::mem::transmute(rgcatidimpl), ::core::mem::transmute(crequired), ::core::mem::transmute(rgcatidreq)).ok()
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for ICatInformation {
    type Vtable = ICatInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ICatInformation> for ::windows::core::IUnknown {
    fn from(value: ICatInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatInformation> for ::windows::core::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lcid: u32, ppenumcategoryinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICatRegister(pub ::windows::core::IUnknown);
impl ICatRegister {
    pub unsafe fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcategoryinfo)).ok()
    }
    pub unsafe fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(ccategories), ::core::mem::transmute(rgcatid)).ok()
    }
}
unsafe impl ::windows::core::Interface for ICatRegister {
    type Vtable = ICatRegister_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ICatRegister> for ::windows::core::IUnknown {
    fn from(value: ICatRegister) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICatRegister> for ::windows::core::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IChannelHook(pub ::windows::core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize)))
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer)))
    }
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep), ::core::mem::transmute(hrfault)))
    }
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep)))
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(hrfault), ::core::mem::transmute(pdatasize)))
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(hrfault)))
    }
}
unsafe impl ::windows::core::Interface for IChannelHook {
    type Vtable = IChannelHook_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
impl ::core::convert::From<IChannelHook> for ::windows::core::IUnknown {
    fn from(value: IChannelHook) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IChannelHook> for ::windows::core::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClassActivator(pub ::windows::core::IUnknown);
impl IClassActivator {
    pub unsafe fn GetClassObject<T: ::windows::core::Interface>(&self, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(locale), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IClassActivator {
    type Vtable = IClassActivator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IClassActivator> for ::windows::core::IUnknown {
    fn from(value: IClassActivator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClassActivator> for ::windows::core::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClassFactory(pub ::windows::core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IClassFactory {
    type Vtable = IClassFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IClassFactory> for ::windows::core::IUnknown {
    fn from(value: IClassFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClassFactory> for ::windows::core::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IClientSecurity(pub ::windows::core::IUnknown);
impl IClientSecurity {
    pub unsafe fn QueryBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
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
    pub unsafe fn SetBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(
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
    pub unsafe fn CopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IClientSecurity {
    type Vtable = IClientSecurity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxy: ::windows::core::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxy: ::windows::core::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproxy: ::windows::core::RawPtr, ppcopy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IComThreadingInfo(pub ::windows::core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::core::Result<APTTYPE> {
        let mut result__: <APTTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<APTTYPE>(result__)
    }
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::core::Result<THDTYPE> {
        let mut result__: <THDTYPE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<THDTYPE>(result__)
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, papttype: *mut APTTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnectionPoint(pub ::windows::core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::core::Result<IConnectionPointContainer> {
        let mut result__: <IConnectionPointContainer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IConnectionPointContainer>(result__)
    }
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punksink: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), punksink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    pub unsafe fn EnumConnections(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::core::Interface for IConnectionPoint {
    type Vtable = IConnectionPoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
impl ::core::convert::From<IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcpc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punksink: ::windows::core::RawPtr, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnectionPointContainer(pub ::windows::core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<IConnectionPoint> {
        let mut result__: <IConnectionPoint as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), &mut result__).from_abi::<IConnectionPoint>(result__)
    }
}
unsafe impl ::windows::core::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
impl ::core::convert::From<IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppcp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IContext(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContextCallback(pub ::windows::core::IUnknown);
impl IContextCallback {
    pub unsafe fn ContextCallback<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pfncallback: ::core::option::Option<PFNCONTEXTCALL>, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pparam), ::core::mem::transmute(riid), ::core::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IContextCallback {
    type Vtable = IContextCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IContextCallback> for ::windows::core::IUnknown {
    fn from(value: IContextCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContextCallback> for ::windows::core::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfncallback: ::windows::core::RawPtr, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl IDLDESC {}
impl ::core::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IDLDESC").field("dwReserved", &self.dwReserved).field("wIDLFlags", &self.wIDLFlags).finish()
    }
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved == other.dwReserved && self.wIDLFlags == other.wIDLFlags
    }
}
impl ::core::cmp::Eq for IDLDESC {}
unsafe impl ::windows::core::Abi for IDLDESC {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataAdviseHolder(pub ::windows::core::IUnknown);
impl IDataAdviseHolder {
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>, Param3: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(pfetc), ::core::mem::transmute(advf), padvise.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(advf)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdataobject: ::windows::core::RawPtr, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDataObject(pub ::windows::core::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::core::Result<STGMEDIUM> {
        let mut result__: <STGMEDIUM as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetcin), &mut result__).from_abi::<STGMEDIUM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium)).ok()
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc)).ok()
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC) -> ::windows::core::Result<FORMATETC> {
        let mut result__: <FORMATETC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatectin), &mut result__).from_abi::<FORMATETC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdirection), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
    pub unsafe fn DAdvise<'a, Param2: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(advf), padvsink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    pub unsafe fn EnumDAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDataObject {
    type Vtable = IDataObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IDataObject> for ::windows::core::IUnknown {
    fn from(value: IDataObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDataObject> for ::windows::core::IUnknown {
    fn from(value: &IDataObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetcin: *const FORMATETC, pmedium: *mut ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pmedium: *mut ::core::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, pmedium: *const ::core::mem::ManuallyDrop<STGMEDIUM>, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwdirection: u32, ppenumformatetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwconnection: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDispatch(pub ::windows::core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for IDispatch {
    type Vtable = IDispatch_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IDispatch> for ::windows::core::IUnknown {
    fn from(value: IDispatch) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDispatch> for ::windows::core::IUnknown {
    fn from(value: &IDispatch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumCATEGORYINFO(pub ::windows::core::IUnknown);
impl IEnumCATEGORYINFO {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumConnectionPoints(pub ::windows::core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, cconnections: u32, ppcp: *mut ::core::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(ppcp), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
impl ::core::convert::From<IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cconnections: u32, ppcp: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cconnections: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumConnections(pub ::windows::core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections), ::core::mem::transmute(rgcd), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumConnections {
    type Vtable = IEnumConnections_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
impl ::core::convert::From<IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cconnections: u32, rgcd: *mut ::core::mem::ManuallyDrop<CONNECTDATA>, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cconnections: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct IEnumContextProps(pub u8);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumFORMATETC(pub ::windows::core::IUnknown);
impl IEnumFORMATETC {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumGUID(pub ::windows::core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumGUID {
    type Vtable = IEnumGUID_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumMoniker(pub ::windows::core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumMoniker {
    type Vtable = IEnumMoniker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumSTATDATA(pub ::windows::core::IUnknown);
impl IEnumSTATDATA {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::core::mem::ManuallyDrop<STATDATA>, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumString(pub ::windows::core::IUnknown);
impl IEnumString {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumString {
    type Vtable = IEnumString_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumString> for ::windows::core::IUnknown {
    fn from(value: IEnumString) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumString> for ::windows::core::IUnknown {
    fn from(value: &IEnumString) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumUnknown(pub ::windows::core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<::windows::core::IUnknown>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: <IEnumUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IEnumUnknown {
    type Vtable = IEnumUnknown_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IErrorInfo(pub ::windows::core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IErrorInfo {
    type Vtable = IErrorInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
impl ::core::convert::From<IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrsource: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IErrorLog(pub ::windows::core::IUnknown);
impl IErrorLog {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropname: Param0, pexcepinfo: *const EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pexcepinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IErrorLog {
    type Vtable = IErrorLog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
impl ::core::convert::From<IErrorLog> for ::windows::core::IUnknown {
    fn from(value: IErrorLog) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IErrorLog> for ::windows::core::IUnknown {
    fn from(value: &IErrorLog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszpropname: super::super::Foundation::PWSTR, pexcepinfo: *const ::core::mem::ManuallyDrop<EXCEPINFO>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IExternalConnection(pub ::windows::core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
unsafe impl ::windows::core::Interface for IExternalConnection {
    type Vtable = IExternalConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extconn: u32, reserved: u32) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IFastRundown(pub ::windows::core::IUnknown);
impl IFastRundown {}
unsafe impl ::windows::core::Interface for IFastRundown {
    type Vtable = IFastRundown_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IFastRundown> for ::windows::core::IUnknown {
    fn from(value: IFastRundown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFastRundown> for ::windows::core::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IForegroundTransfer(pub ::windows::core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvreserved)).ok()
    }
}
unsafe impl ::windows::core::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransfer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGlobalInterfaceTable(pub ::windows::core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(riid), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::core::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punk: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcookie: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGlobalOptions(pub ::windows::core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGlobalOptions {
    type Vtable = IGlobalOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInitializeSpy(pub ::windows::core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcoinit), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInitializeSpy {
    type Vtable = IInitializeSpy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInternalUnknown(pub ::windows::core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::core::Interface for IInternalUnknown {
    type Vtable = IInternalUnknown_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMachineGlobalObjectTable(pub ::windows::core::IUnknown);
impl IMachineGlobalObjectTable {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, clsid: *const ::windows::core::GUID, identifier: Param1, object: Param2) -> ::windows::core::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__: <*mut MachineGlobalObjectTableRegistrationToken__ as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), &mut result__).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID, identifier: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(token)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
impl ::core::convert::From<IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, object: ::windows::core::RawPtr, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMalloc(pub ::windows::core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)))
    }
    pub unsafe fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    pub unsafe fn Free(&self, pv: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    pub unsafe fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    pub unsafe fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    pub unsafe fn HeapMinimize(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IMalloc {
    type Vtable = IMalloc_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMalloc> for ::windows::core::IUnknown {
    fn from(value: IMalloc) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMalloc> for ::windows::core::IUnknown {
    fn from(value: &IMalloc) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMallocSpy(pub ::windows::core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbrequest)))
    }
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreFree<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostFree<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fspyed: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreRealloc<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: Param3) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), ::core::mem::transmute(cbrequest), ::core::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostRealloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pactual: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi(), ::core::mem::transmute(factual)))
    }
    pub unsafe fn PreHeapMinimize(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn PostHeapMinimize(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IMallocSpy {
    type Vtable = IMallocSpy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbrequest: usize) -> usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMoniker(pub ::windows::core::IUnknown);
impl IMoniker {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn BindToObject<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    pub unsafe fn BindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    pub unsafe fn Reduce<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pbc.into_param().abi(), ::core::mem::transmute(dwreducehowfar), ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComposeWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows::core::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforward: Param0) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), fforward.into_param().abi(), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pmkothermoniker.into_param().abi()).ok()
    }
    pub unsafe fn Hash(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    pub unsafe fn Inverse(&self) -> ::windows::core::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    pub unsafe fn RelativePathTo<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmkout)).ok()
    }
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMoniker {
    type Vtable = IMoniker_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMoniker> for ::windows::core::IUnknown {
    fn from(value: IMoniker) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMoniker> for ::windows::core::IUnknown {
    fn from(value: &IMoniker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IPersistStream> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersistStream> for &IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::core::RawPtr, ppmkreduced: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkright: ::windows::core::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkothermoniker: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhash: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pmknewlyrunning: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkother: ::windows::core::RawPtr, ppmkprefix: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkother: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwmksys: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMultiQI(pub ::windows::core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cmqis), ::core::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMultiQI {
    type Vtable = IMultiQI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMultiQI> for ::windows::core::IUnknown {
    fn from(value: IMultiQI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMultiQI> for ::windows::core::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cmqis: u32, pmqis: *mut ::core::mem::ManuallyDrop<MULTI_QI>) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct INTERFACEINFO {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub iid: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for INTERFACEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INVOKEKIND(pub i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::core::convert::From<i32> for INVOKEKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INVOKEKIND {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INoMarshal(pub ::windows::core::IUnknown);
impl INoMarshal {}
unsafe impl ::windows::core::Interface for INoMarshal {
    type Vtable = INoMarshal_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
impl ::core::convert::From<INoMarshal> for ::windows::core::IUnknown {
    fn from(value: INoMarshal) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INoMarshal> for ::windows::core::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IOplockStorage(pub ::windows::core::IUnknown);
impl IOplockStorage {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IOplockStorage {
    type Vtable = IOplockStorage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
impl ::core::convert::From<IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPSFactoryBuffer(pub ::windows::core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows::core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppproxy), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateStub<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punkserver: Param1) -> ::windows::core::Result<IRpcStubBuffer> {
        let mut result__: <IRpcStubBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punkserver.into_param().abi(), &mut result__).from_abi::<IRpcStubBuffer>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
impl ::core::convert::From<IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppproxy: *mut ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, punkserver: ::windows::core::RawPtr, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersist(pub ::windows::core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPersist {
    type Vtable = IPersist_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IPersist> for ::windows::core::IUnknown {
    fn from(value: IPersist) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersist> for ::windows::core::IUnknown {
    fn from(value: &IPersist) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistFile(pub ::windows::core::IUnknown);
impl IPersistFile {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPersistFile {
    type Vtable = IPersistFile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IPersistFile> for ::windows::core::IUnknown {
    fn from(value: IPersistFile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistFile> for ::windows::core::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfilename: super::super::Foundation::PWSTR, dwmode: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfilename: super::super::Foundation::PWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppszfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistMemory(pub ::windows::core::IUnknown);
impl IPersistMemory {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Load(&self, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmem), ::core::mem::transmute(cbsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmem: *mut ::core::ffi::c_void, fcleardirty: Param1, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmem), fcleardirty.into_param().abi(), ::core::mem::transmute(cbsize)).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPersistMemory {
    type Vtable = IPersistMemory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
impl ::core::convert::From<IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: IPersistMemory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: &IPersistMemory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPersistMemory> for IPersist {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for IPersist {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistStream(pub ::windows::core::IUnknown);
impl IPersistStream {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPersistStream {
    type Vtable = IPersistStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IPersistStream> for ::windows::core::IUnknown {
    fn from(value: IPersistStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistStream> for ::windows::core::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbsize: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPersistStreamInit(pub ::windows::core::IUnknown);
impl IPersistStreamInit {
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPersistStreamInit {
    type Vtable = IPersistStreamInit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
impl ::core::convert::From<IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: IPersistStreamInit) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: &IPersistStreamInit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPersistStreamInit> for IPersist {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for IPersist {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeByte(pub ::windows::core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPipeByte {
    type Vtable = IPipeByte_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<IPipeByte> for ::windows::core::IUnknown {
    fn from(value: IPipeByte) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeByte> for ::windows::core::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeDouble(pub ::windows::core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPipeDouble {
    type Vtable = IPipeDouble_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPipeLong(pub ::windows::core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(crequest), ::core::mem::transmute(pcreturned)).ok()
    }
    pub unsafe fn Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPipeLong {
    type Vtable = IPipeLong_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
impl ::core::convert::From<IPipeLong> for ::windows::core::IUnknown {
    fn from(value: IPipeLong) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPipeLong> for ::windows::core::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProcessInitControl(pub ::windows::core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsecondsremaining)).ok()
    }
}
unsafe impl ::windows::core::Interface for IProcessInitControl {
    type Vtable = IProcessInitControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
impl ::core::convert::From<IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwsecondsremaining: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProcessLock(pub ::windows::core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IProcessLock {
    type Vtable = IProcessLock_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IProcessLock> for ::windows::core::IUnknown {
    fn from(value: IProcessLock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProcessLock> for ::windows::core::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProgressNotify(pub ::windows::core::IUnknown);
impl IProgressNotify {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprogresscurrent), ::core::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IProgressNotify {
    type Vtable = IProgressNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
impl ::core::convert::From<IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IROTData(pub ::windows::core::IUnknown);
impl IROTData {
    pub unsafe fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbdata), ::core::mem::transmute(cbmax), ::core::mem::transmute(pcbdata)).ok()
    }
}
unsafe impl ::windows::core::Interface for IROTData {
    type Vtable = IROTData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
impl ::core::convert::From<IROTData> for ::windows::core::IUnknown {
    fn from(value: IROTData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IROTData> for ::windows::core::IUnknown {
    fn from(value: &IROTData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IReleaseMarshalBuffers(pub ::windows::core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
impl ::core::convert::From<IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer(pub ::windows::core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
impl ::core::convert::From<IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer2(pub ::windows::core::IUnknown);
impl IRpcChannelBuffer2 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
impl ::core::convert::From<IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwversion: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcChannelBuffer3(pub ::windows::core::IUnknown);
impl IRpcChannelBuffer3 {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(ulsize), ::core::mem::transmute(pulstatus)).ok()
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn RegisterAsync<'a, Param1: ::windows::core::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
impl ::core::convert::From<IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcHelper(pub ::windows::core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut ::windows::core::GUID> {
        let mut result__: <*mut ::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobjref), &mut result__).from_abi::<*mut ::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRpcHelper {
    type Vtable = IRpcHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcomversion: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcOptions(pub ::windows::core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRpcOptions {
    type Vtable = IRpcOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprx: ::windows::core::RawPtr, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pprx: ::windows::core::RawPtr, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcProxyBuffer(pub ::windows::core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
impl ::core::convert::From<IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcStubBuffer(pub ::windows::core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), punkserver.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn Invoke<'a, Param1: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)))
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
}
unsafe impl ::windows::core::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
impl ::core::convert::From<IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkserver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID) -> ::windows::core::RawPtr,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRpcSyntaxNegotiate(pub ::windows::core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
impl ::core::convert::From<IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRunnableObject(pub ::windows::core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Run<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRunning<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainedObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcontained: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), fcontained.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRunnableObject {
    type Vtable = IRunnableObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRunningObjectTable(pub ::windows::core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister)).ok()
    }
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi()).ok()
    }
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister), ::core::mem::transmute(pfiletime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    pub unsafe fn EnumRunning(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfflags: u32, punkobject: ::windows::core::RawPtr, pmkobjectname: ::windows::core::RawPtr, pdwregister: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwregister: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkobjectname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkobjectname: ::windows::core::RawPtr, ppunkobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pmkobjectname: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISequentialStream(pub ::windows::core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
impl ::core::convert::From<ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServerSecurity(pub ::windows::core::IUnknown);
impl IServerSecurity {
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    pub unsafe fn ImpersonateClient(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IServerSecurity {
    type Vtable = IServerSecurity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IServiceProvider(pub ::windows::core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::core::Interface for IServiceProvider {
    type Vtable = IServiceProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
impl ::core::convert::From<IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStdMarshalInfo(pub ::windows::core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(pclsid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IStream(pub ::windows::core::IUnknown);
impl IStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IStream> {
        let mut result__: <IStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IStream>(result__)
    }
}
unsafe impl ::windows::core::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IStream> for ::windows::core::IUnknown {
    fn from(value: IStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStream> for ::windows::core::IUnknown {
    fn from(value: &IStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISequentialStream> for &IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, libnewsize: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, grfcommitflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISupportErrorInfo(pub ::windows::core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISupportErrorInfo {
    type Vtable = ISupportErrorInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
impl ::core::convert::From<ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ISupportErrorInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ISupportErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurrogate(pub ::windows::core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    pub unsafe fn FreeSurrogate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISurrogate {
    type Vtable = ISurrogate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISurrogate> for ::windows::core::IUnknown {
    fn from(value: ISurrogate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurrogate> for ::windows::core::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISurrogateService(pub ::windows::core::IUnknown);
impl ISurrogateService {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<'a, Param1: ::windows::core::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows::core::GUID, pprocesslock: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid), ::core::mem::transmute(apptype)).ok()
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid)).ok()
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulreserved)).ok()
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(shutdowntype)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISurrogateService {
    type Vtable = ISurrogateService_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidprocessid: *const ::windows::core::GUID, pprocesslock: ::windows::core::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ulreserved: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shutdowntype: ShutdownType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronize(pub ::windows::core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISynchronize {
    type Vtable = ISynchronize_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISynchronize> for ::windows::core::IUnknown {
    fn from(value: ISynchronize) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronize> for ::windows::core::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeContainer(pub ::windows::core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeEvent(pub ::windows::core::IUnknown);
impl ISynchronizeEvent {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ph)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ISynchronizeHandle> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronizeHandle> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronizeHandle> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronizeHandle> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeHandle(pub ::windows::core::IUnknown);
impl ISynchronizeHandle {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISynchronizeMutex(pub ::windows::core::IUnknown);
impl ISynchronizeMutex {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ReleaseMutex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ISynchronize> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronize> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISynchronize> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronize> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITimeAndNoticeControl(pub ::windows::core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(res1), ::core::mem::transmute(res2)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
impl ::core::convert::From<ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, res1: u32, res2: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeComp(pub ::windows::core::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Bind<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szname: Param0, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(wflags), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pdesckind), ::core::mem::transmute(pbindptr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BindType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szname: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pptcomp)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITypeComp {
    type Vtable = ITypeComp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ITypeComp> for ::windows::core::IUnknown {
    fn from(value: ITypeComp) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeComp> for ::windows::core::IUnknown {
    fn from(value: &ITypeComp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szname: super::super::Foundation::PWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows::core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut ::core::mem::ManuallyDrop<BINDPTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, szname: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeInfo(pub ::windows::core::IUnknown);
impl ITypeInfo {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: <*mut TYPEATTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITypeComp>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: <*mut FUNCDESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: <*mut VARDESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut VARDESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc)))
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo {
    type Vtable = ITypeInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, rgbstrnames: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdocstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, pbstrmops: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypeattr: *const TYPEATTR),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeInfo2(pub ::windows::core::IUnknown);
impl ITypeInfo2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: <*mut TYPEATTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TYPEATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITypeComp>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: <*mut FUNCDESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut FUNCDESC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: <*mut VARDESC as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut VARDESC>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(pmemid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc)))
    }
    pub unsafe fn GetTypeKind(&self) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), &mut result__).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<CUSTDATA>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<CUSTDATA>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo2 {
    type Vtable = ITypeInfo2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITypeInfo2> for ITypeInfo {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ITypeInfo {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeInfo> for ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeInfo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeInfo> for &ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, rgbstrnames: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdocstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, punkouter: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, pbstrmops: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypeattr: *const TYPEATTR),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptypeflags: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, memid: i32, lcid: u32, pbstrhelpstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeLib(pub ::windows::core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: <*mut TLIBATTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr)))
    }
}
unsafe impl ::windows::core::Interface for ITypeLib {
    type Vtable = ITypeLib_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ITypeLib> for ::windows::core::IUnknown {
    fn from(value: ITypeLib) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeLib> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdocstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptlibattr: *const TLIBATTR),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeLib2(pub ::windows::core::IUnknown);
impl ITypeLib2 {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: <TYPEKIND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<TYPEKIND>(result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: <ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), &mut result__).from_abi::<ITypeInfo>(result__)
    }
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: <*mut TLIBATTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut TLIBATTR>(result__)
    }
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: <ITypeComp as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ITypeComp>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sznamebuf: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), sznamebuf.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: <VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), &mut result__).from_abi::<VARIANT>(result__)
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcuniquenames), ::core::mem::transmute(pcchuniquenames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: <CUSTDATA as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CUSTDATA>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITypeLib2 {
    type Vtable = ITypeLib2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
impl ::core::convert::From<ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: ITypeLib2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ITypeLib2> for ITypeLib {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ITypeLib {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeLib> for ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeLib> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITypeLib> for &ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeLib> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrdocstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptlibattr: *const TLIBATTR),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guid: *const ::windows::core::GUID, pvarval: *mut ::core::mem::ManuallyDrop<VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, lcid: u32, pbstrhelpstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeLibRegistration(pub ::windows::core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetLcid(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin32Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin64Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistration {
    type Vtable = ITypeLibRegistration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
impl ::core::convert::From<ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pversion: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwin32path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwin64path: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdisplayname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phelpdir: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITypeLibRegistrationReader(pub ::windows::core::IUnknown);
impl ITypeLibRegistrationReader {
    pub unsafe fn EnumTypeLibRegistrations(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: <IEnumUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
impl ::core::convert::From<ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistrationReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistrationReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUri(pub ::windows::core::IUnknown);
impl IUri {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IUri {
    type Vtable = IUri_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
impl ::core::convert::From<IUri> for ::windows::core::IUnknown {
    fn from(value: IUri) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUri> for ::windows::core::IUnknown {
    fn from(value: &IUri) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrabsoluteuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrauthority: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdisplaystring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdomain: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrextension: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrfragment: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrhost: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpassword: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpath: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrpathandquery: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrquery: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrrawuri: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrschemename: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstruserinfo: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrusername: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUriBuilder(pub ::windows::core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwuribuilderflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<IUri> {
        let mut result__: <IUri as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IUri>(result__)
    }
    pub unsafe fn SetIUri<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), piuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchfragment), ::core::mem::transmute(ppwzfragment)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchhost), ::core::mem::transmute(ppwzhost)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpassword), ::core::mem::transmute(ppwzpassword)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpath), ::core::mem::transmute(ppwzpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfhasport), ::core::mem::transmute(pdwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchquery), ::core::mem::transmute(ppwzquery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchschemename), ::core::mem::transmute(ppwzschemename)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchusername), ::core::mem::transmute(ppwzusername)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFragment<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPassword<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), fhasport.into_param().abi(), ::core::mem::transmute(dwnewvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSchemeName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpropertymask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasBeenModified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IUriBuilder {
    type Vtable = IUriBuilder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
impl ::core::convert::From<IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwpropertymask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IUrlMon(pub ::windows::core::IUnknown);
impl IUrlMon {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AsyncGetClassBits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, IBindCtx>>(
        &self,
        rclsid: *const ::windows::core::GUID,
        psztype: Param1,
        pszext: Param2,
        dwfileversionms: u32,
        dwfileversionls: u32,
        pszcodebase: Param5,
        pbc: Param6,
        dwclasscontext: u32,
        riid: *const ::windows::core::GUID,
        flags: u32,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
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
unsafe impl ::windows::core::Interface for IUrlMon {
    type Vtable = IUrlMon_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IUrlMon> for ::windows::core::IUnknown {
    fn from(value: IUrlMon) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUrlMon> for ::windows::core::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, psztype: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWaitMultiple(pub ::windows::core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IWaitMultiple {
    type Vtable = IWaitMultiple_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: u32, psync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = unsafe extern "system" fn(pexcepinfo: *mut ::core::mem::ManuallyDrop<EXCEPINFO>) -> ::windows::core::HRESULT;
pub type LPFNCANUNLOADNOW = unsafe extern "system" fn() -> ::windows::core::HRESULT;
pub type LPFNGETCLASSOBJECT = unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
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
unsafe impl ::windows::core::Abi for MEMCTX {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for MKREDUCE {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for MKSYS {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for MSHCTX {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for MSHLFLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct MULTI_QI {
    pub pIID: *mut ::windows::core::GUID,
    pub pItf: ::core::option::Option<::windows::core::IUnknown>,
    pub hr: ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for MULTI_QI {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows::core::RawPtr, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MonikerCommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkthis: Param0, pmkother: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerCommonPrefixWith(pmkthis: ::windows::core::RawPtr, pmkother: ::windows::core::RawPtr, ppmkcommon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MonikerRelativePathTo<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pmksrc: Param0, pmkdest: Param1, ppmkrelpath: *mut ::core::option::Option<IMoniker>, dwreserved: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerRelativePathTo(pmksrc: ::windows::core::RawPtr, pmkdest: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr, dwreserved: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        MonikerRelativePathTo(pmksrc.into_param().abi(), pmkdest.into_param().abi(), ::core::mem::transmute(ppmkrelpath), dwreserved.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
unsafe impl ::windows::core::Abi for PENDINGMSG {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for PENDINGTYPE {
    type Abi = Self;
}
pub type PFNCONTEXTCALL = unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        ProgIDFromCLSID(::core::mem::transmute(clsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for QUERYCONTEXT {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for REGCLS {
    type Abi = Self;
}
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for RPC_C_AUTHN_LEVEL {
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
unsafe impl ::windows::core::Abi for RPC_C_IMP_LEVEL {
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
unsafe impl ::windows::core::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for SAFEARRAY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for SERVERCALL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: super::super::Foundation::PWSTR,
    pub hr: ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for STATDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: super::super::Foundation::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: u32,
    pub grfLocksSupported: u32,
    pub clsid: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for STATSTG {
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
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
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
unsafe impl ::windows::core::Abi for STGMEDIUM {
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
    pub pstm: ::windows::core::RawPtr,
    pub pstg: ::windows::core::RawPtr,
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
unsafe impl ::windows::core::Abi for STGMEDIUM_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
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
unsafe impl ::windows::core::Abi for STGTY {
    type Abi = Self;
}
pub const STGTY_REPEAT: i32 = 256i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
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
unsafe impl ::windows::core::Abi for STREAM_SEEK {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYSKIND(pub i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::core::convert::From<i32> for SYSKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SYSKIND {
    type Abi = Self;
}
#[inline]
pub unsafe fn SetErrorInfo<'a, Param1: ::windows::core::IntoParam<'a, IErrorInfo>>(dwreserved: u32, perrinfo: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetErrorInfo(dwreserved: u32, perrinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        SetErrorInfo(::core::mem::transmute(dwreserved), perrinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
unsafe impl ::windows::core::Abi for ShutdownType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for StorageLayout {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        StringFromCLSID(::core::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
        }
        ::core::mem::transmute(StringFromGUID2(::core::mem::transmute(rguid), ::core::mem::transmute(lpsz), ::core::mem::transmute(cchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows::core::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        StringFromIID(::core::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
unsafe impl ::windows::core::Abi for THDTYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct TLIBATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl TLIBATTR {}
impl ::core::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TLIBATTR").field("guid", &self.guid).field("lcid", &self.lcid).field("syskind", &self.syskind).field("wMajorVerNum", &self.wMajorVerNum).field("wMinorVerNum", &self.wMinorVerNum).field("wLibFlags", &self.wLibFlags).finish()
    }
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.lcid == other.lcid && self.syskind == other.syskind && self.wMajorVerNum == other.wMajorVerNum && self.wMinorVerNum == other.wMinorVerNum && self.wLibFlags == other.wLibFlags
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
unsafe impl ::windows::core::Abi for TLIBATTR {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for TYMED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct TYPEATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl TYPEATTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for TYPEATTR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for TYPEATTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for TYPEATTR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TYPEKIND(pub i32);
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::core::convert::From<i32> for TYPEKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TYPEKIND {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for TYSPEC {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for URI_CREATE_FLAGS {
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
unsafe impl ::windows::core::Abi for Uri_PROPERTY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: super::super::Foundation::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut ::core::mem::ManuallyDrop<VARIANT>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
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
    pub punkVal: ::windows::core::RawPtr,
    pub pdispVal: ::windows::core::RawPtr,
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
    pub ppunkVal: *mut ::windows::core::RawPtr,
    pub ppdispVal: *mut ::windows::core::RawPtr,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::core::option::Option<super::Ole::IRecordInfo>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VARKIND(pub i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::core::convert::From<i32> for VARKIND {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VARKIND {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for WORD_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
unsafe impl ::windows::core::Abi for uCLSSPEC {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union uCLSSPEC_0 {
    pub clsid: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::super::Foundation::PWSTR,
    pub PolicyId: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::core::GUID,
    pub PolicyId: ::windows::core::GUID,
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
unsafe impl ::windows::core::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
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
unsafe impl ::windows::core::Abi for userFLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
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
unsafe impl ::windows::core::Abi for userSTGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
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
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0 {
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
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
