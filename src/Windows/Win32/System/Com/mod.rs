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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ADVF(pub i32);
pub const ADVF_NODATA: ADVF = ADVF(1i32);
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::std::convert::From<i32> for ADVF {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct APTTYPE(pub i32);
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::std::convert::From<i32> for APTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for APTTYPEQUALIFIER {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationType(pub i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::std::convert::From<i32> for ApplicationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ApplicationType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIAdviseSink(::windows::runtime::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnRename(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(336, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<AsyncIAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>),
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIAdviseSink2(::windows::runtime::IUnknown);
impl AsyncIAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnRename(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(337, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<AsyncIAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIAdviseSink2> for AsyncIAdviseSink {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AsyncIAdviseSink> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, AsyncIAdviseSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AsyncIAdviseSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, AsyncIAdviseSink> for &AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, AsyncIAdviseSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<AsyncIAdviseSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>),
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIMultiQI(::windows::runtime::IUnknown);
impl AsyncIMultiQI {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cmqis), ::std::mem::transmute(pmqis)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(917536, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<AsyncIMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmqis: u32, pmqis: *mut ::std::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmqis: *mut ::std::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIPipeByte(::windows::runtime::IUnknown);
impl AsyncIPipeByte {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const u8, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305547, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<AsyncIPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIPipeDouble(::windows::runtime::IUnknown);
impl AsyncIPipeDouble {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const f64, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305551, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<AsyncIPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIPipeLong(::windows::runtime::IUnknown);
impl AsyncIPipeLong {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Push(&self, buf: *const i32, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305549, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<AsyncIPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncIUnknown(::windows::runtime::IUnknown);
impl AsyncIUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_AddRef(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Begin_Release(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(917504, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<AsyncIUnknown> for ::windows::runtime::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AsyncIUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AsyncIUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::clone::Clone for BINDINFO {
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
    pub pUnk: ::std::option::Option<::windows::runtime::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for BINDINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for BINDINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BINDINFOF(pub i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::std::convert::From<i32> for BINDINFOF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BINDINFOF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BIND_FLAGS(pub i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::std::convert::From<i32> for BIND_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BIND_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl BIND_OPTS {}
impl ::std::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BIND_OPTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
impl ::std::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.grfFlags == other.grfFlags && self.grfMode == other.grfMode && self.dwTickCountDeadline == other.dwTickCountDeadline
    }
}
impl ::std::cmp::Eq for BIND_OPTS {}
unsafe impl ::windows::runtime::Abi for BIND_OPTS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BIND_OPTS2").field("__AnonymousBase_objidl_L9017_C36", &self.__AnonymousBase_objidl_L9017_C36).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9017_C36 == other.__AnonymousBase_objidl_L9017_C36 && self.dwTrackFlags == other.dwTrackFlags && self.dwClassContext == other.dwClassContext && self.locale == other.locale && self.pServerInfo == other.pServerInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BIND_OPTS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BIND_OPTS2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BIND_OPTS3").field("__AnonymousBase_objidl_L9041_C36", &self.__AnonymousBase_objidl_L9041_C36).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        self.__AnonymousBase_objidl_L9041_C36 == other.__AnonymousBase_objidl_L9041_C36 && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl BLOB {}
impl ::std::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
impl ::std::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pBlobData == other.pBlobData
    }
}
impl ::std::cmp::Eq for BLOB {}
unsafe impl ::windows::runtime::Abi for BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl BYTE_BLOB {}
impl ::std::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
impl ::std::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::std::cmp::Eq for BYTE_BLOB {}
unsafe impl ::windows::runtime::Abi for BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl BYTE_SIZEDARR {}
impl ::std::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for BYTE_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn BindMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(pmk: Param0, grfopt: u32, iidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindMoniker(pmk: ::windows::runtime::RawPtr, grfopt: u32, iidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        BindMoniker(pmk.into_param().abi(), ::std::mem::transmute(grfopt), ::std::mem::transmute(iidresult), ::std::mem::transmute(ppvresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CALLTYPE(pub i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::std::convert::From<i32> for CALLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CALLTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CATEGORYINFO {
    pub catid: ::windows::runtime::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl CATEGORYINFO {}
impl ::std::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
impl ::std::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.catid == other.catid && self.lcid == other.lcid && self.szDescription == other.szDescription
    }
}
impl ::std::cmp::Eq for CATEGORYINFO {}
unsafe impl ::windows::runtime::Abi for CATEGORYINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for CLSCTX {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLSCTX {
    type Abi = Self;
}
impl ::std::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszprogid: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: super::super::Foundation::PWSTR, lpclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for COAUTHIDENTITY {}
unsafe impl ::windows::runtime::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COAUTHINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pwszServerPrincName == other.pwszServerPrincName && self.dwAuthnLevel == other.dwAuthnLevel && self.dwImpersonationLevel == other.dwImpersonationLevel && self.pAuthIdentityData == other.pAuthIdentityData && self.dwCapabilities == other.dwCapabilities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COAUTHINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COAUTHINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COINIT(pub u32);
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::std::convert::From<u32> for COINIT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COINIT {
    type Abi = Self;
}
impl ::std::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COINITBASE(pub i32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::std::convert::From<i32> for COINITBASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COINITBASE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMSD(pub i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::std::convert::From<i32> for COMSD {
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
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CONNECTDATA {
    pub pUnk: ::std::option::Option<::windows::runtime::IUnknown>,
    pub dwCookie: u32,
}
impl CONNECTDATA {}
impl ::std::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CONNECTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
impl ::std::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::std::cmp::Eq for CONNECTDATA {}
unsafe impl ::windows::runtime::Abi for CONNECTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COSERVERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1 && self.pwszName == other.pwszName && self.pAuthInfo == other.pAuthInfo && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COSERVERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COSERVERINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COWAIT_FLAGS(pub i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::std::convert::From<i32> for COWAIT_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COWAIT_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl ::std::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CO_DEVICE_CATALOG_COOKIE {}
unsafe impl ::windows::runtime::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl ::std::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for CO_MTA_USAGE_COOKIE {}
unsafe impl ::windows::runtime::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl CSPLATFORM {}
impl ::std::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CSPLATFORM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
impl ::std::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        self.dwPlatformId == other.dwPlatformId && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo && self.dwProcessorArch == other.dwProcessorArch
    }
}
impl ::std::cmp::Eq for CSPLATFORM {}
unsafe impl ::windows::runtime::Abi for CSPLATFORM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CWMO_FLAGS(pub i32);
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::std::convert::From<i32> for CWMO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CWMO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl CY {}
impl ::std::default::Default for CY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CY {}
unsafe impl ::windows::runtime::Abi for CY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl CY_0 {}
impl ::std::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CY_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
impl ::std::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Lo == other.Lo && self.Hi == other.Hi
    }
}
impl ::std::cmp::Eq for CY_0 {}
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
        ::std::mem::transmute(CoAddRefServerProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0, lpvreserved: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: ::windows::runtime::RawPtr, lpvreserved: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::std::mem::transmute(lpvreserved)).ok()
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
        CoAllowUnmarshalerCLSID(::std::mem::transmute(clsid)).ok()
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
        ::std::mem::transmute(CoBuildVersion())
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
        CoCancelCall(::std::mem::transmute(dwthreadid), ::std::mem::transmute(ultimeout)).ok()
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
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
            fn CoCreateInstance(rclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclscontext: CLSCTX, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CoCreateInstance(::std::mem::transmute(rclsid), punkouter.into_param().abi(), ::std::mem::transmute(dwclscontext), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut ::std::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoCreateInstanceEx(::std::mem::transmute(clsid), punkouter.into_param().abi(), ::std::mem::transmute(dwclsctx), ::std::mem::transmute(pserverinfo), ::std::mem::transmute(dwcount), ::std::mem::transmute(presults)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(clsid: *const ::windows::runtime::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::std::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, dwclsctx: CLSCTX, reserved: *const ::std::ffi::c_void, dwcount: u32, presults: *mut ::std::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT;
        }
        CoCreateInstanceFromApp(::std::mem::transmute(clsid), punkouter.into_param().abi(), ::std::mem::transmute(dwclsctx), ::std::mem::transmute(reserved), ::std::mem::transmute(dwcount), ::std::mem::transmute(presults)).ok()
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
pub unsafe fn CoDisableCallCancellation(preserved: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisableCallCancellation(preserved: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoDisableCallCancellation(::std::mem::transmute(preserved)).ok()
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
        CoDisconnectContext(::std::mem::transmute(dwtimeout)).ok()
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
        CoDisconnectObject(punk.into_param().abi(), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CoDosDateTimeToFileTime(::std::mem::transmute(ndosdate), ::std::mem::transmute(ndostime), ::std::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoEnableCallCancellation(preserved: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoEnableCallCancellation(::std::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoFileTimeNow(&mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CoFileTimeToDosDateTime(::std::mem::transmute(lpfiletime), ::std::mem::transmute(lpdosdate), ::std::mem::transmute(lpdostime)))
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
        ::std::mem::transmute(CoFreeAllLibraries())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
        }
        ::std::mem::transmute(CoFreeLibrary(hinst.into_param().abi()))
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
        ::std::mem::transmute(CoFreeUnusedLibraries())
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
        ::std::mem::transmute(CoFreeUnusedLibrariesEx(::std::mem::transmute(dwunloaddelay), ::std::mem::transmute(dwreserved)))
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
        CoGetApartmentType(::std::mem::transmute(papttype), ::std::mem::transmute(paptqualifier)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCallContext(riid: *const ::windows::runtime::GUID, ppinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallContext(riid: *const ::windows::runtime::GUID, ppinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetCallContext(::std::mem::transmute(riid), ::std::mem::transmute(ppinterface)).ok()
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
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetCallerTID(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCancelObject(dwthreadid: u32, iid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetCancelObject(::std::mem::transmute(dwthreadid), ::std::mem::transmute(iid), ::std::mem::transmute(ppunk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetClassObject(rclsid: *const ::windows::runtime::GUID, dwclscontext: CLSCTX, pvreserved: *const ::std::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetClassObject(rclsid: *const ::windows::runtime::GUID, dwclscontext: CLSCTX, pvreserved: *const ::std::ffi::c_void, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetClassObject(::std::mem::transmute(rclsid), ::std::mem::transmute(dwclscontext), ::std::mem::transmute(pvreserved), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
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
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        ::std::mem::transmute(CoGetCurrentProcess())
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
        let mut result__: <IMalloc as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetMalloc(::std::mem::transmute(dwmemcontext), &mut result__).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: super::super::Foundation::PWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::std::mem::transmute(pbindoptions), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoGetObjectContext(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObjectContext(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoGetObjectContext(::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
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
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetPSClsid(::std::mem::transmute(riid), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::HRESULT;
        }
        CoGetSystemSecurityPermissions(::std::mem::transmute(comsdtype), ::std::mem::transmute(ppsd)).ok()
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
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetTreatAsClass(::std::mem::transmute(clsidold), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
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
        let mut result__: <CO_MTA_USAGE_COOKIE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoIncrementMTAUsage(&mut result__).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoInitialize(pvreserved: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitialize(pvreserved: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoInitialize(::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: *const ::std::ffi::c_void, dwcoinit: COINIT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeEx(pvreserved: *const ::std::ffi::c_void, dwcoinit: COINIT) -> ::windows::runtime::HRESULT;
        }
        CoInitializeEx(::std::mem::transmute(pvreserved), ::std::mem::transmute(dwcoinit)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Security`*"]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::std::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::std::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CoInitializeSecurity(
            ::std::mem::transmute(psecdesc),
            ::std::mem::transmute(cauthsvc),
            ::std::mem::transmute(asauthsvc),
            ::std::mem::transmute(preserved1),
            ::std::mem::transmute(dwauthnlevel),
            ::std::mem::transmute(dwimplevel),
            ::std::mem::transmute(pauthlist),
            ::std::mem::transmute(dwcapabilities),
            ::std::mem::transmute(preserved3),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows::runtime::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(pclassspec), ::std::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
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
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punk: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsOle1Class(rclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CoIsOle1Class(::std::mem::transmute(rclsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLoadLibrary(lpszlibname: super::super::Foundation::PWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(CoLoadLibrary(lpszlibname.into_param().abi(), bautofree.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
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
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> ::windows::runtime::HRESULT;
        }
        CoQueryAuthenticationServices(::std::mem::transmute(pcauthsvc), ::std::mem::transmute(asauthsvc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::std::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::std::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CoQueryClientBlanket(::std::mem::transmute(pauthnsvc), ::std::mem::transmute(pauthzsvc), ::std::mem::transmute(pserverprincname), ::std::mem::transmute(pauthnlevel), ::std::mem::transmute(pimplevel), ::std::mem::transmute(pprivs), ::std::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::std::ffi::c_void, pcapabilites: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: ::windows::runtime::RawPtr, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut super::super::Foundation::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::std::ffi::c_void, pcapabilites: *mut u32) -> ::windows::runtime::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::std::mem::transmute(pwauthnsvc), ::std::mem::transmute(pauthzsvc), ::std::mem::transmute(pserverprincname), ::std::mem::transmute(pauthnlevel), ::std::mem::transmute(pimplevel), ::std::mem::transmute(pauthinfo), ::std::mem::transmute(pcapabilites)).ok()
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
        CoRegisterChannelHook(::std::mem::transmute(extensionuuid), pchannelhook.into_param().abi()).ok()
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
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoRegisterClassObject(::std::mem::transmute(rclsid), punk.into_param().abi(), ::std::mem::transmute(dwclscontext), ::std::mem::transmute(flags), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(deviceinstanceid: Param0) -> ::windows::runtime::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: super::super::Foundation::PWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <CO_DEVICE_CATALOG_COOKIE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        CoRegisterPSClsid(::std::mem::transmute(riid), ::std::mem::transmute(rclsid)).ok()
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
        ::std::mem::transmute(CoReleaseServerProcess())
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
        CoRevokeClassObject(::std::mem::transmute(dwregister)).ok()
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
        CoRevokeInitializeSpy(::std::mem::transmute(ulicookie)).ok()
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
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: ::windows::runtime::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::std::mem::transmute(dwauthnsvc), ::std::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::std::mem::transmute(dwauthnlevel), ::std::mem::transmute(dwimplevel), ::std::mem::transmute(pauthinfo), ::std::mem::transmute(dwcapabilities)).ok()
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
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemAlloc(cb: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(CoTaskMemAlloc(::std::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemFree(pv: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(CoTaskMemFree(::std::mem::transmute(pv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: *const ::std::ffi::c_void, cb: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemRealloc(pv: *const ::std::ffi::c_void, cb: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(CoTaskMemRealloc(::std::mem::transmute(pv), ::std::mem::transmute(cb)))
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
        CoTreatAsClass(::std::mem::transmute(clsidold), ::std::mem::transmute(clsidnew)).ok()
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
        ::std::mem::transmute(CoUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoWaitForMultipleHandles(::std::mem::transmute(dwflags), ::std::mem::transmute(dwtimeout), ::std::mem::transmute(chandles), ::std::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoWaitForMultipleObjects(::std::mem::transmute(dwflags), ::std::mem::transmute(dwtimeout), ::std::mem::transmute(chandles), ::std::mem::transmute(phandles), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::std::ffi::c_void,
}
impl ComCallData {}
impl ::std::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ComCallData {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
impl ::std::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        self.dwDispid == other.dwDispid && self.dwReserved == other.dwReserved && self.pUserDefined == other.pUserDefined
    }
}
impl ::std::cmp::Eq for ComCallData {}
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IBindCtx as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateBindCtx(::std::mem::transmute(reserved), &mut result__).from_abi::<IBindCtx>(result__)
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateClassMoniker(::std::mem::transmute(rclsid), &mut result__).from_abi::<IMoniker>(result__)
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
        let mut result__: <IDataAdviseHolder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateDataAdviseHolder(&mut result__).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkouter: Param0, rclsid: *const ::windows::runtime::GUID, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::std::mem::transmute(rclsid), ::std::mem::transmute(iid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszpathname: Param0) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IUriBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows::runtime::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: super::super::Foundation::PWSTR, lpszitem: super::super::Foundation::PWSTR, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows::runtime::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pibsccaller: ::windows::runtime::RawPtr, ppibsc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IBindStatusCallback as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), &mut result__).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: super::super::Foundation::PWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: super::super::Foundation::PSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::std::mem::transmute(dwencodingflags), ::std::mem::transmute(dwcodepage), ::std::mem::transmute(dwcreateflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: super::super::Foundation::PWSTR, pwzfragment: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DATADIR(pub i32);
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::std::convert::From<i32> for DATADIR {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOM_CALL_STATE(pub i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::std::convert::From<i32> for DCOM_CALL_STATE {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DVASPECT(pub i32);
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1i32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2i32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4i32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8i32);
impl ::std::convert::From<i32> for DVASPECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DVASPECT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.tdSize == other.tdSize && self.tdDriverNameOffset == other.tdDriverNameOffset && self.tdDeviceNameOffset == other.tdDeviceNameOffset && self.tdPortNameOffset == other.tdPortNameOffset && self.tdExtDevmodeOffset == other.tdExtDevmodeOffset && self.tdData == other.tdData
    }
}
impl ::std::cmp::Eq for DVTARGETDEVICE {}
unsafe impl ::windows::runtime::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl DWORD_BLOB {}
impl ::std::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
impl ::std::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.alData == other.alData
    }
}
impl ::std::cmp::Eq for DWORD_BLOB {}
unsafe impl ::windows::runtime::Abi for DWORD_BLOB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: *const ::std::ffi::c_void, pulreserved: *const u32, appshr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DcomChannelSetHResult(pvreserved: *const ::std::ffi::c_void, pulreserved: *const u32, appshr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        DcomChannelSetHResult(::std::mem::transmute(pvreserved), ::std::mem::transmute(pulreserved), ::std::mem::transmute(appshr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EOLE_AUTHENTICATION_CAPABILITIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EXTCONN(pub i32);
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::std::convert::From<i32> for EXTCONN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXTCONN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl FLAGGED_BYTE_BLOB {}
impl ::std::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
impl ::std::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.abData == other.abData
    }
}
impl ::std::cmp::Eq for FLAGGED_BYTE_BLOB {}
unsafe impl ::windows::runtime::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl FLAGGED_WORD_BLOB {}
impl ::std::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
impl ::std::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.fFlags == other.fFlags && self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::std::cmp::Eq for FLAGGED_WORD_BLOB {}
unsafe impl ::windows::runtime::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::clone::Clone for FLAG_STGMEDIUM {
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
impl ::std::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for FLAG_STGMEDIUM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FORMATETC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
impl ::std::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::std::cmp::Eq for FORMATETC {}
unsafe impl ::windows::runtime::Abi for FORMATETC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::std::convert::From<i32> for GLOBALOPT_EH_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GLOBALOPT_PROPERTIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for GLOBALOPT_RO_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
impl ::std::convert::From<i32> for GLOBALOPT_RPCTP_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::std::convert::From<i32> for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
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
        let mut result__: <IRunningObjectTable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetRunningObjectTable(::std::mem::transmute(reserved), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl HYPER_SIZEDARR {}
impl ::std::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for HYPER_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IActivationFilter(::windows::runtime::IUnknown);
impl IActivationFilter {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwactivationtype), ::std::mem::transmute(rclsid), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IActivationFilter {
    type Vtable = IActivationFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(23, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IActivationFilter> for ::windows::runtime::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IActivationFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IActivationFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAddrExclusionControl(::windows::runtime::IUnknown);
impl IAddrExclusionControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::runtime::GUID, ppenumerator: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppenumerator)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, penumerator: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), penumerator.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(328, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAddrExclusionControl> for ::windows::runtime::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAddrExclusionControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAddrExclusionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppenumerator: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, penumerator: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAddrTrackingControl(::windows::runtime::IUnknown);
impl IAddrTrackingControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(327, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAddrTrackingControl> for ::windows::runtime::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAddrTrackingControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAddrTrackingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAdviseSink(::windows::runtime::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IAdviseSink {
    type Vtable = IAdviseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(271, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAdviseSink2(::windows::runtime::IUnknown);
impl IAdviseSink2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwaspect), ::std::mem::transmute(lindex)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnSave(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnClose(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pmk.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(293, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAdviseSink2> for ::windows::runtime::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IAdviseSink2> for IAdviseSink {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAdviseSink2> for IAdviseSink {
    fn from(value: &IAdviseSink2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdviseSink> for IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdviseSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAdviseSink>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAdviseSink> for &IAdviseSink2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAdviseSink> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IAdviseSink>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwaspect: u32, lindex: i32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAgileObject(::windows::runtime::IUnknown);
impl IAgileObject {}
unsafe impl ::windows::runtime::Interface for IAgileObject {
    type Vtable = IAgileObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2498374548, 59852, 18912, [192, 255, 238, 100, 202, 143, 91, 144]);
}
impl ::std::convert::From<IAgileObject> for ::windows::runtime::IUnknown {
    fn from(value: IAgileObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAgileObject> for ::windows::runtime::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAgileObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAsyncManager(::windows::runtime::IUnknown);
impl IAsyncManager {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CompleteCall(&self, result: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(result)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAsyncManager {
    type Vtable = IAsyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(42, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAsyncManager> for ::windows::runtime::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAsyncManager> for ::windows::runtime::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAsyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulstateflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAsyncRpcChannelBuffer(::windows::runtime::IUnknown);
impl IAsyncRpcChannelBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Send<'a, Param1: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), psync.into_param().abi(), ::std::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2768412598, 15412, 4561, [156, 153, 0, 192, 79, 185, 152, 170]);
}
impl ::std::convert::From<IAsyncRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAsyncRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAsyncRpcChannelBuffer> for IRpcChannelBuffer {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::runtime::RawPtr, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBindCtx(::windows::runtime::IUnknown);
impl IBindCtx {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::runtime::Result<IRunningObjectTable> {
        let mut result__: <IRunningObjectTable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszkey.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumObjectParam(&self) -> ::windows::runtime::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszkey: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pszkey.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindCtx {
    type Vtable = IBindCtx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(14, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IBindCtx> for ::windows::runtime::IUnknown {
    fn from(value: IBindCtx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindCtx> for ::windows::runtime::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindCtx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBindHost(::windows::runtime::IUnknown);
impl IBindHost {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn CreateMoniker<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::std::option::Option<IMoniker>, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), szname.into_param().abi(), pbc.into_param().abi(), ::std::mem::transmute(ppmk), ::std::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IBindCtx>, Param2: ::windows::runtime::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindHost {
    type Vtable = IBindHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4232577441, 11177, 4559, [162, 41, 0, 170, 0, 61, 115, 82]);
}
impl ::std::convert::From<IBindHost> for ::windows::runtime::IUnknown {
    fn from(value: IBindHost) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindHost> for ::windows::runtime::IUnknown {
    fn from(value: &IBindHost) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindHost {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pbsc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmk: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pbsc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBindStatusCallback(::windows::runtime::IUnknown);
impl IBindStatusCallback {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::runtime::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulprogress), ::std::mem::transmute(ulprogressmax), ::std::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::runtime::HRESULT, szerror: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbindf), ::std::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbscf), ::std::mem::transmute(dwsize), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045430209, 47865, 4558, [140, 130, 0, 170, 0, 75, 169, 11]);
}
impl ::std::convert::From<IBindStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::std::mem::ManuallyDrop<BINDINFO>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBindStatusCallbackEx(::windows::runtime::IUnknown);
impl IBindStatusCallbackEx {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::runtime::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulprogress), ::std::mem::transmute(ulprogressmax), ::std::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, hresult: ::windows::runtime::HRESULT, szerror: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbindf), ::std::mem::transmute(pbindinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbscf), ::std::mem::transmute(dwsize), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfbindf), ::std::mem::transmute(pbindinfo), ::std::mem::transmute(grfbindf2), ::std::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2863091449, 36583, 18009, [136, 217, 248, 197, 4, 218, 115, 204]);
}
impl ::std::convert::From<IBindStatusCallbackEx> for ::windows::runtime::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindStatusCallbackEx> for ::windows::runtime::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBindStatusCallbackEx> for IBindStatusCallback {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindStatusCallback> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBindStatusCallback>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBindStatusCallback> for &IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBindStatusCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IBindStatusCallback>::into(::std::clone::Clone::clone(self)))
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
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::std::mem::ManuallyDrop<BINDINFO>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const ::std::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfbindf: *mut u32, pbindinfo: *mut ::std::mem::ManuallyDrop<BINDINFO>, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBinding(::windows::runtime::IUnknown);
impl IBinding {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Abort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Suspend(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(npriority)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::runtime::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pclsidprotocol), ::std::mem::transmute(pdwresult), ::std::mem::transmute(pszresult), ::std::mem::transmute(pdwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBinding {
    type Vtable = IBinding_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2045430208, 47865, 4558, [140, 130, 0, 170, 0, 75, 169, 11]);
}
impl ::std::convert::From<IBinding> for ::windows::runtime::IUnknown {
    fn from(value: IBinding) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBinding> for ::windows::runtime::IUnknown {
    fn from(value: &IBinding) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBinding {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IBlockingLock(::windows::runtime::IUnknown);
impl IBlockingLock {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unlock(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IBlockingLock {
    type Vtable = IBlockingLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(821286010, 25671, 4561, [142, 60, 0, 192, 79, 185, 56, 109]);
}
impl ::std::convert::From<IBlockingLock> for ::windows::runtime::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBlockingLock> for ::windows::runtime::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBlockingLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICallFactory(::windows::runtime::IUnknown);
impl ICallFactory {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateCall<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, pctrlunk: Param1, riid2: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), pctrlunk.into_param().abi(), ::std::mem::transmute(riid2), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICallFactory {
    type Vtable = ICallFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(477313584, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
impl ::std::convert::From<ICallFactory> for ::windows::runtime::IUnknown {
    fn from(value: ICallFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICallFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICallFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICancelMethodCalls(::windows::runtime::IUnknown);
impl ICancelMethodCalls {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn TestCancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(41, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICancelMethodCalls> for ::windows::runtime::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICancelMethodCalls> for ::windows::runtime::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICancelMethodCalls {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICatInformation(::windows::runtime::IUnknown);
impl ICatInformation {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::runtime::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcid), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::runtime::GUID, lcid: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rcatid), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(cimplemented), ::std::mem::transmute(rgcatidimpl), ::std::mem::transmute(crequired), ::std::mem::transmute(rgcatidreq), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::runtime::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::runtime::GUID, crequired: u32, rgcatidreq: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(cimplemented), ::std::mem::transmute(rgcatidimpl), ::std::mem::transmute(crequired), ::std::mem::transmute(rgcatidreq)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICatInformation {
    type Vtable = ICatInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188435, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICatInformation> for ::windows::runtime::IUnknown {
    fn from(value: ICatInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICatInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICatInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICatRegister(::windows::runtime::IUnknown);
impl ICatRegister {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcategoryinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcatid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::runtime::GUID, ccategories: u32, rgcatid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(ccategories), ::std::mem::transmute(rgcatid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICatRegister {
    type Vtable = ICatRegister_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188434, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ICatRegister> for ::windows::runtime::IUnknown {
    fn from(value: ICatRegister) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICatRegister> for ::windows::runtime::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICatRegister {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IChannelHook(::windows::runtime::IUnknown);
impl IChannelHook {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(pdatasize)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(pdatasize), ::std::mem::transmute(pdatabuffer)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::std::ffi::c_void, ldatarep: u32, hrfault: ::windows::runtime::HRESULT) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(cbdatasize), ::std::mem::transmute(pdatabuffer), ::std::mem::transmute(ldatarep), ::std::mem::transmute(hrfault)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::std::ffi::c_void, ldatarep: u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(cbdatasize), ::std::mem::transmute(pdatabuffer), ::std::mem::transmute(ldatarep)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, hrfault: ::windows::runtime::HRESULT, pdatasize: *mut u32) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(hrfault), ::std::mem::transmute(pdatasize)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::std::ffi::c_void, hrfault: ::windows::runtime::HRESULT) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(uextent), ::std::mem::transmute(riid), ::std::mem::transmute(pdatasize), ::std::mem::transmute(pdatabuffer), ::std::mem::transmute(hrfault)))
    }
}
unsafe impl ::windows::runtime::Interface for IChannelHook {
    type Vtable = IChannelHook_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(269010080, 30227, 4559, [154, 241, 0, 32, 175, 110, 114, 244]);
}
impl ::std::convert::From<IChannelHook> for ::windows::runtime::IUnknown {
    fn from(value: IChannelHook) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IChannelHook> for ::windows::runtime::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IChannelHook {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::std::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::std::ffi::c_void, ldatarep: u32, hrfault: ::windows::runtime::HRESULT),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, cbdatasize: u32, pdatabuffer: *const ::std::ffi::c_void, ldatarep: u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, hrfault: ::windows::runtime::HRESULT, pdatasize: *mut u32),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uextent: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, pdatasize: *mut u32, pdatabuffer: *const ::std::ffi::c_void, hrfault: ::windows::runtime::HRESULT),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClassActivator(::windows::runtime::IUnknown);
impl IClassActivator {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassObject<T: ::windows::runtime::Interface>(&self, rclsid: *const ::windows::runtime::GUID, dwclasscontext: u32, locale: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(rclsid), ::std::mem::transmute(dwclasscontext), ::std::mem::transmute(locale), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IClassActivator {
    type Vtable = IClassActivator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(320, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IClassActivator> for ::windows::runtime::IUnknown {
    fn from(value: IClassActivator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IClassActivator> for ::windows::runtime::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IClassActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClassFactory(::windows::runtime::IUnknown);
impl IClassFactory {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(&self, punkouter: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn LockServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClassFactory {
    type Vtable = IClassFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IClassFactory> for ::windows::runtime::IUnknown {
    fn from(value: IClassFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IClassFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IClassFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flock: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClientSecurity(::windows::runtime::IUnknown);
impl IClientSecurity {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::std::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pproxy.into_param().abi(),
            ::std::mem::transmute(pauthnsvc),
            ::std::mem::transmute(pauthzsvc),
            ::std::mem::transmute(pserverprincname),
            ::std::mem::transmute(pauthnlevel),
            ::std::mem::transmute(pimplevel),
            ::std::mem::transmute(pauthinfo),
            ::std::mem::transmute(pcapabilites),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetBlanket<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pproxy.into_param().abi(),
            ::std::mem::transmute(dwauthnsvc),
            ::std::mem::transmute(dwauthzsvc),
            pserverprincname.into_param().abi(),
            ::std::mem::transmute(dwauthnlevel),
            ::std::mem::transmute(dwimplevel),
            ::std::mem::transmute(pauthinfo),
            ::std::mem::transmute(dwcapabilities),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CopyProxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pproxy: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pproxy.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IClientSecurity {
    type Vtable = IClientSecurity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(317, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IClientSecurity> for ::windows::runtime::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IClientSecurity> for ::windows::runtime::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IClientSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::std::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::std::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproxy: ::windows::runtime::RawPtr, ppcopy: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IComThreadingInfo(::windows::runtime::IUnknown);
impl IComThreadingInfo {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::runtime::Result<APTTYPE> {
        let mut result__: <APTTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<APTTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::runtime::Result<THDTYPE> {
        let mut result__: <THDTYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<THDTYPE>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(462, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IComThreadingInfo> for ::windows::runtime::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IComThreadingInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IComThreadingInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IConnectionPoint(::windows::runtime::IUnknown);
impl IConnectionPoint {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::runtime::Result<IConnectionPointContainer> {
        let mut result__: <IConnectionPointContainer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IConnectionPointContainer>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punksink: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), punksink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumConnections(&self) -> ::windows::runtime::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnectionPoint {
    type Vtable = IConnectionPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443334, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IConnectionPoint> for ::windows::runtime::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConnectionPoint> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConnectionPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IConnectionPointContainer(::windows::runtime::IUnknown);
impl IConnectionPointContainer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::runtime::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IConnectionPoint> {
        let mut result__: <IConnectionPoint as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), &mut result__).from_abi::<IConnectionPoint>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443332, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IConnectionPointContainer> for ::windows::runtime::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IConnectionPointContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IConnectionPointContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IContext(pub u8);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IContextCallback(::windows::runtime::IUnknown);
impl IContextCallback {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ContextCallback<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfncallback: ::std::option::Option<PFNCONTEXTCALL>, pparam: *const ComCallData, riid: *const ::windows::runtime::GUID, imethod: i32, punk: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfncallback), ::std::mem::transmute(pparam), ::std::mem::transmute(riid), ::std::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContextCallback {
    type Vtable = IContextCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(474, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IContextCallback> for ::windows::runtime::IUnknown {
    fn from(value: IContextCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContextCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContextCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDataAdviseHolder(::windows::runtime::IUnknown);
impl IDataAdviseHolder {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, IDataObject>, Param3: ::windows::runtime::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pdataobject.into_param().abi(), ::std::mem::transmute(pfetc), ::std::mem::transmute(advf), padvise.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows::runtime::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdataobject.into_param().abi(), ::std::mem::transmute(dwreserved), ::std::mem::transmute(advf)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(272, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDataAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDataAdviseHolder> for ::windows::runtime::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDataAdviseHolder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDataObject(::windows::runtime::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::runtime::Result<STGMEDIUM> {
        let mut result__: <STGMEDIUM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetcin), &mut result__).from_abi::<STGMEDIUM>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pmedium)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC) -> ::windows::runtime::Result<FORMATETC> {
        let mut result__: <FORMATETC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatectin), &mut result__).from_abi::<FORMATETC>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn SetData<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::runtime::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdirection), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DAdvise<'a, Param2: ::windows::runtime::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pformatetc), ::std::mem::transmute(advf), padvsink.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumDAdvise(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDataObject {
    type Vtable = IDataObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(270, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IDataObject> for ::windows::runtime::IUnknown {
    fn from(value: IDataObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDataObject> for ::windows::runtime::IUnknown {
    fn from(value: &IDataObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDataObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetcin: *const FORMATETC, pmedium: *mut ::std::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pmedium: *mut ::std::mem::ManuallyDrop<STGMEDIUM>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, pmedium: *const ::std::mem::ManuallyDrop<STGMEDIUM>, frelease: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdirection: u32, ppenumformatetc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::runtime::RawPtr, pdwconnection: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconnection: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenumadvise: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumCATEGORYINFO(::windows::runtime::IUnknown);
impl IEnumCATEGORYINFO {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumCATEGORYINFO> {
        let mut result__: <IEnumCATEGORYINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188433, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumCATEGORYINFO> for ::windows::runtime::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumCATEGORYINFO> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumConnectionPoints(::windows::runtime::IUnknown);
impl IEnumConnectionPoints {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, cconnections: u32, ppcp: *mut ::std::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cconnections), ::std::mem::transmute(ppcp), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumConnectionPoints> {
        let mut result__: <IEnumConnectionPoints as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnectionPoints>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443333, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IEnumConnectionPoints> for ::windows::runtime::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumConnectionPoints> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumConnectionPoints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumConnections(::windows::runtime::IUnknown);
impl IEnumConnections {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cconnections), ::std::mem::transmute(rgcd), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumConnections> {
        let mut result__: <IEnumConnections as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumConnections>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumConnections {
    type Vtable = IEnumConnections_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2979443335, 47796, 4122, [182, 156, 0, 170, 0, 52, 29, 7]);
}
impl ::std::convert::From<IEnumConnections> for ::windows::runtime::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumConnections> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32, rgcd: *mut ::std::mem::ManuallyDrop<CONNECTDATA>, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnections: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct IEnumContextProps(pub u8);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumFORMATETC(::windows::runtime::IUnknown);
impl IEnumFORMATETC {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumFORMATETC> {
        let mut result__: <IEnumFORMATETC as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFORMATETC>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(259, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumFORMATETC> for ::windows::runtime::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumFORMATETC> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumFORMATETC {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumGUID(::windows::runtime::IUnknown);
impl IEnumGUID {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::windows::runtime::GUID, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumGUID> {
        let mut result__: <IEnumGUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumGUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumGUID {
    type Vtable = IEnumGUID_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188416, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumGUID> for ::windows::runtime::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumGUID> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumGUID {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumMoniker(::windows::runtime::IUnknown);
impl IEnumMoniker {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumMoniker {
    type Vtable = IEnumMoniker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(258, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumMoniker> for ::windows::runtime::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumMoniker> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSTATDATA(::windows::runtime::IUnknown);
impl IEnumSTATDATA {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSTATDATA> {
        let mut result__: <IEnumSTATDATA as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSTATDATA>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(261, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumSTATDATA> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumSTATDATA> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumSTATDATA {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, rgelt: *mut ::std::mem::ManuallyDrop<STATDATA>, pceltfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumString(::windows::runtime::IUnknown);
impl IEnumString {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumString> {
        let mut result__: <IEnumString as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumString>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumString {
    type Vtable = IEnumString_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(257, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumString> for ::windows::runtime::IUnknown {
    fn from(value: IEnumString) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumString> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumString) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumString {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumUnknown(::windows::runtime::IUnknown);
impl IEnumUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::std::option::Option<::windows::runtime::IUnknown>, pceltfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(rgelt), ::std::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumUnknown> {
        let mut result__: <IEnumUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumUnknown {
    type Vtable = IEnumUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(256, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IEnumUnknown> for ::windows::runtime::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IExternalConnection(::windows::runtime::IUnknown);
impl IExternalConnection {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(extconn), ::std::mem::transmute(reserved)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(extconn), ::std::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
unsafe impl ::windows::runtime::Interface for IExternalConnection {
    type Vtable = IExternalConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(25, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IExternalConnection> for ::windows::runtime::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IExternalConnection> for ::windows::runtime::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IExternalConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFastRundown(::windows::runtime::IUnknown);
impl IFastRundown {}
unsafe impl ::windows::runtime::Interface for IFastRundown {
    type Vtable = IFastRundown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(64, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IFastRundown> for ::windows::runtime::IUnknown {
    fn from(value: IFastRundown) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFastRundown> for ::windows::runtime::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFastRundown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IForegroundTransfer(::windows::runtime::IUnknown);
impl IForegroundTransfer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lpvreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransfer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(325, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IForegroundTransfer> for ::windows::runtime::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IForegroundTransfer> for ::windows::runtime::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IForegroundTransfer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGlobalInterfaceTable(::windows::runtime::IUnknown);
impl IGlobalInterfaceTable {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punk.into_param().abi(), ::std::mem::transmute(riid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcookie), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(326, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IGlobalInterfaceTable> for ::windows::runtime::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGlobalInterfaceTable> for ::windows::runtime::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcookie: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGlobalOptions(::windows::runtime::IUnknown);
impl IGlobalOptions {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwproperty), ::std::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGlobalOptions {
    type Vtable = IGlobalOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IGlobalOptions> for ::windows::runtime::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGlobalOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGlobalOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsz: Param0) -> ::windows::runtime::Result<::windows::runtime::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: super::super::Foundation::PWSTR, lpiid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        IIDFromString(lpsz.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInitializeSpy(::windows::runtime::IUnknown);
impl IInitializeSpy {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcoinit), ::std::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::runtime::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrcoinit), ::std::mem::transmute(dwcoinit), ::std::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInitializeSpy {
    type Vtable = IInitializeSpy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(52, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IInitializeSpy> for ::windows::runtime::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInitializeSpy> for ::windows::runtime::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInitializeSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInternalUnknown(::windows::runtime::IUnknown);
impl IInternalUnknown {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInternalUnknown {
    type Vtable = IInternalUnknown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(33, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IInternalUnknown> for ::windows::runtime::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInternalUnknown> for ::windows::runtime::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInternalUnknown {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMachineGlobalObjectTable(::windows::runtime::IUnknown);
impl IMachineGlobalObjectTable {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn RegisterObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, clsid: *const ::windows::runtime::GUID, identifier: Param1, object: Param2) -> ::windows::runtime::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__: <*mut MachineGlobalObjectTableRegistrationToken__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), &mut result__).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, clsid: *const ::windows::runtime::GUID, identifier: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(token)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(651626924, 63243, 17441, [169, 111, 210, 135, 143, 175, 176, 13]);
}
impl ::std::convert::From<IMachineGlobalObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMachineGlobalObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsid: *const ::windows::runtime::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMalloc(::windows::runtime::IUnknown);
impl IMalloc {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cb)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Realloc(&self, pv: *const ::std::ffi::c_void, cb: usize) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Free(&self, pv: *const ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSize(&self, pv: *const ::std::ffi::c_void) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DidAlloc(&self, pv: *const ::std::ffi::c_void) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn HeapMinimize(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IMalloc {
    type Vtable = IMalloc_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMalloc> for ::windows::runtime::IUnknown {
    fn from(value: IMalloc) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMalloc> for ::windows::runtime::IUnknown {
    fn from(value: &IMalloc) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMalloc {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cb: usize) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: usize) -> *mut ::std::ffi::c_void,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void) -> i32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMallocSpy(::windows::runtime::IUnknown);
impl IMallocSpy {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbrequest)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostAlloc(&self, pactual: *const ::std::ffi::c_void) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pactual)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreFree<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::std::ffi::c_void, fspyed: Param1) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostFree<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fspyed: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreRealloc<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::std::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::std::ffi::c_void, fspyed: Param3) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequest), ::std::mem::transmute(cbrequest), ::std::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostRealloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pactual: *const ::std::ffi::c_void, fspyed: Param1) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreGetSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::std::ffi::c_void, fspyed: Param1) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostGetSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::std::ffi::c_void, fspyed: Param1) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::std::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(prequest), fspyed.into_param().abi(), ::std::mem::transmute(factual)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PreHeapMinimize(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn PostHeapMinimize(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IMallocSpy {
    type Vtable = IMallocSpy_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(29, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMallocSpy> for ::windows::runtime::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMallocSpy> for ::windows::runtime::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMallocSpy {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbrequest: usize) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactual: *const ::std::ffi::c_void) -> *mut ::std::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::std::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::std::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactual: *const ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::std::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::std::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::std::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prequest: *const ::std::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMoniker(::windows::runtime::IUnknown);
impl IMoniker {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn BindToObject<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::std::mem::transmute(riidresult), ::std::mem::transmute(ppvresult)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn BindToStorage<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reduce<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::std::option::Option<IMoniker>, ppmkreduced: *mut ::std::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pbc.into_param().abi(), ::std::mem::transmute(dwreducehowfar), ::std::mem::transmute(ppmktoleft), ::std::mem::transmute(ppmkreduced)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ComposeWith<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Enum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforward: Param0) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), fforward.into_param().abi(), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pmkothermoniker.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Hash(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param2: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Inverse(&self) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RelativePathTo<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::runtime::Result<IMoniker> {
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::std::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::std::mem::transmute(pcheaten), ::std::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMoniker {
    type Vtable = IMoniker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMoniker> for ::windows::runtime::IUnknown {
    fn from(value: IMoniker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMoniker> for ::windows::runtime::IUnknown {
    fn from(value: &IMoniker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMoniker> for IPersistStream {
    fn from(value: IMoniker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMoniker> for IPersistStream {
    fn from(value: &IMoniker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistStream> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersistStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersistStream> for &IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersistStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersistStream>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMoniker> for IPersist {
    fn from(value: IMoniker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMoniker> for IPersist {
    fn from(value: &IMoniker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IMoniker {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, riidresult: *const ::windows::runtime::GUID, ppvresult: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbc: ::windows::runtime::RawPtr, pmktoleft: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMultiQI(::windows::runtime::IUnknown);
impl IMultiQI {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cmqis), ::std::mem::transmute(pmqis)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMultiQI {
    type Vtable = IMultiQI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(32, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: IMultiQI) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMultiQI> for ::windows::runtime::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMultiQI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmqis: u32, pmqis: *mut ::std::mem::ManuallyDrop<MULTI_QI>) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct INTERFACEINFO {
    pub pUnk: ::std::option::Option<::windows::runtime::IUnknown>,
    pub iid: ::windows::runtime::GUID,
    pub wMethod: u16,
}
impl INTERFACEINFO {}
impl ::std::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
impl ::std::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::std::cmp::Eq for INTERFACEINFO {}
unsafe impl ::windows::runtime::Abi for INTERFACEINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct INoMarshal(::windows::runtime::IUnknown);
impl INoMarshal {}
unsafe impl ::windows::runtime::Interface for INoMarshal {
    type Vtable = INoMarshal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3972557083, 49627, 19904, [133, 94, 101, 246, 197, 81, 175, 73]);
}
impl ::std::convert::From<INoMarshal> for ::windows::runtime::IUnknown {
    fn from(value: INoMarshal) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&INoMarshal> for ::windows::runtime::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &INoMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOplockStorage(::windows::runtime::IUnknown);
impl IOplockStorage {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(stgfmt), ::std::mem::transmute(grfattrs), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwcsname.into_param().abi(), ::std::mem::transmute(grfmode), ::std::mem::transmute(stgfmt), ::std::mem::transmute(grfattrs), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IOplockStorage {
    type Vtable = IOplockStorage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2367277108, 34937, 4561, [131, 233, 0, 192, 79, 194, 198, 212]);
}
impl ::std::convert::From<IOplockStorage> for ::windows::runtime::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOplockStorage> for ::windows::runtime::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IOplockStorage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::runtime::GUID, ppstgopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::runtime::GUID, ppstgopen: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPSFactoryBuffer(::windows::runtime::IUnknown);
impl IPSFactoryBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateProxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows::runtime::GUID, ppproxy: *mut ::std::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkouter.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppproxy), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateStub<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, riid: *const ::windows::runtime::GUID, punkserver: Param1) -> ::windows::runtime::Result<IRpcStubBuffer> {
        let mut result__: <IRpcStubBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), punkserver.into_param().abi(), &mut result__).from_abi::<IRpcStubBuffer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630416, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::std::convert::From<IPSFactoryBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPSFactoryBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPSFactoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppproxy: *mut ::windows::runtime::RawPtr, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punkserver: ::windows::runtime::RawPtr, ppstub: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersist(::windows::runtime::IUnknown);
impl IPersist {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersist {
    type Vtable = IPersist_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(268, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPersist> for ::windows::runtime::IUnknown {
    fn from(value: IPersist) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersist> for ::windows::runtime::IUnknown {
    fn from(value: &IPersist) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPersist {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistFile(::windows::runtime::IUnknown);
impl IPersistFile {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszfilename.into_param().abi(), ::std::mem::transmute(dwmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszfilename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetCurFile(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersistFile {
    type Vtable = IPersistFile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(267, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPersistFile> for IPersist {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistFile> for IPersist {
    fn from(value: &IPersistFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPersistStream(::windows::runtime::IUnknown);
impl IPersistStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Save<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPersistStream {
    type Vtable = IPersistStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IPersistStream> for ::windows::runtime::IUnknown {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistStream> for ::windows::runtime::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IPersistStream> for IPersist {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPersistStream> for IPersist {
    fn from(value: &IPersistStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPersist> for &IPersistStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPersist> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPersist>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPipeByte(::windows::runtime::IUnknown);
impl IPipeByte {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(crequest), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const u8, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeByte {
    type Vtable = IPipeByte_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305546, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<IPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: IPipeByte) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPipeByte> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPipeByte {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPipeDouble(::windows::runtime::IUnknown);
impl IPipeDouble {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(crequest), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const f64, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeDouble {
    type Vtable = IPipeDouble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305550, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<IPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPipeDouble> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPipeDouble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPipeLong(::windows::runtime::IUnknown);
impl IPipeLong {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(crequest), ::std::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Push(&self, buf: *const i32, csent: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(buf), ::std::mem::transmute(csent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPipeLong {
    type Vtable = IPipeLong_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3677305548, 12166, 4561, [142, 4, 0, 192, 79, 185, 152, 154]);
}
impl ::std::convert::From<IPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: IPipeLong) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPipeLong> for ::windows::runtime::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IPipeLong {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProcessInitControl(::windows::runtime::IUnknown);
impl IProcessInitControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwsecondsremaining)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProcessInitControl {
    type Vtable = IProcessInitControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1916276053, 36139, 17315, [133, 19, 43, 110, 243, 20, 52, 233]);
}
impl ::std::convert::From<IProcessInitControl> for ::windows::runtime::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProcessInitControl> for ::windows::runtime::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProcessInitControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProcessLock(::windows::runtime::IUnknown);
impl IProcessLock {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IProcessLock {
    type Vtable = IProcessLock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IProcessLock> for ::windows::runtime::IUnknown {
    fn from(value: IProcessLock) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProcessLock> for ::windows::runtime::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProcessLock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProgressNotify(::windows::runtime::IUnknown);
impl IProgressNotify {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn OnProgress<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwprogresscurrent), ::std::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProgressNotify {
    type Vtable = IProgressNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2849462432, 17943, 4559, [149, 252, 0, 170, 0, 104, 13, 180]);
}
impl ::std::convert::From<IProgressNotify> for ::windows::runtime::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProgressNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IProgressNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IROTData(::windows::runtime::IUnknown);
impl IROTData {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbdata), ::std::mem::transmute(cbmax), ::std::mem::transmute(pcbdata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IROTData {
    type Vtable = IROTData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4070534080, 20513, 4558, [170, 21, 0, 0, 105, 1, 41, 63]);
}
impl ::std::convert::From<IROTData> for ::windows::runtime::IUnknown {
    fn from(value: IROTData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IROTData> for ::windows::runtime::IUnknown {
    fn from(value: &IROTData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IROTData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IROTData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IReleaseMarshalBuffers(::windows::runtime::IUnknown);
impl IReleaseMarshalBuffers {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3943479784, 31126, 4562, [135, 46, 0, 0, 248, 8, 8, 89]);
}
impl ::std::convert::From<IReleaseMarshalBuffers> for ::windows::runtime::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IReleaseMarshalBuffers> for ::windows::runtime::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcChannelBuffer(::windows::runtime::IUnknown);
impl IRpcChannelBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630816, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::std::convert::From<IRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcChannelBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcChannelBuffer2(::windows::runtime::IUnknown);
impl IRpcChannelBuffer2 {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1498362320, 32537, 4560, [177, 148, 0, 160, 201, 13, 200, 191]);
}
impl ::std::convert::From<IRpcChannelBuffer2> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer2> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer2> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcChannelBuffer3(::windows::runtime::IUnknown);
impl IRpcChannelBuffer3 {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage), ::std::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(ulsize), ::std::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(riid), ::std::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), ::std::mem::transmute(pdwdestcontext), ::std::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RegisterAsync<'a, Param1: ::windows::runtime::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(632378880, 277, 4560, [191, 13, 0, 170, 0, 184, 223, 210]);
}
impl ::std::convert::From<IRpcChannelBuffer3> for ::windows::runtime::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer3> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer2 {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer2> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcChannelBuffer3> for IRpcChannelBuffer {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRpcChannelBuffer> for &IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRpcChannelBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IRpcChannelBuffer>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::runtime::GUID, pinterface: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcHelper(::windows::runtime::IUnknown);
impl IRpcHelper {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::std::ffi::c_void) -> ::windows::runtime::Result<*mut ::windows::runtime::GUID> {
        let mut result__: <*mut ::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pobjref), &mut result__).from_abi::<*mut ::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcHelper {
    type Vtable = IRpcHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(329, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRpcHelper> for ::windows::runtime::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjref: *const ::std::ffi::c_void, piid: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcOptions(::windows::runtime::IUnknown);
impl IRpcOptions {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Set<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pprx.into_param().abi(), ::std::mem::transmute(dwproperty), ::std::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Query<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pprx.into_param().abi(), ::std::mem::transmute(dwproperty), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRpcOptions {
    type Vtable = IRpcOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(324, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRpcOptions> for ::windows::runtime::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcOptions> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcProxyBuffer(::windows::runtime::IUnknown);
impl IRpcProxyBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Disconnect(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630516, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::std::convert::From<IRpcProxyBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcProxyBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcProxyBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcStubBuffer(::windows::runtime::IUnknown);
impl IRpcStubBuffer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Disconnect(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::runtime::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::runtime::GUID) -> ::std::option::Option<IRpcStubBuffer> {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CountRefs(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn DebugServerRelease(&self, pv: *const ::std::ffi::c_void) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv)))
    }
}
unsafe impl ::windows::runtime::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3589630716, 22843, 4122, [181, 105, 8, 0, 43, 45, 191, 122]);
}
impl ::std::convert::From<IRpcStubBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcStubBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcStubBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void),
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRpcSyntaxNegotiate(::windows::runtime::IUnknown);
impl IRpcSyntaxNegotiate {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmsg)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1486914841, 9416, 18741, [180, 130, 63, 216, 35, 51, 58, 79]);
}
impl ::std::convert::From<IRpcSyntaxNegotiate> for ::windows::runtime::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRpcSyntaxNegotiate> for ::windows::runtime::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRunnableObject(::windows::runtime::IUnknown);
impl IRunnableObject {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetRunningClass(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Run<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn LockRunning<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetContainedObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcontained: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), fcontained.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRunnableObject {
    type Vtable = IRunnableObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(294, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRunnableObject> for ::windows::runtime::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRunnableObject> for ::windows::runtime::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRunnableObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRunningObjectTable(::windows::runtime::IUnknown);
impl IRunningObjectTable {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Register<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregister)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pmkobjectname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwregister), ::std::mem::transmute(pfiletime)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::runtime::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pmkobjectname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn EnumRunning(&self) -> ::windows::runtime::Result<IEnumMoniker> {
        let mut result__: <IEnumMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumMoniker>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(16, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IRunningObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRunningObjectTable> for ::windows::runtime::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRunningObjectTable {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISequentialStream(::windows::runtime::IUnknown);
impl ISequentialStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Read(&self, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Write(&self, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISequentialStream {
    type Vtable = ISequentialStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(208878128, 10780, 4558, [173, 229, 0, 170, 0, 68, 119, 61]);
}
impl ::std::convert::From<ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISequentialStream> for ::windows::runtime::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISequentialStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IServerSecurity(::windows::runtime::IUnknown);
impl IServerSecurity {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::std::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pauthnsvc), ::std::mem::transmute(pauthzsvc), ::std::mem::transmute(pserverprincname), ::std::mem::transmute(pauthnlevel), ::std::mem::transmute(pimplevel), ::std::mem::transmute(pprivs), ::std::mem::transmute(pcapabilities)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ImpersonateClient(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RevertToSelf(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IServerSecurity {
    type Vtable = IServerSecurity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(318, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IServerSecurity> for ::windows::runtime::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IServerSecurity> for ::windows::runtime::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IServerSecurity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::std::ffi::c_void, pcapabilities: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IServiceProvider(::windows::runtime::IUnknown);
impl IServiceProvider {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(guidservice), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobject)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IServiceProvider {
    type Vtable = IServiceProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1834041537, 29750, 4558, [128, 52, 0, 170, 0, 96, 9, 250]);
}
impl ::std::convert::From<IServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IServiceProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IServiceProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStdMarshalInfo(::windows::runtime::IUnknown);
impl IStdMarshalInfo {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(pclsid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IStdMarshalInfo> for ::windows::runtime::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStdMarshalInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStdMarshalInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Com`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStream(::windows::runtime::IUnknown);
impl IStream {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Read(&self, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Write(&self, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dlibmove), ::std::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread), ::std::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatstg), ::std::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IStream> {
        let mut result__: <IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IStream>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStream {
    type Vtable = IStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IStream> for ::windows::runtime::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ::windows::runtime::IUnknown {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IStream> for ISequentialStream {
    fn from(value: IStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStream> for ISequentialStream {
    fn from(value: &IStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISequentialStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISequentialStream> for &IStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISequentialStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISurrogate(::windows::runtime::IUnknown);
impl ISurrogate {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn FreeSurrogate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurrogate {
    type Vtable = ISurrogate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(34, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISurrogate> for ::windows::runtime::IUnknown {
    fn from(value: ISurrogate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISurrogate> for ::windows::runtime::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISurrogate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISurrogateService(::windows::runtime::IUnknown);
impl ISurrogateService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn Init<'a, Param1: ::windows::runtime::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows::runtime::GUID, pprocesslock: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::runtime::GUID, apptype: ApplicationType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidapplid), ::std::mem::transmute(apptype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(rguidapplid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulreserved)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(shutdowntype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurrogateService {
    type Vtable = ISurrogateService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(468, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISurrogateService> for ::windows::runtime::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISurrogateService> for ::windows::runtime::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISurrogateService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronize(::windows::runtime::IUnknown);
impl ISynchronize {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Signal(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronize {
    type Vtable = ISynchronize_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(48, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISynchronize> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronize) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronize> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISynchronize {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronizeContainer(::windows::runtime::IUnknown);
impl ISynchronizeContainer {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::runtime::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwtimeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(51, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISynchronizeContainer> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeContainer> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISynchronizeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronizeEvent(::windows::runtime::IUnknown);
impl ISynchronizeEvent {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ph)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(50, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISynchronizeEvent> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeEvent> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeEvent> for ISynchronizeHandle {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronizeHandle> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronizeHandle> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISynchronizeHandle>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronizeHandle> for &ISynchronizeEvent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronizeHandle> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISynchronizeHandle>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronizeHandle(::windows::runtime::IUnknown);
impl ISynchronizeHandle {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(49, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISynchronizeHandle> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeHandle> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISynchronizeHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronizeMutex(::windows::runtime::IUnknown);
impl ISynchronizeMutex {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Signal(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn ReleaseMutex(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<ISynchronizeMutex> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeMutex> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISynchronizeMutex> for ISynchronize {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISynchronizeMutex> for ISynchronize {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronize> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronize> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISynchronize>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISynchronize> for &ISynchronizeMutex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISynchronize> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISynchronize>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITimeAndNoticeControl(::windows::runtime::IUnknown);
impl ITimeAndNoticeControl {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(res1), ::std::mem::transmute(res2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154900654, 34936, 4561, [131, 233, 0, 192, 79, 194, 198, 212]);
}
impl ::std::convert::From<ITimeAndNoticeControl> for ::windows::runtime::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITimeAndNoticeControl> for ::windows::runtime::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUri(::windows::runtime::IUnknown);
impl IUri {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pbstrproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pcchproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), ::std::mem::transmute(pdwproperty), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uriprop), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetAuthority(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetDomain(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetExtension(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetRawUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserInfo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetHostType(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetPort(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetScheme(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetZone(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), puri.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUri {
    type Vtable = IUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2745100104, 27175, 18455, [166, 242, 19, 145, 75, 239, 88, 144]);
}
impl ::std::convert::From<IUri> for ::windows::runtime::IUnknown {
    fn from(value: IUri) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUri> for ::windows::runtime::IUnknown {
    fn from(value: &IUri) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUri {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pbstrproperty: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrabsoluteuri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrauthority: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdisplaystring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrdomain: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrextension: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrfragment: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrhost: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpassword: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrpathandquery: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrquery: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrrawuri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrschemename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruserinfo: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrusername: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUriBuilder(::windows::runtime::IUnknown);
impl IUriBuilder {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwallowencodingpropertymask), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcreateflags), ::std::mem::transmute(dwallowencodingpropertymask), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwcreateflags), ::std::mem::transmute(dwuribuilderflags), ::std::mem::transmute(dwallowencodingpropertymask), ::std::mem::transmute(dwreserved), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn GetIUri(&self) -> ::windows::runtime::Result<IUri> {
        let mut result__: <IUri as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn SetIUri<'a, Param0: ::windows::runtime::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), piuri.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchfragment), ::std::mem::transmute(ppwzfragment)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchhost), ::std::mem::transmute(ppwzhost)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchpassword), ::std::mem::transmute(ppwzpassword)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchpath), ::std::mem::transmute(ppwzpath)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfhasport), ::std::mem::transmute(pdwport)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchquery), ::std::mem::transmute(ppwzquery)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchschemename), ::std::mem::transmute(ppwzschemename)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcchusername), ::std::mem::transmute(ppwzusername)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetFragment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetHost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPassword<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetPort<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), fhasport.into_param().abi(), ::std::mem::transmute(dwnewvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetSchemeName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn SetUserName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwznewvalue: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwpropertymask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
    pub unsafe fn HasBeenModified(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUriBuilder {
    type Vtable = IUriBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1109504737, 35157, 18112, [189, 91, 222, 152, 151, 86, 93, 231]);
}
impl ::std::convert::From<IUriBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUriBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUriBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUrlMon(::windows::runtime::IUnknown);
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
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rclsid),
            psztype.into_param().abi(),
            pszext.into_param().abi(),
            ::std::mem::transmute(dwfileversionms),
            ::std::mem::transmute(dwfileversionls),
            pszcodebase.into_param().abi(),
            pbc.into_param().abi(),
            ::std::mem::transmute(dwclasscontext),
            ::std::mem::transmute(riid),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUrlMon {
    type Vtable = IUrlMon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(38, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IUrlMon> for ::windows::runtime::IUnknown {
    fn from(value: IUrlMon) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUrlMon> for ::windows::runtime::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUrlMon {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWaitMultiple(::windows::runtime::IUnknown);
impl IWaitMultiple {
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::runtime::Result<ISynchronize> {
        let mut result__: <ISynchronize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(timeout), &mut result__).from_abi::<ISynchronize>(result__)
    }
    #[doc = "*Required features: `Win32_System_Com`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::runtime::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWaitMultiple {
    type Vtable = IWaitMultiple_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(43, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IWaitMultiple> for ::windows::runtime::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWaitMultiple> for ::windows::runtime::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWaitMultiple {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl LONG_SIZEDARR {}
impl ::std::default::Default for LONG_SIZEDARR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LONG_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for LONG_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
pub type LPFNCANUNLOADNOW = unsafe extern "system" fn() -> ::windows::runtime::HRESULT;
pub type LPFNGETCLASSOBJECT = unsafe extern "system" fn(param0: *const ::windows::runtime::GUID, param1: *const ::windows::runtime::GUID, param2: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `Win32_System_Com`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MEMCTX(pub i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::std::convert::From<i32> for MEMCTX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEMCTX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MKREDUCE(pub i32);
pub const MKRREDUCE_ONE: MKREDUCE = MKREDUCE(196608i32);
pub const MKRREDUCE_TOUSER: MKREDUCE = MKREDUCE(131072i32);
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = MKREDUCE(65536i32);
pub const MKRREDUCE_ALL: MKREDUCE = MKREDUCE(0i32);
impl ::std::convert::From<i32> for MKREDUCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MKREDUCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for MKSYS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MKSYS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MSHCTX(pub i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::std::convert::From<i32> for MSHCTX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSHCTX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for MSHLFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MSHLFLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct MULTI_QI {
    pub pIID: *mut ::windows::runtime::GUID,
    pub pItf: ::std::option::Option<::windows::runtime::IUnknown>,
    pub hr: ::windows::runtime::HRESULT,
}
impl MULTI_QI {}
impl ::std::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MULTI_QI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
impl ::std::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::std::cmp::Eq for MULTI_QI {}
unsafe impl ::windows::runtime::Abi for MULTI_QI {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl MachineGlobalObjectTableRegistrationToken__ {}
impl ::std::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
impl ::std::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
unsafe impl ::windows::runtime::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, IBindCtx>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::std::option::Option<IMoniker>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows::runtime::RawPtr, szusername: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::std::mem::transmute(pcheaten), ::std::mem::transmute(ppmk)).ok()
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
        let mut result__: <IMoniker as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), &mut result__).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn MonikerRelativePathTo<'a, Param0: ::windows::runtime::IntoParam<'a, IMoniker>, Param1: ::windows::runtime::IntoParam<'a, IMoniker>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pmksrc: Param0, pmkdest: Param1, ppmkrelpath: *mut ::std::option::Option<IMoniker>, dwreserved: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerRelativePathTo(pmksrc: ::windows::runtime::RawPtr, pmkdest: ::windows::runtime::RawPtr, ppmkrelpath: *mut ::windows::runtime::RawPtr, dwreserved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        MonikerRelativePathTo(pmksrc.into_param().abi(), pmkdest.into_param().abi(), ::std::mem::transmute(ppmkrelpath), dwreserved.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PENDINGMSG(pub i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::std::convert::From<i32> for PENDINGMSG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PENDINGMSG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PENDINGTYPE(pub i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::std::convert::From<i32> for PENDINGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PENDINGTYPE {
    type Abi = Self;
}
pub type PFNCONTEXTCALL = unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows::runtime::GUID, lplpszprogid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ProgIDFromCLSID(::std::mem::transmute(clsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
impl ::std::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.dwContext == other.dwContext && self.Platform == other.Platform && self.Locale == other.Locale && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo
    }
}
impl ::std::cmp::Eq for QUERYCONTEXT {}
unsafe impl ::windows::runtime::Abi for QUERYCONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGCLS(pub i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::std::convert::From<i32> for REGCLS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGCLS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::std::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::std::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::std::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl RPCOLEMESSAGE {}
impl ::std::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
impl ::std::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.dataRepresentation == other.dataRepresentation && self.Buffer == other.Buffer && self.cbBuffer == other.cbBuffer && self.iMethod == other.iMethod && self.reserved2 == other.reserved2 && self.rpcFlags == other.rpcFlags
    }
}
impl ::std::cmp::Eq for RPCOLEMESSAGE {}
unsafe impl ::windows::runtime::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPCOPT_PROPERTIES(pub i32);
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::std::convert::From<i32> for RPCOPT_PROPERTIES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::std::convert::From<i32> for RPCOPT_SERVER_LOCALITY_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::std::convert::From<u32> for RPC_C_AUTHN_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_AUTHN_LEVEL {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_AUTHN_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_AUTHN_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_AUTHN_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_IMP_LEVEL(pub u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::std::convert::From<u32> for RPC_C_IMP_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_IMP_LEVEL {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_IMP_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_IMP_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_IMP_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
impl ::std::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.dwHandleType == other.dwHandleType && self.pData == other.pData && self.pUnkForRelease == other.pUnkForRelease && self.cbData == other.cbData && self.data == other.data
    }
}
impl ::std::cmp::Eq for RemSTGMEDIUM {}
unsafe impl ::windows::runtime::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::std::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl SAFEARRAY {}
impl ::std::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SAFEARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
impl ::std::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.cDims == other.cDims && self.fFeatures == other.fFeatures && self.cbElements == other.cbElements && self.cLocks == other.cLocks && self.pvData == other.pvData && self.rgsabound == other.rgsabound
    }
}
impl ::std::cmp::Eq for SAFEARRAY {}
unsafe impl ::windows::runtime::Abi for SAFEARRAY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl SAFEARRAYBOUND {}
impl ::std::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
impl ::std::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        self.cElements == other.cElements && self.lLbound == other.lLbound
    }
}
impl ::std::cmp::Eq for SAFEARRAYBOUND {}
unsafe impl ::windows::runtime::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::runtime::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::runtime::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::std::ffi::c_void,
}
impl SChannelHookCallInfo {}
impl ::std::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
impl ::std::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.cbSize == other.cbSize && self.uCausality == other.uCausality && self.dwServerPid == other.dwServerPid && self.iMethod == other.iMethod && self.pObject == other.pObject
    }
}
impl ::std::cmp::Eq for SChannelHookCallInfo {}
unsafe impl ::windows::runtime::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVERCALL(pub i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::std::convert::From<i32> for SERVERCALL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVERCALL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl SHORT_SIZEDARR {}
impl ::std::default::Default for SHORT_SIZEDARR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SHORT_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for SHORT_SIZEDARR {}
unsafe impl ::windows::runtime::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::std::ffi::c_void,
}
impl SOLE_AUTHENTICATION_INFO {}
impl ::std::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
impl ::std::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pAuthInfo == other.pAuthInfo
    }
}
impl ::std::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl SOLE_AUTHENTICATION_LIST {}
impl ::std::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
impl ::std::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cAuthInfo == other.cAuthInfo && self.aAuthInfo == other.aAuthInfo
    }
}
impl ::std::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAuthnSvc == other.dwAuthnSvc && self.dwAuthzSvc == other.dwAuthzSvc && self.pPrincipalName == other.pPrincipalName && self.hr == other.hr
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::std::option::Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl STATDATA {}
impl ::std::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STATDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
impl ::std::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::std::cmp::Eq for STATDATA {}
unsafe impl ::windows::runtime::Abi for STATDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STATSTG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsName == other.pwcsName && self.r#type == other.r#type && self.cbSize == other.cbSize && self.mtime == other.mtime && self.ctime == other.ctime && self.atime == other.atime && self.grfMode == other.grfMode && self.grfLocksSupported == other.grfLocksSupported && self.clsid == other.clsid && self.grfStateBits == other.grfStateBits && self.reserved == other.reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STATSTG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::clone::Clone for STGMEDIUM {
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
    pub pUnkForRelease: ::std::option::Option<::windows::runtime::IUnknown>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for STGMEDIUM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for STGMEDIUM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::std::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: super::super::Foundation::PWSTR,
    pub pstm: ::windows::runtime::RawPtr,
    pub pstg: ::windows::runtime::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::std::cmp::Eq for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::runtime::Abi for STGMEDIUM_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STGTY(pub i32);
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::std::convert::From<i32> for STGTY {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STREAM_SEEK(pub u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::std::convert::From<u32> for STREAM_SEEK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STREAM_SEEK {
    type Abi = Self;
}
impl ::std::ops::BitOr for STREAM_SEEK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for STREAM_SEEK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for STREAM_SEEK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for STREAM_SEEK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for STREAM_SEEK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ShutdownType(pub i32);
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::std::convert::From<i32> for ShutdownType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ShutdownType {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for StorageLayout {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        self.LayoutType == other.LayoutType && self.pwcsElementName == other.pwcsElementName && self.cOffset == other.cOffset && self.cBytes == other.cBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for StorageLayout {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for StorageLayout {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StringFromCLSID(::std::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::runtime::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows::runtime::GUID, lpsz: super::super::Foundation::PWSTR, cchmax: i32) -> i32;
        }
        ::std::mem::transmute(StringFromGUID2(::std::mem::transmute(rguid), ::std::mem::transmute(lpsz), ::std::mem::transmute(cchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows::runtime::GUID, lplpsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        StringFromIID(::std::mem::transmute(rclsid), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct THDTYPE(pub i32);
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::std::convert::From<i32> for THDTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THDTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for TYMED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYMED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TYSPEC(pub i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::std::convert::From<i32> for TYSPEC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TYSPEC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for URI_CREATE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for URI_CREATE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Com`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for Uri_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Uri_PROPERTY {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for VARIANT {
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
impl ::std::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for VARIANT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
pub union VARIANT_0 {
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
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
impl ::std::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_System_Ole_Automation`*"]
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
    pub bstrVal: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
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
    pub pbstrVal: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    pub ppunkVal: *mut ::windows::runtime::RawPtr,
    pub ppdispVal: *mut ::windows::runtime::RawPtr,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut ::std::mem::ManuallyDrop<VARIANT>,
    pub byref: *mut ::std::ffi::c_void,
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
    pub Anonymous: ::std::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
impl ::std::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_Ole_Automation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::std::ffi::c_void,
    pub pRecInfo: ::std::option::Option<super::Ole::Automation::IRecordInfo>,
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl VARIANT_0_0_0_0 {}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::cmp::PartialEq for VARIANT_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pvRecord == other.pvRecord && self.pRecInfo == other.pRecInfo
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::cmp::Eq for VARIANT_0_0_0_0 {}
#[cfg(feature = "Win32_System_Ole_Automation")]
unsafe impl ::windows::runtime::Abi for VARIANT_0_0_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl WORD_BLOB {}
impl ::std::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WORD_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
impl ::std::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.clSize == other.clSize && self.asData == other.asData
    }
}
impl ::std::cmp::Eq for WORD_BLOB {}
unsafe impl ::windows::runtime::Abi for WORD_BLOB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for uCLSSPEC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
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
impl ::std::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for uCLSSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`*"]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: super::super::Foundation::PWSTR,
    pub PolicyId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ByName_e__Struct").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pPackageName == other.pPackageName && self.PolicyId == other.PolicyId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for uCLSSPEC_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::runtime::GUID,
    pub PolicyId: ::windows::runtime::GUID,
}
impl uCLSSPEC_0_1 {}
impl ::std::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ByObjectId_e__Struct").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
impl ::std::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectId == other.ObjectId && self.PolicyId == other.PolicyId
    }
}
impl ::std::cmp::Eq for uCLSSPEC_0_1 {}
unsafe impl ::windows::runtime::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl userFLAG_STGMEDIUM {}
impl ::std::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
impl ::std::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::std::cmp::Eq for userFLAG_STGMEDIUM {}
unsafe impl ::windows::runtime::Abi for userFLAG_STGMEDIUM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Com`*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl userSTGMEDIUM {}
impl ::std::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
impl ::std::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::std::cmp::Eq for userSTGMEDIUM {}
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_System_Com`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
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
impl ::std::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
