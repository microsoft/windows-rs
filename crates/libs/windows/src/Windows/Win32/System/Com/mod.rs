#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ADVF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_NODATA: ADVF = ADVF(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
impl ::core::marker::Copy for ADVF {}
impl ::core::clone::Clone for ADVF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ADVF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ADVF {
    type Abi = Self;
}
impl ::core::fmt::Debug for ADVF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADVF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
impl ::core::marker::Copy for APTTYPE {}
impl ::core::clone::Clone for APTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APTTYPEQUALIFIER(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
impl ::core::marker::Copy for APTTYPEQUALIFIER {}
impl ::core::clone::Clone for APTTYPEQUALIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APTTYPEQUALIFIER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APTTYPEQUALIFIER {
    type Abi = Self;
}
impl ::core::fmt::Debug for APTTYPEQUALIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APTTYPEQUALIFIER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for AUTHENTICATEINFO {}
impl ::core::clone::Clone for AUTHENTICATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHENTICATEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATEINFO").field("dwFlags", &self.dwFlags).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHENTICATEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHENTICATEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHENTICATEINFO {}
impl ::core::default::Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
impl ::core::marker::Copy for ApplicationType {}
impl ::core::clone::Clone for ApplicationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink(::windows::core::IUnknown);
impl AsyncIAdviseSink {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).Begin_OnDataChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnDataChange)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).Begin_OnViewChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnViewChange)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).Begin_OnRename)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnRename)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Interface::vtable(self).Begin_OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Interface::vtable(self).Begin_OnClose)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnClose)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink {}
impl ::core::fmt::Debug for AsyncIAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink {
    type Vtable = AsyncIAdviseSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000150_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_OnDataChange: usize,
    pub Finish_OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub Finish_OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub Finish_OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Begin_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Finish_OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIAdviseSink2(::windows::core::IUnknown);
impl AsyncIAdviseSink2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).base.Begin_OnDataChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnDataChange(&self) {
        (::windows::core::Interface::vtable(self).base.Finish_OnDataChange)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).base.Begin_OnViewChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnViewChange(&self) {
        (::windows::core::Interface::vtable(self).base.Finish_OnViewChange)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).base.Begin_OnRename)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnRename(&self) {
        (::windows::core::Interface::vtable(self).base.Finish_OnRename)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnSave(&self) {
        (::windows::core::Interface::vtable(self).base.Begin_OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnSave(&self) {
        (::windows::core::Interface::vtable(self).base.Finish_OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnClose(&self) {
        (::windows::core::Interface::vtable(self).base.Begin_OnClose)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnClose(&self) {
        (::windows::core::Interface::vtable(self).base.Finish_OnClose)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).Begin_OnLinkSrcChange)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        (::windows::core::Interface::vtable(self).Finish_OnLinkSrcChange)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: AsyncIAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &AsyncIAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, AsyncIAdviseSink> for &'a AsyncIAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, AsyncIAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIAdviseSink2 {}
impl ::core::fmt::Debug for AsyncIAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIAdviseSink2 {
    type Vtable = AsyncIAdviseSink2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000151_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIMultiQI(::windows::core::IUnknown);
impl AsyncIMultiQI {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(::core::mem::transmute_copy(self), pmqis.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pmqis))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmqis)).ok()
    }
}
impl ::core::convert::From<AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: AsyncIMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIMultiQI> for ::windows::core::IUnknown {
    fn from(value: &AsyncIMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMultiQI {}
impl ::core::fmt::Debug for AsyncIMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIMultiQI {
    type Vtable = AsyncIMultiQI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMultiQI_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeByte(::windows::core::IUnknown);
impl AsyncIPipeByte {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Push(&self, buf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Push)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeByte> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeByte {}
impl ::core::fmt::Debug for AsyncIPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeByte {
    type Vtable = AsyncIPipeByte_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeByte_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeDouble(::windows::core::IUnknown);
impl AsyncIPipeDouble {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Push(&self, buf: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Push)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeDouble {}
impl ::core::fmt::Debug for AsyncIPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeDouble {
    type Vtable = AsyncIPipeDouble_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIPipeLong(::windows::core::IUnknown);
impl AsyncIPipeLong {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequest)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(buf), ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Push(&self, buf: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Push(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Push)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: AsyncIPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIPipeLong> for ::windows::core::IUnknown {
    fn from(value: &AsyncIPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIPipeLong {}
impl ::core::fmt::Debug for AsyncIPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIPipeLong {
    type Vtable = AsyncIPipeLong_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIPipeLong_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct AsyncIUnknown(::windows::core::IUnknown);
impl AsyncIUnknown {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_QueryInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_QueryInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_QueryInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_AddRef(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_AddRef)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Finish_AddRef)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Begin_Release(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Release)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Finish_Release(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Finish_Release)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: AsyncIUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIUnknown> for ::windows::core::IUnknown {
    fn from(value: &AsyncIUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIUnknown {}
impl ::core::fmt::Debug for AsyncIUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIUnknown {
    type Vtable = AsyncIUnknown_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000e0000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIUnknown_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: ::windows::core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: ::windows::core::PWSTR,
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
impl ::core::clone::Clone for BINDINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            szExtraInfo: self.szExtraInfo,
            stgmedData: self.stgmedData.clone(),
            grfBindInfoF: self.grfBindInfoF,
            dwBindVerb: self.dwBindVerb,
            szCustomVerb: self.szCustomVerb,
            cbstgmedData: self.cbstgmedData,
            dwOptions: self.dwOptions,
            dwOptionsFlags: self.dwOptionsFlags,
            dwCodePage: self.dwCodePage,
            securityAttributes: self.securityAttributes,
            iid: self.iid,
            pUnk: self.pUnk.clone(),
            dwReserved: self.dwReserved,
        }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for BINDINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for BINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szExtraInfo == other.szExtraInfo && self.stgmedData == other.stgmedData && self.grfBindInfoF == other.grfBindInfoF && self.dwBindVerb == other.dwBindVerb && self.szCustomVerb == other.szCustomVerb && self.cbstgmedData == other.cbstgmedData && self.dwOptions == other.dwOptions && self.dwOptionsFlags == other.dwOptionsFlags && self.dwCodePage == other.dwCodePage && self.securityAttributes == other.securityAttributes && self.iid == other.iid && self.pUnk == other.pUnk && self.dwReserved == other.dwReserved
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for BINDINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for BINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BINDINFOF(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
impl ::core::marker::Copy for BINDINFOF {}
impl ::core::clone::Clone for BINDINFOF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BINDINFOF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BINDINFOF {
    type Abi = Self;
}
impl ::core::fmt::Debug for BINDINFOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFOF").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: ::core::mem::ManuallyDrop<::core::option::Option<ITypeComp>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for BINDPTR {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for BINDPTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BINDPTR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for BINDPTR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for BINDPTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BIND_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
impl ::core::marker::Copy for BIND_FLAGS {}
impl ::core::clone::Clone for BIND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BIND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for BIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIND_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl ::core::marker::Copy for BIND_OPTS {}
impl ::core::clone::Clone for BIND_OPTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS").field("cbStruct", &self.cbStruct).field("grfFlags", &self.grfFlags).field("grfMode", &self.grfMode).field("dwTickCountDeadline", &self.dwTickCountDeadline).finish()
    }
}
unsafe impl ::windows::core::Abi for BIND_OPTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS {}
impl ::core::default::Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BIND_OPTS2 {
    pub __AnonymousBase_objidl_L9017_C36: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl ::core::marker::Copy for BIND_OPTS2 {}
impl ::core::clone::Clone for BIND_OPTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BIND_OPTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS2").field("__AnonymousBase_objidl_L9017_C36", &self.__AnonymousBase_objidl_L9017_C36).field("dwTrackFlags", &self.dwTrackFlags).field("dwClassContext", &self.dwClassContext).field("locale", &self.locale).field("pServerInfo", &self.pServerInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for BIND_OPTS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BIND_OPTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for BIND_OPTS2 {}
impl ::core::default::Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BIND_OPTS3 {
    pub __AnonymousBase_objidl_L9041_C36: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BIND_OPTS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BIND_OPTS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIND_OPTS3").field("__AnonymousBase_objidl_L9041_C36", &self.__AnonymousBase_objidl_L9041_C36).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for BIND_OPTS3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BIND_OPTS3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BIND_OPTS3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BIND_OPTS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl ::core::marker::Copy for BLOB {}
impl ::core::clone::Clone for BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLOB").field("cbSize", &self.cbSize).field("pBlobData", &self.pBlobData).finish()
    }
}
unsafe impl ::windows::core::Abi for BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BLOB {}
impl ::core::default::Default for BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for BYTE_BLOB {}
impl ::core::clone::Clone for BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_BLOB").field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_BLOB {}
impl ::core::default::Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for BYTE_SIZEDARR {}
impl ::core::clone::Clone for BYTE_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BYTE_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BYTE_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for BYTE_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BYTE_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BYTE_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for BYTE_SIZEDARR {}
impl ::core::default::Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLCONV(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
impl ::core::marker::Copy for CALLCONV {}
impl ::core::clone::Clone for CALLCONV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLCONV {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CALLCONV {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLCONV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLCONV").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CALLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
impl ::core::marker::Copy for CALLTYPE {}
impl ::core::clone::Clone for CALLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CALLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CALLTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CALLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CATEGORYINFO {
    pub catid: ::windows::core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl ::core::marker::Copy for CATEGORYINFO {}
impl ::core::clone::Clone for CATEGORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORYINFO").field("catid", &self.catid).field("lcid", &self.lcid).field("szDescription", &self.szDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for CATEGORYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CATEGORYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CATEGORYINFO {}
impl ::core::default::Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLSCTX(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
impl ::core::marker::Copy for CLSCTX {}
impl ::core::clone::Clone for CLSCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLSCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLSCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLSCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLSCTX").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromProgID<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgID(lpszprogid: ::windows::core::PCWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromProgID(lpszprogid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromProgIDEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszprogid: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromProgIDEx(lpszprogid: ::windows::core::PCWSTR, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromProgIDEx(lpszprogid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CLSIDFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLSIDFromString(lpsz: ::windows::core::PCWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CLSIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for COAUTHIDENTITY {}
impl ::core::clone::Clone for COAUTHIDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHIDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHIDENTITY").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for COAUTHIDENTITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHIDENTITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHIDENTITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for COAUTHIDENTITY {}
impl ::core::default::Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: ::windows::core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl ::core::marker::Copy for COAUTHINFO {}
impl ::core::clone::Clone for COAUTHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COAUTHINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COAUTHINFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pwszServerPrincName", &self.pwszServerPrincName).field("dwAuthnLevel", &self.dwAuthnLevel).field("dwImpersonationLevel", &self.dwImpersonationLevel).field("pAuthIdentityData", &self.pAuthIdentityData).field("dwCapabilities", &self.dwCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for COAUTHINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COAUTHINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COAUTHINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COAUTHINFO {}
impl ::core::default::Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COINIT(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
impl ::core::marker::Copy for COINIT {}
impl ::core::clone::Clone for COINIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COINIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COINITBASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
impl ::core::marker::Copy for COINITBASE {}
impl ::core::clone::Clone for COINITBASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COINITBASE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COINITBASE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COINITBASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COINITBASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMSD(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
impl ::core::marker::Copy for COMSD {}
impl ::core::clone::Clone for COMSD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMSD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMSD {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMSD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMSD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CONNECTDATA {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub dwCookie: u32,
}
impl ::core::clone::Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), dwCookie: self.dwCookie }
    }
}
impl ::core::fmt::Debug for CONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDATA").field("pUnk", &self.pUnk).field("dwCookie", &self.dwCookie).finish()
    }
}
unsafe impl ::windows::core::Abi for CONNECTDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for CONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.dwCookie == other.dwCookie
    }
}
impl ::core::cmp::Eq for CONNECTDATA {}
impl ::core::default::Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: ::windows::core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for COSERVERINFO {}
impl ::core::clone::Clone for COSERVERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COSERVERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COSERVERINFO").field("dwReserved1", &self.dwReserved1).field("pwszName", &self.pwszName).field("pAuthInfo", &self.pAuthInfo).field("dwReserved2", &self.dwReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for COSERVERINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COSERVERINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COSERVERINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for COSERVERINFO {}
impl ::core::default::Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COWAIT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
impl ::core::marker::Copy for COWAIT_FLAGS {}
impl ::core::clone::Clone for COWAIT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COWAIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COWAIT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for COWAIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COWAIT_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_DEVICE_CATALOG_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_DEVICE_CATALOG_COOKIE {}
impl ::core::fmt::Debug for CO_DEVICE_CATALOG_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_DEVICE_CATALOG_COOKIE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for CO_DEVICE_CATALOG_COOKIE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
impl ::core::marker::Copy for CO_MARSHALING_CONTEXT_ATTRIBUTES {}
impl ::core::clone::Clone for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MARSHALING_CONTEXT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_MTA_USAGE_COOKIE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_MTA_USAGE_COOKIE {}
impl ::core::fmt::Debug for CO_MTA_USAGE_COOKIE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MTA_USAGE_COOKIE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for CO_MTA_USAGE_COOKIE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl ::core::marker::Copy for CSPLATFORM {}
impl ::core::clone::Clone for CSPLATFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CSPLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSPLATFORM").field("dwPlatformId", &self.dwPlatformId).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).field("dwProcessorArch", &self.dwProcessorArch).finish()
    }
}
unsafe impl ::windows::core::Abi for CSPLATFORM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CSPLATFORM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CSPLATFORM>()) == 0 }
    }
}
impl ::core::cmp::Eq for CSPLATFORM {}
impl ::core::default::Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for CUSTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CUSTDATA").field("cCustData", &self.cCustData).field("prgCustData", &self.prgCustData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CUSTDATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct CUSTDATAITEM {
    pub guid: ::windows::core::GUID,
    pub varValue: VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        Self { guid: self.guid, varValue: self.varValue.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for CUSTDATAITEM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for CUSTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.varValue == other.varValue
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for CUSTDATAITEM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CWMO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
impl ::core::marker::Copy for CWMO_FLAGS {}
impl ::core::clone::Clone for CWMO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CWMO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CWMO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CWMO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CWMO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const CWMO_MAX_HANDLES: u32 = 56u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl ::core::marker::Copy for CY {}
impl ::core::clone::Clone for CY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY {}
impl ::core::default::Default for CY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl ::core::marker::Copy for CY_0 {}
impl ::core::clone::Clone for CY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CY_0").field("Lo", &self.Lo).field("Hi", &self.Hi).finish()
    }
}
unsafe impl ::windows::core::Abi for CY_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CY_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CY_0 {}
impl ::core::default::Default for CY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoAllowSetForegroundWindow(punk: *mut ::core::ffi::c_void, lpvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoAllowSetForegroundWindow(punk.into_param().abi(), ::core::mem::transmute(lpvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCopyProxy(pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoCopyProxy(pproxy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateFreeThreadedMarshaler(punkouter: *mut ::core::ffi::c_void, ppunkmarshal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoCreateFreeThreadedMarshaler(punkouter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateGuid() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateGuid(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoCreateGuid(::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(rclsid: *const ::windows::core::GUID, punkouter: Param1, dwclscontext: CLSCTX) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstance(rclsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoCreateInstance(::core::mem::transmute(rclsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclscontext), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstanceEx<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, presults: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceEx(clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, pserverinfo: *const COSERVERINFO, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceEx(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pserverinfo), presults.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presults))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoCreateInstanceFromApp<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(clsid: *const ::windows::core::GUID, punkouter: Param1, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, presults: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoCreateInstanceFromApp(clsid: *const ::windows::core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: CLSCTX, reserved: *const ::core::ffi::c_void, dwcount: u32, presults: *mut MULTI_QI) -> ::windows::core::HRESULT;
        }
        CoCreateInstanceFromApp(::core::mem::transmute(clsid), punkouter.into_param().abi(), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(reserved), presults.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(presults))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoDisconnectObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0, dwreserved: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDisconnectObject(punk: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT;
        }
        CoDisconnectObject(punk.into_param().abi(), ::core::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFileTimeNow() -> ::windows::core::Result<super::super::Foundation::FILETIME> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFileTimeNow(lpfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        CoFileTimeNow(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeAllLibraries();
        }
        CoFreeAllLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoFreeLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinst: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeLibrary(hinst: super::super::Foundation::HINSTANCE);
        }
        CoFreeLibrary(hinst.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibraries();
        }
        CoFreeUnusedLibraries()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32);
        }
        CoFreeUnusedLibrariesEx(::core::mem::transmute(dwunloaddelay), ::core::mem::transmute(dwreserved))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCallerTID() -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCallerTID(lpdwtid: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoGetCallerTID(::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetContextToken() -> ::windows::core::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetContextToken(ptoken: *mut usize) -> ::windows::core::HRESULT;
        }
        let mut result__: usize = ::core::mem::zeroed();
        CoGetContextToken(::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetCurrentLogicalThreadId(pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetCurrentLogicalThreadId(::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> ::windows::core::Result<IMalloc> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMalloc(dwmemcontext: u32, ppmalloc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CoGetMalloc(::core::mem::transmute(dwmemcontext), ::core::mem::transmute(&mut result__)).from_abi::<IMalloc>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszname: Param0, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetObject(pszname: ::windows::core::PCWSTR, pbindoptions: *const BIND_OPTS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoGetObject(pszname.into_param().abi(), ::core::mem::transmute(pbindoptions), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetPSClsid(riid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetPSClsid(::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetTreatAsClass(clsidold: *const ::windows::core::GUID, pclsidnew: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        CoGetTreatAsClass(::core::mem::transmute(clsidold), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> ::windows::core::Result<CO_MTA_USAGE_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIncrementMTAUsage(pcookie: *mut CO_MTA_USAGE_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: CO_MTA_USAGE_COOKIE = ::core::mem::zeroed();
        CoIncrementMTAUsage(::core::mem::transmute(&mut result__)).from_abi::<CO_MTA_USAGE_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, asauthsvc: &[SOLE_AUTHENTICATION_SERVICE], preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInitializeSecurity(psecdesc: *const super::super::Security::SECURITY_DESCRIPTOR, cauthsvc: i32, asauthsvc: *const SOLE_AUTHENTICATION_SERVICE, preserved1: *const ::core::ffi::c_void, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoInitializeSecurity(::core::mem::transmute(psecdesc), asauthsvc.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(asauthsvc)), ::core::mem::transmute(preserved1), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthlist), ::core::mem::transmute(dwcapabilities), ::core::mem::transmute(preserved3)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInstall<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pbc: Param0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInstall(pbc: ::windows::core::RawPtr, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        CoInstall(pbc.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pclassspec), ::core::mem::transmute(pquery), pszcodebase.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszmachinename: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoInvalidateRemoteMachineBindings(pszmachinename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        CoInvalidateRemoteMachineBindings(pszmachinename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoIsHandlerConnected<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoIsHandlerConnected(punk: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CoIsHandlerConnected(punk.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLoadLibrary<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszlibname: Param0, bautofree: Param1) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLoadLibrary(lpszlibname: ::windows::core::PCWSTR, bautofree: super::super::Foundation::BOOL) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(CoLoadLibrary(lpszlibname.into_param().abi(), bautofree.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoLockObjectExternal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(punk: Param0, flock: Param1, flastunlockreleases: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoLockObjectExternal(punk: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockreleases: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        CoLockObjectExternal(punk.into_param().abi(), flock.into_param().abi(), flastunlockreleases.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryClientBlanket(pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryClientBlanket(::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoQueryProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pproxy: Param0, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoQueryProxyBlanket(pproxy: *mut ::core::ffi::c_void, pwauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut ::windows::core::PWSTR, pauthnlevel: *mut u32, pimplevel: *mut u32, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut u32) -> ::windows::core::HRESULT;
        }
        CoQueryProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(pwauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterClassObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(rclsid: *const ::windows::core::GUID, punk: Param1, dwclscontext: CLSCTX, flags: u32) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterClassObject(rclsid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwclscontext: CLSCTX, flags: u32, lpdwregister: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoRegisterClassObject(::core::mem::transmute(rclsid), punk.into_param().abi(), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(flags), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(deviceinstanceid: Param0) -> ::windows::core::Result<CO_DEVICE_CATALOG_COOKIE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterDeviceCatalog(deviceinstanceid: ::windows::core::PCWSTR, cookie: *mut CO_DEVICE_CATALOG_COOKIE) -> ::windows::core::HRESULT;
        }
        let mut result__: CO_DEVICE_CATALOG_COOKIE = ::core::mem::zeroed();
        CoRegisterDeviceCatalog(deviceinstanceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<CO_DEVICE_CATALOG_COOKIE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoRegisterInitializeSpy<'a, Param0: ::windows::core::IntoParam<'a, IInitializeSpy>>(pspy: Param0) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterInitializeSpy(pspy: ::windows::core::RawPtr, pulicookie: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        CoRegisterInitializeSpy(pspy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSetCancelObject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetCancelObject(punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CoSetCancelObject(punk.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSetProxyBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSetProxyBlanket(pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT;
        }
        CoSetProxyBlanket(pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoSwitchCallContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pnewobject: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoSwitchCallContext(pnewobject: *mut ::core::ffi::c_void, ppoldobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoSwitchCallContext(pnewobject.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoTaskMemFree(pv: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoTaskMemFree(pv: *const ::core::ffi::c_void);
        }
        CoTaskMemFree(::core::mem::transmute(pv))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUninitialize();
        }
        CoUninitialize()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoWaitForMultipleHandles(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), phandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phandles)), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, chandles: u32, phandles: *const super::super::Foundation::HANDLE, lpdwindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        CoWaitForMultipleObjects(::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), phandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(phandles)), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ComCallData {}
impl ::core::clone::Clone for ComCallData {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ComCallData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ComCallData").field("dwDispid", &self.dwDispid).field("dwReserved", &self.dwReserved).field("pUserDefined", &self.pUserDefined).finish()
    }
}
unsafe impl ::windows::core::Abi for ComCallData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ComCallData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ComCallData>()) == 0 }
    }
}
impl ::core::cmp::Eq for ComCallData {}
impl ::core::default::Default for ComCallData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateAntiMoniker() -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAntiMoniker(ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateAntiMoniker(::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> ::windows::core::Result<IBindCtx> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBindCtx(reserved: u32, ppbc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateBindCtx(::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IBindCtx>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateClassMoniker(rclsid: *const ::windows::core::GUID, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateClassMoniker(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> ::windows::core::Result<IDataAdviseHolder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataAdviseHolder(ppdaholder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateDataAdviseHolder(::core::mem::transmute(&mut result__)).from_abi::<IDataAdviseHolder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateDataCache<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDataCache(punkouter: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, iid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CreateDataCache(punkouter.into_param().abi(), ::core::mem::transmute(rclsid), ::core::mem::transmute(iid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateFileMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszpathname: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMoniker(lpszpathname: ::windows::core::PCWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateFileMoniker(lpszpathname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateGenericComposite<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkfirst: Param0, pmkrest: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGenericComposite(pmkfirst: ::windows::core::RawPtr, pmkrest: ::windows::core::RawPtr, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateGenericComposite(pmkfirst.into_param().abi(), pmkrest.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateIUriBuilder<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(piuri: Param0, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUriBuilder> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIUriBuilder(piuri: ::windows::core::RawPtr, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateIUriBuilder(piuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUriBuilder>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateItemMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszdelim: Param0, lpszitem: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateItemMoniker(lpszdelim: ::windows::core::PCWSTR, lpszitem: ::windows::core::PCWSTR, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateItemMoniker(lpszdelim.into_param().abi(), lpszitem.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateObjrefMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateObjrefMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateObjrefMoniker(punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreatePointerMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punk: Param0) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePointerMoniker(punk: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreatePointerMoniker(punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStdProgressIndicator<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(hwndparent: Param0, psztitle: Param1, pibsccaller: Param2) -> ::windows::core::Result<IBindStatusCallback> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStdProgressIndicator(hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, pibsccaller: ::windows::core::RawPtr, ppibsc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateStdProgressIndicator(hwndparent.into_param().abi(), psztitle.into_param().abi(), pibsccaller.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IBindStatusCallback>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pwzuri: Param0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUri(pwzuri: ::windows::core::PCWSTR, dwflags: URI_CREATE_FLAGS, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUri(pwzuri.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUriFromMultiByteString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszansiinputuri: Param0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriFromMultiByteString(pszansiinputuri: ::windows::core::PCSTR, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUriFromMultiByteString(pszansiinputuri.into_param().abi(), ::core::mem::transmute(dwencodingflags), ::core::mem::transmute(dwcodepage), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn CreateUriWithFragment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pwzuri: Param0, pwzfragment: Param1, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUriWithFragment(pwzuri: ::windows::core::PCWSTR, pwzfragment: ::windows::core::PCWSTR, dwflags: u32, dwreserved: usize, ppuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateUriWithFragment(pwzuri.into_param().abi(), pwzfragment.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DATADIR(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
impl ::core::marker::Copy for DATADIR {}
impl ::core::clone::Clone for DATADIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DATADIR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DATADIR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DATADIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATADIR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOM_CALL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
impl ::core::marker::Copy for DCOM_CALL_STATE {}
impl ::core::clone::Clone for DCOM_CALL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOM_CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOM_CALL_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOM_CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOM_CALL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DESCKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
impl ::core::marker::Copy for DESCKIND {}
impl ::core::clone::Clone for DESCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DESCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DESCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for DESCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DESCKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct DISPPARAMS {
    pub rgvarg: *mut VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for DISPPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for DISPPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPPARAMS").field("rgvarg", &self.rgvarg).field("rgdispidNamedArgs", &self.rgdispidNamedArgs).field("cArgs", &self.cArgs).field("cNamedArgs", &self.cNamedArgs).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for DISPPARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for DISPPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISPPARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for DISPPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DMUS_ERRBASE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DVASPECT(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8i32);
impl ::core::marker::Copy for DVASPECT {}
impl ::core::clone::Clone for DVASPECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DVASPECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DVASPECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for DVASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DVASPECT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl ::core::marker::Copy for DVTARGETDEVICE {}
impl ::core::clone::Clone for DVTARGETDEVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DVTARGETDEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DVTARGETDEVICE").field("tdSize", &self.tdSize).field("tdDriverNameOffset", &self.tdDriverNameOffset).field("tdDeviceNameOffset", &self.tdDeviceNameOffset).field("tdPortNameOffset", &self.tdPortNameOffset).field("tdExtDevmodeOffset", &self.tdExtDevmodeOffset).field("tdData", &self.tdData).finish()
    }
}
unsafe impl ::windows::core::Abi for DVTARGETDEVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DVTARGETDEVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DVTARGETDEVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DVTARGETDEVICE {}
impl ::core::default::Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl ::core::marker::Copy for DWORD_BLOB {}
impl ::core::clone::Clone for DWORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWORD_BLOB").field("clSize", &self.clSize).field("alData", &self.alData).finish()
    }
}
unsafe impl ::windows::core::Abi for DWORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DWORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DWORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for DWORD_BLOB {}
impl ::core::default::Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for ELEMDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for ELEMDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for ELEMDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ELEMDESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for ELEMDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(128i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(256i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(512i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
impl ::core::marker::Copy for EOLE_AUTHENTICATION_CAPABILITIES {}
impl ::core::clone::Clone for EOLE_AUTHENTICATION_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOLE_AUTHENTICATION_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EOLE_AUTHENTICATION_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOLE_AUTHENTICATION_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOLE_AUTHENTICATION_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: super::super::Foundation::BSTR,
    pub bstrDescription: super::super::Foundation::BSTR,
    pub bstrHelpFile: super::super::Foundation::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        Self {
            wCode: self.wCode,
            wReserved: self.wReserved,
            bstrSource: self.bstrSource.clone(),
            bstrDescription: self.bstrDescription.clone(),
            bstrHelpFile: self.bstrHelpFile.clone(),
            dwHelpContext: self.dwHelpContext,
            pvReserved: self.pvReserved,
            pfnDeferredFillIn: self.pfnDeferredFillIn,
            scode: self.scode,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXCEPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXCEPINFO").field("wCode", &self.wCode).field("wReserved", &self.wReserved).field("bstrSource", &self.bstrSource).field("bstrDescription", &self.bstrDescription).field("bstrHelpFile", &self.bstrHelpFile).field("dwHelpContext", &self.dwHelpContext).field("pvReserved", &self.pvReserved).field("pfnDeferredFillIn", &self.pfnDeferredFillIn.map(|f| f as usize)).field("scode", &self.scode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXCEPINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EXTCONN(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
impl ::core::marker::Copy for EXTCONN {}
impl ::core::clone::Clone for EXTCONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXTCONN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EXTCONN {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXTCONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXTCONN").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl ::core::marker::Copy for FLAGGED_BYTE_BLOB {}
impl ::core::clone::Clone for FLAGGED_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_BYTE_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("abData", &self.abData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_BYTE_BLOB {}
impl ::core::default::Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for FLAGGED_WORD_BLOB {}
impl ::core::clone::Clone for FLAGGED_WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FLAGGED_WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLAGGED_WORD_BLOB").field("fFlags", &self.fFlags).field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for FLAGGED_WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FLAGGED_WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLAGGED_WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FLAGGED_WORD_BLOB {}
impl ::core::default::Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for FLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for FLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for FLAG_STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl ::core::marker::Copy for FORMATETC {}
impl ::core::clone::Clone for FORMATETC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
unsafe impl ::windows::core::Abi for FORMATETC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FORMATETC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FORMATETC>()) == 0 }
    }
}
impl ::core::cmp::Eq for FORMATETC {}
impl ::core::default::Default for FORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
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
impl ::core::marker::Copy for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for FUNCDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for FUNCDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for FUNCDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FUNCDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for FUNCDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for FUNCDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FUNCKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
impl ::core::marker::Copy for FUNCKIND {}
impl ::core::clone::Clone for FUNCKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FUNCKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FUNCKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for FUNCKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FUNCKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for GDI_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for GDI_OBJECT_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for GDI_OBJECT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GDI_OBJECT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for GDI_OBJECT_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_EH_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_EH_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_EH_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_EH_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_EH_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_EH_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
impl ::core::marker::Copy for GLOBALOPT_PROPERTIES {}
impl ::core::clone::Clone for GLOBALOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(128i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
impl ::core::marker::Copy for GLOBALOPT_RO_FLAGS {}
impl ::core::clone::Clone for GLOBALOPT_RO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_RO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RO_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
impl ::core::marker::Copy for GLOBALOPT_RPCTP_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_RPCTP_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_RPCTP_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_RPCTP_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_RPCTP_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RPCTP_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
impl ::core::marker::Copy for GLOBALOPT_UNMARSHALING_POLICY_VALUES {}
impl ::core::clone::Clone for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBALOPT_UNMARSHALING_POLICY_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetClassFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(szfilename: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetClassFile(szfilename: ::windows::core::PCWSTR, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        GetClassFile(szfilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> ::windows::core::Result<IErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetErrorInfo(dwreserved: u32, pperrinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        GetErrorInfo(::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> ::windows::core::Result<IRunningObjectTable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRunningObjectTable(reserved: u32, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        GetRunningObjectTable(::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<IRunningObjectTable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl ::core::marker::Copy for HYPER_SIZEDARR {}
impl ::core::clone::Clone for HYPER_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYPER_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPER_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for HYPER_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HYPER_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HYPER_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HYPER_SIZEDARR {}
impl ::core::default::Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IActivationFilter(::windows::core::IUnknown);
impl IActivationFilter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HandleActivation)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwactivationtype), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: IActivationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivationFilter> for ::windows::core::IUnknown {
    fn from(value: &IActivationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivationFilter {}
impl ::core::fmt::Debug for IActivationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivationFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivationFilter {
    type Vtable = IActivationFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000017_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub HandleActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAddrExclusionControl(::windows::core::IUnknown);
impl IAddrExclusionControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentAddrExclusionList)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppenumerator)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn UpdateAddrExclusionList<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, penumerator: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateAddrExclusionList)(::core::mem::transmute_copy(self), penumerator.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: IAddrExclusionControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrExclusionControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrExclusionControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAddrExclusionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrExclusionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrExclusionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrExclusionControl {}
impl ::core::fmt::Debug for IAddrExclusionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrExclusionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAddrExclusionControl {
    type Vtable = IAddrExclusionControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000148_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrExclusionControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAddrTrackingControl(::windows::core::IUnknown);
impl IAddrTrackingControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableCOMDynamicAddrTracking)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableCOMDynamicAddrTracking)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: IAddrTrackingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAddrTrackingControl> for ::windows::core::IUnknown {
    fn from(value: &IAddrTrackingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAddrTrackingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAddrTrackingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAddrTrackingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAddrTrackingControl {}
impl ::core::fmt::Debug for IAddrTrackingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAddrTrackingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAddrTrackingControl {
    type Vtable = IAddrTrackingControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000147_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddrTrackingControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAdviseSink(::windows::core::IUnknown);
impl IAdviseSink {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).OnDataChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).OnViewChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).OnRename)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Interface::vtable(self).OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Interface::vtable(self).OnClose)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdviseSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink {}
impl ::core::fmt::Debug for IAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink {
    type Vtable = IAdviseSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataChange: usize,
    pub OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32),
    pub OnRename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
    pub OnSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub OnClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAdviseSink2(::windows::core::IUnknown);
impl IAdviseSink2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (::windows::core::Interface::vtable(self).base.OnDataChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (::windows::core::Interface::vtable(self).base.OnViewChange)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwaspect), ::core::mem::transmute(lindex))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnRename<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).base.OnRename)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnSave(&self) {
        (::windows::core::Interface::vtable(self).base.OnSave)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnClose(&self) {
        (::windows::core::Interface::vtable(self).base.OnClose)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnLinkSrcChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmk: Param0) {
        (::windows::core::Interface::vtable(self).OnLinkSrcChange)(::core::mem::transmute_copy(self), pmk.into_param().abi())
    }
}
impl ::core::convert::From<IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: IAdviseSink2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdviseSink2> for ::windows::core::IUnknown {
    fn from(value: &IAdviseSink2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAdviseSink> for &'a IAdviseSink2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAdviseSink> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdviseSink2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdviseSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdviseSink2 {}
impl ::core::fmt::Debug for IAdviseSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdviseSink2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAdviseSink2 {
    type Vtable = IAdviseSink2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000125_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_Vtbl {
    pub base: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAgileObject(::windows::core::IUnknown);
impl IAgileObject {}
impl ::core::convert::From<IAgileObject> for ::windows::core::IUnknown {
    fn from(value: IAgileObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAgileObject> for ::windows::core::IUnknown {
    fn from(value: &IAgileObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAgileObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAgileObject {}
impl ::core::fmt::Debug for IAgileObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAgileObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAsyncManager(::windows::core::IUnknown);
impl IAsyncManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CompleteCall(&self, result: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CompleteCall)(::core::mem::transmute_copy(self), ::core::mem::transmute(result)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCallContext(&self, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCallContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: IAsyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncManager> for ::windows::core::IUnknown {
    fn from(value: &IAsyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncManager {}
impl ::core::fmt::Debug for IAsyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsyncManager {
    type Vtable = IAsyncManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002a_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CompleteCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAsyncRpcChannelBuffer(::windows::core::IUnknown);
impl IAsyncRpcChannelBuffer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendReceive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDestCtx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.IsConnected)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProtocolVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Send<'a, Param1: ::windows::core::IntoParam<'a, ISynchronize>>(&self, pmsg: *mut RPCOLEMESSAGE, psync: Param1, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Send)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), psync.into_param().abi(), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Receive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDestCtxEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
}
impl ::core::convert::From<IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IAsyncRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsyncRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IAsyncRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &'a IAsyncRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAsyncRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncRpcChannelBuffer {}
impl ::core::fmt::Debug for IAsyncRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsyncRpcChannelBuffer {
    type Vtable = IAsyncRpcChannelBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAuthenticate(::windows::core::IUnknown);
impl IAuthenticate {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Authenticate)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
}
impl ::core::convert::From<IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: IAuthenticate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticate> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAuthenticate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticate {}
impl ::core::fmt::Debug for IAuthenticate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticate {
    type Vtable = IAuthenticate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticate_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Authenticate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Authenticate: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IAuthenticateEx(::windows::core::IUnknown);
impl IAuthenticateEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Authenticate)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AuthenticateEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(phwnd), ::core::mem::transmute(pszusername), ::core::mem::transmute(pszpassword), ::core::mem::transmute(pauthinfo)).ok()
    }
}
impl ::core::convert::From<IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: IAuthenticateEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticateEx> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticateEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAuthenticate> for &'a IAuthenticateEx {
    fn into_param(self) -> ::windows::core::Param<'a, IAuthenticate> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAuthenticateEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticateEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticateEx {}
impl ::core::fmt::Debug for IAuthenticateEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticateEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticateEx {
    type Vtable = IAuthenticateEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticateEx_Vtbl {
    pub base: IAuthenticate_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticateEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindCtx(::windows::core::IUnknown);
impl IBindCtx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterObjectBound)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RevokeObjectBound<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevokeObjectBound)(::core::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseBoundObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseBoundObjects)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBindOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindOptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbindopts)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRunningObjectTable(&self) -> ::windows::core::Result<IRunningObjectTable> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRunningObjectTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IRunningObjectTable>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterObjectParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pszkey: Param0, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterObjectParam)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetObjectParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetObjectParam)(::core::mem::transmute_copy(self), pszkey.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumObjectParam(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumObjectParam)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumString>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RevokeObjectParam<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszkey: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevokeObjectParam)(::core::mem::transmute_copy(self), pszkey.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindCtx> for ::windows::core::IUnknown {
    fn from(value: IBindCtx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindCtx> for ::windows::core::IUnknown {
    fn from(value: &IBindCtx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindCtx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindCtx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindCtx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCtx {}
impl ::core::fmt::Debug for IBindCtx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCtx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindCtx {
    type Vtable = IBindCtx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeObjectBound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseBoundObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT,
    pub GetBindOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT,
    pub GetRunningObjectTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegisterObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RevokeObjectParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindHost(::windows::core::IUnknown);
impl IBindHost {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateMoniker<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IBindCtx>>(&self, szname: Param0, pbc: Param1, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateMoniker)(::core::mem::transmute_copy(self), szname.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(ppmk), ::core::mem::transmute(dwreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn MonikerBindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonikerBindToStorage)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn MonikerBindToObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IBindCtx>, Param2: ::windows::core::IntoParam<'a, IBindStatusCallback>>(&self, pmk: Param0, pbc: Param1, pbsc: Param2, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonikerBindToObject)(::core::mem::transmute_copy(self), pmk.into_param().abi(), pbc.into_param().abi(), pbsc.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
}
impl ::core::convert::From<IBindHost> for ::windows::core::IUnknown {
    fn from(value: IBindHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindHost> for ::windows::core::IUnknown {
    fn from(value: &IBindHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindHost {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHost {}
impl ::core::fmt::Debug for IBindHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHost").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindHost {
    type Vtable = IBindHost_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindHost_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, pbc: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT,
    pub MonikerBindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MonikerBindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindStatusCallback(::windows::core::IUnknown);
impl IBindStatusCallback {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStartBinding)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLowResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStopBinding)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDataAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnObjectAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallback {}
impl ::core::fmt::Debug for IBindStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallback {
    type Vtable = IBindStatusCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnStartBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub OnLowResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnStopBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataAvailable: usize,
    pub OnObjectAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBindStatusCallbackEx(::windows::core::IUnknown);
impl IBindStatusCallbackEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnStartBinding<'a, Param1: ::windows::core::IntoParam<'a, IBinding>>(&self, dwreserved: u32, pib: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnStartBinding)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved), pib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnLowResource)(::core::mem::transmute_copy(self), ::core::mem::transmute(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnProgress<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulprogress), ::core::mem::transmute(ulprogressmax), ::core::mem::transmute(ulstatuscode), szstatustext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnStopBinding<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hresult: ::windows::core::HRESULT, szerror: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnStopBinding)(::core::mem::transmute_copy(self), ::core::mem::transmute(hresult), szerror.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBindInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnDataAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbscf), ::core::mem::transmute(dwsize), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pstgmed)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OnObjectAvailable<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.OnObjectAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindInfoEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfbindf), ::core::mem::transmute(pbindinfo), ::core::mem::transmute(grfbindf2), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: IBindStatusCallbackEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBindStatusCallbackEx> for ::windows::core::IUnknown {
    fn from(value: &IBindStatusCallbackEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IBindStatusCallback> for &'a IBindStatusCallbackEx {
    fn into_param(self) -> ::windows::core::Param<'a, IBindStatusCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBindStatusCallbackEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBindStatusCallbackEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindStatusCallbackEx {}
impl ::core::fmt::Debug for IBindStatusCallbackEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindStatusCallbackEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBindStatusCallbackEx {
    type Vtable = IBindStatusCallbackEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBinding(::windows::core::IUnknown);
impl IBinding {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Suspend)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(npriority)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPriority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBindResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(pclsidprotocol), ::core::mem::transmute(pdwresult), ::core::mem::transmute(pszresult), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IBinding> for ::windows::core::IUnknown {
    fn from(value: IBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBinding> for ::windows::core::IUnknown {
    fn from(value: &IBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBinding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBinding {}
impl ::core::fmt::Debug for IBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBinding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Suspend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IBlockingLock(::windows::core::IUnknown);
impl IBlockingLock {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Lock(&self, dwtimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Lock)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwtimeout)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: IBlockingLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBlockingLock> for ::windows::core::IUnknown {
    fn from(value: &IBlockingLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBlockingLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBlockingLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBlockingLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBlockingLock {}
impl ::core::fmt::Debug for IBlockingLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBlockingLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBlockingLock {
    type Vtable = IBlockingLock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICallFactory(::windows::core::IUnknown);
impl ICallFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateCall<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, pctrlunk: Param1, riid2: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCall)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pctrlunk.into_param().abi(), ::core::mem::transmute(riid2), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ICallFactory> for ::windows::core::IUnknown {
    fn from(value: ICallFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICallFactory> for ::windows::core::IUnknown {
    fn from(value: &ICallFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICallFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICallFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICallFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICallFactory {}
impl ::core::fmt::Debug for ICallFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICallFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICallFactory {
    type Vtable = ICallFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICallFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICancelMethodCalls(::windows::core::IUnknown);
impl ICancelMethodCalls {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Cancel(&self, ulseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulseconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn TestCancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TestCancel)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: ICancelMethodCalls) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICancelMethodCalls> for ::windows::core::IUnknown {
    fn from(value: &ICancelMethodCalls) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICancelMethodCalls {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICancelMethodCalls {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICancelMethodCalls {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICancelMethodCalls {}
impl ::core::fmt::Debug for ICancelMethodCalls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICancelMethodCalls").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICancelMethodCalls {
    type Vtable = ICancelMethodCalls_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000029_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICancelMethodCalls_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICatInformation(::windows::core::IUnknown);
impl ICatInformation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumCategories(&self, lcid: u32) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumCATEGORYINFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const ::windows::core::GUID, lcid: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCategoryDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(rcatid), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumClassesOfCategories(&self, rgcatidimpl: &[::windows::core::GUID], rgcatidreq: &[::windows::core::GUID]) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumClassesOfCategories)(::core::mem::transmute_copy(self), rgcatidimpl.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatidimpl)), rgcatidreq.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatidreq)), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const ::windows::core::GUID, rgcatidimpl: &[::windows::core::GUID], rgcatidreq: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsClassOfCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), rgcatidimpl.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatidimpl)), rgcatidreq.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatidreq))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumImplCategoriesOfClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumReqCategoriesOfClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<ICatInformation> for ::windows::core::IUnknown {
    fn from(value: ICatInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatInformation> for ::windows::core::IUnknown {
    fn from(value: &ICatInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatInformation {}
impl ::core::fmt::Debug for ICatInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatInformation {
    type Vtable = ICatInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e013_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatInformation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCategoryDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub EnumClassesOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ICatRegister(::windows::core::IUnknown);
impl ICatRegister {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterCategories)(::core::mem::transmute_copy(self), rgcategoryinfo.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcategoryinfo))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnRegisterCategories)(::core::mem::transmute_copy(self), rgcatid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterClassImplCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnRegisterClassImplCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterClassReqCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, rgcatid: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnRegisterClassReqCategories)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), rgcatid.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgcatid))).ok()
    }
}
impl ::core::convert::From<ICatRegister> for ::windows::core::IUnknown {
    fn from(value: ICatRegister) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICatRegister> for ::windows::core::IUnknown {
    fn from(value: &ICatRegister) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICatRegister {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICatRegister {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICatRegister {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatRegister {}
impl ::core::fmt::Debug for ICatRegister {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatRegister").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICatRegister {
    type Vtable = ICatRegister_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e012_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICatRegister_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT,
    pub UnRegisterCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IChannelHook(::windows::core::IUnknown);
impl IChannelHook {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ClientGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
        (::windows::core::Interface::vtable(self).ClientGetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ClientFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).ClientFillBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ClientNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).ClientNotify)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep), ::core::mem::transmute(hrfault))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ServerNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
        (::windows::core::Interface::vtable(self).ServerNotify)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(cbdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(ldatarep))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ServerGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
        (::windows::core::Interface::vtable(self).ServerGetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(hrfault), ::core::mem::transmute(pdatasize))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ServerFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).ServerFillBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(uextent), ::core::mem::transmute(riid), ::core::mem::transmute(pdatasize), ::core::mem::transmute(pdatabuffer), ::core::mem::transmute(hrfault))
    }
}
impl ::core::convert::From<IChannelHook> for ::windows::core::IUnknown {
    fn from(value: IChannelHook) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelHook> for ::windows::core::IUnknown {
    fn from(value: &IChannelHook) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IChannelHook {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChannelHook {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelHook {}
impl ::core::fmt::Debug for IChannelHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelHook").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChannelHook {
    type Vtable = IChannelHook_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelHook_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ClientGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32),
    pub ServerGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClassActivator(::windows::core::IUnknown);
impl IClassActivator {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassObject<T: ::windows::core::Interface>(&self, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetClassObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(locale), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IClassActivator> for ::windows::core::IUnknown {
    fn from(value: IClassActivator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassActivator> for ::windows::core::IUnknown {
    fn from(value: &IClassActivator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClassActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassActivator {}
impl ::core::fmt::Debug for IClassActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClassActivator {
    type Vtable = IClassActivator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000140_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetClassObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClassFactory(::windows::core::IUnknown);
impl IClassFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInstance)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockServer)(::core::mem::transmute_copy(self), flock.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IClassFactory> for ::windows::core::IUnknown {
    fn from(value: IClassFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClassFactory> for ::windows::core::IUnknown {
    fn from(value: &IClassFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClassFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClassFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClassFactory {}
impl ::core::fmt::Debug for IClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClassFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClassFactory {
    type Vtable = IClassFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LockServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockServer: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IClientSecurity(::windows::core::IUnknown);
impl IClientSecurity {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryBlanket)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(pcapabilites)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetBlanket<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pproxy: Param0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: Param3, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlanket)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(dwauthnsvc), ::core::mem::transmute(dwauthzsvc), pserverprincname.into_param().abi(), ::core::mem::transmute(dwauthnlevel), ::core::mem::transmute(dwimplevel), ::core::mem::transmute(pauthinfo), ::core::mem::transmute(dwcapabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CopyProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pproxy: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CopyProxy)(::core::mem::transmute_copy(self), pproxy.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: IClientSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClientSecurity> for ::windows::core::IUnknown {
    fn from(value: &IClientSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClientSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IClientSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClientSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClientSecurity {}
impl ::core::fmt::Debug for IClientSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClientSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClientSecurity {
    type Vtable = IClientSecurity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClientSecurity_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    pub SetBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT,
    pub CopyProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IComThreadingInfo(::windows::core::IUnknown);
impl IComThreadingInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCurrentApartmentType(&self) -> ::windows::core::Result<APTTYPE> {
        let mut result__: APTTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentApartmentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<APTTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCurrentThreadType(&self) -> ::windows::core::Result<THDTYPE> {
        let mut result__: THDTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentThreadType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<THDTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentLogicalThreadId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCurrentLogicalThreadId)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguid)).ok()
    }
}
impl ::core::convert::From<IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: IComThreadingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IComThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: &IComThreadingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IComThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IComThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IComThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComThreadingInfo {}
impl ::core::fmt::Debug for IComThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComThreadingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComThreadingInfo {
    type Vtable = IComThreadingInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001ce_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComThreadingInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IConnectionPoint(::windows::core::IUnknown);
impl IConnectionPoint {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetConnectionPointContainer(&self) -> ::windows::core::Result<IConnectionPointContainer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectionPointContainer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConnectionPointContainer>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punksink: Param0) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Advise)(::core::mem::transmute_copy(self), punksink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumConnections(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumConnections)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: IConnectionPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPoint> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnectionPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPoint {}
impl ::core::fmt::Debug for IConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnectionPoint {
    type Vtable = IConnectionPoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b286_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPoint_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcpc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IConnectionPointContainer(::windows::core::IUnknown);
impl IConnectionPointContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumConnectionPoints(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumConnectionPoints)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnectionPoints>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<IConnectionPoint> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FindConnectionPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<IConnectionPoint>(result__)
    }
}
impl ::core::convert::From<IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: IConnectionPointContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnectionPointContainer> for ::windows::core::IUnknown {
    fn from(value: &IConnectionPointContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnectionPointContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnectionPointContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnectionPointContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionPointContainer {}
impl ::core::fmt::Debug for IConnectionPointContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionPointContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnectionPointContainer {
    type Vtable = IConnectionPointContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b284_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionPointContainer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IContext(pub u8);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IContextCallback(::windows::core::IUnknown);
impl IContextCallback {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ContextCallback<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ContextCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pparam), ::core::mem::transmute(riid), ::core::mem::transmute(imethod), punk.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IContextCallback> for ::windows::core::IUnknown {
    fn from(value: IContextCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IContextCallback> for ::windows::core::IUnknown {
    fn from(value: &IContextCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContextCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IContextCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IContextCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextCallback {}
impl ::core::fmt::Debug for IContextCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextCallback {
    type Vtable = IContextCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001da_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ContextCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: ::windows::core::RawPtr, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: u16,
}
impl ::core::marker::Copy for IDLDESC {}
impl ::core::clone::Clone for IDLDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDLDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDLDESC").field("dwReserved", &self.dwReserved).field("wIDLFlags", &self.wIDLFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for IDLDESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDLDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IDLDESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IDLDESC {}
impl ::core::default::Default for IDLDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDataAdviseHolder(::windows::core::IUnknown);
impl IDataAdviseHolder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>, Param3: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pdataobject: Param0, pfetc: *const FORMATETC, advf: u32, padvise: Param3) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Advise)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(pfetc), ::core::mem::transmute(advf), padvise.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumAdvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SendOnDataChange<'a, Param0: ::windows::core::IntoParam<'a, IDataObject>>(&self, pdataobject: Param0, dwreserved: u32, advf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendOnDataChange)(::core::mem::transmute_copy(self), pdataobject.into_param().abi(), ::core::mem::transmute(dwreserved), ::core::mem::transmute(advf)).ok()
    }
}
impl ::core::convert::From<IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: IDataAdviseHolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataAdviseHolder> for ::windows::core::IUnknown {
    fn from(value: &IDataAdviseHolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataAdviseHolder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataAdviseHolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataAdviseHolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataAdviseHolder {}
impl ::core::fmt::Debug for IDataAdviseHolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataAdviseHolder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataAdviseHolder {
    type Vtable = IDataAdviseHolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000110_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDataObject(::windows::core::IUnknown);
impl IDataObject {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::core::Result<STGMEDIUM> {
        let mut result__: ::core::mem::ManuallyDrop<STGMEDIUM> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetcin), ::core::mem::transmute(&mut result__)).from_abi::<STGMEDIUM>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataHere)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryGetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC) -> ::windows::core::Result<FORMATETC> {
        let mut result__: FORMATETC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCanonicalFormatEtc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatectin), ::core::mem::transmute(&mut result__)).from_abi::<FORMATETC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(pmedium), frelease.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumFormatEtc)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdirection), ::core::mem::transmute(&mut result__)).from_abi::<IEnumFORMATETC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DAdvise<'a, Param2: ::windows::core::IntoParam<'a, IAdviseSink>>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DAdvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformatetc), ::core::mem::transmute(advf), padvsink.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DUnadvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconnection)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumDAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumDAdvise)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IDataObject> for ::windows::core::IUnknown {
    fn from(value: IDataObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataObject> for ::windows::core::IUnknown {
    fn from(value: &IDataObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDataObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataObject {}
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataObject {
    type Vtable = IDataObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetData: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetDataHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetDataHere: usize,
    pub QueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT,
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IDispatch(::windows::core::IUnknown);
impl IDispatch {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeInfoCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetIDsOfNames<'a, const PARAM2: usize>(&self, riid: *const ::windows::core::GUID, rgsznames: &[::windows::core::PWSTR; PARAM2], lcid: u32, rgdispid: &mut [i32; PARAM2]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIDsOfNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsznames)), PARAM2 as _, ::core::mem::transmute(lcid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgdispid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::core::mem::transmute_copy(self), ::core::mem::transmute(dispidmember), ::core::mem::transmute(riid), ::core::mem::transmute(lcid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
}
impl ::core::convert::From<IDispatch> for ::windows::core::IUnknown {
    fn from(value: IDispatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDispatch> for ::windows::core::IUnknown {
    fn from(value: &IDispatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDispatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDispatch {}
impl ::core::fmt::Debug for IDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDispatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDispatch {
    type Vtable = IDispatch_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020400_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const ::windows::core::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumCATEGORYINFO(::windows::core::IUnknown);
impl IEnumCATEGORYINFO {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [CATEGORYINFO], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumCATEGORYINFO> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumCATEGORYINFO>(result__)
    }
}
impl ::core::convert::From<IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: IEnumCATEGORYINFO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumCATEGORYINFO> for ::windows::core::IUnknown {
    fn from(value: &IEnumCATEGORYINFO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumCATEGORYINFO {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumCATEGORYINFO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumCATEGORYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCATEGORYINFO {}
impl ::core::fmt::Debug for IEnumCATEGORYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCATEGORYINFO").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumCATEGORYINFO {
    type Vtable = IEnumCATEGORYINFO_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e011_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumConnectionPoints(::windows::core::IUnknown);
impl IEnumConnectionPoints {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, ppcp: &mut [::core::option::Option<IConnectionPoint>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ppcp.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppcp)), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnectionPoints> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnectionPoints>(result__)
    }
}
impl ::core::convert::From<IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: IEnumConnectionPoints) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnectionPoints> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnectionPoints) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumConnectionPoints {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnectionPoints {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnectionPoints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnectionPoints {}
impl ::core::fmt::Debug for IEnumConnectionPoints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnectionPoints").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumConnectionPoints {
    type Vtable = IEnumConnectionPoints_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b285_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumConnections(::windows::core::IUnknown);
impl IEnumConnections {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgcd: &mut [CONNECTDATA], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgcd.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgcd)), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnections)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumConnections> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumConnections>(result__)
    }
}
impl ::core::convert::From<IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: IEnumConnections) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumConnections> for ::windows::core::IUnknown {
    fn from(value: &IEnumConnections) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumConnections {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumConnections {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumConnections {}
impl ::core::fmt::Debug for IEnumConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumConnections").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumConnections {
    type Vtable = IEnumConnections_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb196b287_bab4_101a_b69c_00aa00341d07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumConnections_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IEnumContextProps(pub u8);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumFORMATETC(::windows::core::IUnknown);
impl IEnumFORMATETC {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFORMATETC> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumFORMATETC>(result__)
    }
}
impl ::core::convert::From<IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: IEnumFORMATETC) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFORMATETC> for ::windows::core::IUnknown {
    fn from(value: &IEnumFORMATETC) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumFORMATETC {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumFORMATETC {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFORMATETC {}
impl ::core::fmt::Debug for IEnumFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFORMATETC").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumFORMATETC {
    type Vtable = IEnumFORMATETC_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumGUID(::windows::core::IUnknown);
impl IEnumGUID {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::GUID], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumGUID> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumGUID>(result__)
    }
}
impl ::core::convert::From<IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: IEnumGUID) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumGUID> for ::windows::core::IUnknown {
    fn from(value: &IEnumGUID) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumGUID {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumGUID {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumGUID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumGUID {}
impl ::core::fmt::Debug for IEnumGUID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumGUID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumGUID {
    type Vtable = IEnumGUID_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0002e000_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumGUID_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumMoniker(::windows::core::IUnknown);
impl IEnumMoniker {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IMoniker>], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: IEnumMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumMoniker> for ::windows::core::IUnknown {
    fn from(value: &IEnumMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMoniker {}
impl ::core::fmt::Debug for IEnumMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumMoniker {
    type Vtable = IEnumMoniker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000102_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumSTATDATA(::windows::core::IUnknown);
impl IEnumSTATDATA {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSTATDATA> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumSTATDATA>(result__)
    }
}
impl ::core::convert::From<IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: IEnumSTATDATA) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSTATDATA> for ::windows::core::IUnknown {
    fn from(value: &IEnumSTATDATA) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumSTATDATA {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumSTATDATA {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSTATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSTATDATA {}
impl ::core::fmt::Debug for IEnumSTATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSTATDATA").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSTATDATA {
    type Vtable = IEnumSTATDATA_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumString(::windows::core::IUnknown);
impl IEnumString {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [::windows::core::PWSTR], pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumString> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumString>(result__)
    }
}
impl ::core::convert::From<IEnumString> for ::windows::core::IUnknown {
    fn from(value: IEnumString) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumString> for ::windows::core::IUnknown {
    fn from(value: &IEnumString) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumString {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumString {}
impl ::core::fmt::Debug for IEnumString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumString {
    type Vtable = IEnumString_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000101_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumString_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IEnumUnknown(::windows::core::IUnknown);
impl IEnumUnknown {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<::windows::core::IUnknown>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: IEnumUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumUnknown> for ::windows::core::IUnknown {
    fn from(value: &IEnumUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumUnknown {}
impl ::core::fmt::Debug for IEnumUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumUnknown {
    type Vtable = IEnumUnknown_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000100_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumUnknown_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IErrorInfo(::windows::core::IUnknown);
impl IErrorInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGUID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSource)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDescription)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetHelpContext(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: IErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &IErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorInfo {}
impl ::core::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSource: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHelpFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHelpFile: usize,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IErrorLog(::windows::core::IUnknown);
impl IErrorLog {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddError<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpropname: Param0, pexcepinfo: *const EXCEPINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddError)(::core::mem::transmute_copy(self), pszpropname.into_param().abi(), ::core::mem::transmute(pexcepinfo)).ok()
    }
}
impl ::core::convert::From<IErrorLog> for ::windows::core::IUnknown {
    fn from(value: IErrorLog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IErrorLog> for ::windows::core::IUnknown {
    fn from(value: &IErrorLog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IErrorLog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IErrorLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IErrorLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLog {}
impl ::core::fmt::Debug for IErrorLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IErrorLog {
    type Vtable = IErrorLog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3127ca40_446e_11ce_8135_00aa004bb851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorLog_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddError: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IExternalConnection(::windows::core::IUnknown);
impl IExternalConnection {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).AddConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseConnection<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, extconn: u32, reserved: u32, flastreleasecloses: Param2) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).ReleaseConnection)(::core::mem::transmute_copy(self), ::core::mem::transmute(extconn), ::core::mem::transmute(reserved), flastreleasecloses.into_param().abi()))
    }
}
impl ::core::convert::From<IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: IExternalConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExternalConnection> for ::windows::core::IUnknown {
    fn from(value: &IExternalConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IExternalConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExternalConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExternalConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExternalConnection {}
impl ::core::fmt::Debug for IExternalConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExternalConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExternalConnection {
    type Vtable = IExternalConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000019_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExternalConnection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseConnection: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IFastRundown(::windows::core::IUnknown);
impl IFastRundown {}
impl ::core::convert::From<IFastRundown> for ::windows::core::IUnknown {
    fn from(value: IFastRundown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFastRundown> for ::windows::core::IUnknown {
    fn from(value: &IFastRundown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFastRundown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFastRundown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFastRundown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFastRundown {}
impl ::core::fmt::Debug for IFastRundown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFastRundown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFastRundown {
    type Vtable = IFastRundown_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFastRundown_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IForegroundTransfer(::windows::core::IUnknown);
impl IForegroundTransfer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllowForegroundTransfer)(::core::mem::transmute_copy(self), ::core::mem::transmute(lpvreserved)).ok()
    }
}
impl ::core::convert::From<IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: IForegroundTransfer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForegroundTransfer> for ::windows::core::IUnknown {
    fn from(value: &IForegroundTransfer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IForegroundTransfer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IForegroundTransfer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForegroundTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForegroundTransfer {}
impl ::core::fmt::Debug for IForegroundTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForegroundTransfer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IForegroundTransfer {
    type Vtable = IForegroundTransfer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000145_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IGlobalInterfaceTable(::windows::core::IUnknown);
impl IGlobalInterfaceTable {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterInterfaceInGlobal<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punk: Param0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterInterfaceInGlobal)(::core::mem::transmute_copy(self), punk.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevokeInterfaceFromGlobal)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInterfaceFromGlobal)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcookie), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: IGlobalInterfaceTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalInterfaceTable> for ::windows::core::IUnknown {
    fn from(value: &IGlobalInterfaceTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGlobalInterfaceTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalInterfaceTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalInterfaceTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalInterfaceTable {}
impl ::core::fmt::Debug for IGlobalInterfaceTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalInterfaceTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGlobalInterfaceTable {
    type Vtable = IGlobalInterfaceTable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000146_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IGlobalOptions(::windows::core::IUnknown);
impl IGlobalOptions {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Query)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwproperty), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: IGlobalOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGlobalOptions> for ::windows::core::IUnknown {
    fn from(value: &IGlobalOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGlobalOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGlobalOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGlobalOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGlobalOptions {}
impl ::core::fmt::Debug for IGlobalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGlobalOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGlobalOptions {
    type Vtable = IGlobalOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000015b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGlobalOptions_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn IIDFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpsz: Param0) -> ::windows::core::Result<::windows::core::GUID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IIDFromString(lpsz: ::windows::core::PCWSTR, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        IIDFromString(lpsz.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IInitializeSpy(::windows::core::IUnknown);
impl IInitializeSpy {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreInitialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PostInitialize(&self, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostInitialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrcoinit), ::core::mem::transmute(dwcoinit), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreUninitialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcurthreadaptrefs)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostUninitialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwnewthreadaptrefs)).ok()
    }
}
impl ::core::convert::From<IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: IInitializeSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInitializeSpy> for ::windows::core::IUnknown {
    fn from(value: &IInitializeSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInitializeSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInitializeSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInitializeSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInitializeSpy {}
impl ::core::fmt::Debug for IInitializeSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInitializeSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInitializeSpy {
    type Vtable = IInitializeSpy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000034_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub PreInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IInternalUnknown(::windows::core::IUnknown);
impl IInternalUnknown {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryInternalInterface(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryInternalInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: IInternalUnknown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInternalUnknown> for ::windows::core::IUnknown {
    fn from(value: &IInternalUnknown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInternalUnknown {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInternalUnknown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInternalUnknown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalUnknown {}
impl ::core::fmt::Debug for IInternalUnknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalUnknown").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternalUnknown {
    type Vtable = IInternalUnknown_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000021_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalUnknown_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMachineGlobalObjectTable(::windows::core::IUnknown);
impl IMachineGlobalObjectTable {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, clsid: *const ::windows::core::GUID, identifier: Param1, object: Param2) -> ::windows::core::Result<*mut MachineGlobalObjectTableRegistrationToken__> {
        let mut result__: *mut MachineGlobalObjectTableRegistrationToken__ = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), object.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<*mut MachineGlobalObjectTableRegistrationToken__>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, T: ::windows::core::Interface>(&self, clsid: *const ::windows::core::GUID, identifier: Param1) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid), identifier.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevokeObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(token)).ok()
    }
}
impl ::core::convert::From<IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: IMachineGlobalObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMachineGlobalObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IMachineGlobalObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMachineGlobalObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMachineGlobalObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMachineGlobalObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMachineGlobalObjectTable {}
impl ::core::fmt::Debug for IMachineGlobalObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMachineGlobalObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMachineGlobalObjectTable {
    type Vtable = IMachineGlobalObjectTable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26d709ac_f70b_4421_a96f_d2878fafb00d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMalloc(::windows::core::IUnknown);
impl IMalloc {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Alloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).Realloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Free(&self, pv: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).Free)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).DidAlloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn HeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).HeapMinimize)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IMalloc> for ::windows::core::IUnknown {
    fn from(value: IMalloc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMalloc> for ::windows::core::IUnknown {
    fn from(value: &IMalloc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMalloc {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMalloc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMalloc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMalloc {}
impl ::core::fmt::Debug for IMalloc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMalloc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMalloc {
    type Vtable = IMalloc_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000002_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMalloc_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Alloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void,
    pub Free: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMallocSpy(::windows::core::IUnknown);
impl IMallocSpy {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PreAlloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbrequest)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PostAlloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreFree<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PreFree)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostFree<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fspyed: Param0) {
        (::windows::core::Interface::vtable(self).PostFree)(::core::mem::transmute_copy(self), fspyed.into_param().abi())
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreRealloc<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: Param3) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PreRealloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), ::core::mem::transmute(cbrequest), ::core::mem::transmute(ppnewrequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostRealloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pactual: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PostRealloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pactual), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PreGetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostGetSize<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, cbactual: usize, fspyed: Param1) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PostGetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(cbactual), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PreDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1) -> *mut ::core::ffi::c_void {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PreDidAlloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi()))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PostDidAlloc<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, prequest: *const ::core::ffi::c_void, fspyed: Param1, factual: i32) -> i32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).PostDidAlloc)(::core::mem::transmute_copy(self), ::core::mem::transmute(prequest), fspyed.into_param().abi(), ::core::mem::transmute(factual)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PreHeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).PreHeapMinimize)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn PostHeapMinimize(&self) {
        (::windows::core::Interface::vtable(self).PostHeapMinimize)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: IMallocSpy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMallocSpy> for ::windows::core::IUnknown {
    fn from(value: &IMallocSpy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMallocSpy {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMallocSpy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMallocSpy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMallocSpy {}
impl ::core::fmt::Debug for IMallocSpy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMallocSpy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMallocSpy {
    type Vtable = IMallocSpy_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000001d_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub PreAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    #[cfg(feature = "Win32_Foundation")]
    pub PreFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreFree: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    PostFree: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreRealloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostRealloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostRealloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreGetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostGetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostGetSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PreDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void,
    #[cfg(not(feature = "Win32_Foundation"))]
    PreDidAlloc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PostDidAlloc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32,
    #[cfg(not(feature = "Win32_Foundation"))]
    PostDidAlloc: usize,
    pub PreHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMoniker(::windows::core::IUnknown);
impl IMoniker {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Load)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Save)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn BindToObject<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindToObject)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riidresult), ::core::mem::transmute(ppvresult)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn BindToStorage<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindToStorage)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobj)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reduce<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reduce)(::core::mem::transmute_copy(self), pbc.into_param().abi(), ::core::mem::transmute(dwreducehowfar), ::core::mem::transmute(ppmktoleft), ::core::mem::transmute(ppmkreduced)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComposeWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmkright: Param0, fonlyifnotgeneric: Param1) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ComposeWith)(::core::mem::transmute_copy(self), pmkright.into_param().abi(), fonlyifnotgeneric.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fforward: Param0) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enum)(::core::mem::transmute_copy(self), fforward.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkothermoniker: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsEqual)(::core::mem::transmute_copy(self), pmkothermoniker.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Hash(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Hash)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1, pmknewlyrunning: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsRunning)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pmknewlyrunning.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTimeOfLastChange)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Inverse(&self) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Inverse)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CommonPrefixWith)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RelativePathTo<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkother: Param0) -> ::windows::core::Result<IMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RelativePathTo)(::core::mem::transmute_copy(self), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(&self, pbc: Param0, pmktoleft: Param1) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, IMoniker>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pbc: Param0, pmktoleft: Param1, pszdisplayname: Param2, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ParseDisplayName)(::core::mem::transmute_copy(self), pbc.into_param().abi(), pmktoleft.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmkout)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsSystemMoniker(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsSystemMoniker)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMoniker> for ::windows::core::IUnknown {
    fn from(value: IMoniker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMoniker> for ::windows::core::IUnknown {
    fn from(value: &IMoniker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &'a IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersistStream> for &'a IMoniker {
    fn into_param(self) -> ::windows::core::Param<'a, IPersistStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMoniker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMoniker {}
impl ::core::fmt::Debug for IMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMoniker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMoniker {
    type Vtable = IMoniker_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000f_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_Vtbl {
    pub base: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reduce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::core::RawPtr, ppmkreduced: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ComposeWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkright: ::windows::core::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComposeWith: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enum: usize,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkothermoniker: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Hash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pmknewlyrunning: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeOfLastChange: usize,
    pub Inverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CommonPrefixWith: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkprefix: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RelativePathTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, ppszdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub ParseDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pszdisplayname: ::windows::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsSystemMoniker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IMultiQI(::windows::core::IUnknown);
impl IMultiQI {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryMultipleInterfaces)(::core::mem::transmute_copy(self), pmqis.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pmqis))).ok()
    }
}
impl ::core::convert::From<IMultiQI> for ::windows::core::IUnknown {
    fn from(value: IMultiQI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiQI> for ::windows::core::IUnknown {
    fn from(value: &IMultiQI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMultiQI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiQI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiQI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiQI {}
impl ::core::fmt::Debug for IMultiQI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiQI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiQI {
    type Vtable = IMultiQI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000020_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiQI_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct INTERFACEINFO {
    pub pUnk: ::core::option::Option<::windows::core::IUnknown>,
    pub iid: ::windows::core::GUID,
    pub wMethod: u16,
}
impl ::core::clone::Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        Self { pUnk: self.pUnk.clone(), iid: self.iid, wMethod: self.wMethod }
    }
}
impl ::core::fmt::Debug for INTERFACEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INTERFACEINFO").field("pUnk", &self.pUnk).field("iid", &self.iid).field("wMethod", &self.wMethod).finish()
    }
}
unsafe impl ::windows::core::Abi for INTERFACEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for INTERFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pUnk == other.pUnk && self.iid == other.iid && self.wMethod == other.wMethod
    }
}
impl ::core::cmp::Eq for INTERFACEINFO {}
impl ::core::default::Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INVOKEKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
impl ::core::marker::Copy for INVOKEKIND {}
impl ::core::clone::Clone for INVOKEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INVOKEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INVOKEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for INVOKEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INVOKEKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct INoMarshal(::windows::core::IUnknown);
impl INoMarshal {}
impl ::core::convert::From<INoMarshal> for ::windows::core::IUnknown {
    fn from(value: INoMarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INoMarshal> for ::windows::core::IUnknown {
    fn from(value: &INoMarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INoMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INoMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INoMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INoMarshal {}
impl ::core::fmt::Debug for INoMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INoMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INoMarshal {
    type Vtable = INoMarshal_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc8691b_c1db_4dc0_855e_65f6c551af49);
}
#[repr(C)]
#[doc(hidden)]
pub struct INoMarshal_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IOplockStorage(::windows::core::IUnknown);
impl IOplockStorage {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateStorageEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateStorageEx)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn OpenStorageEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, T: ::windows::core::Interface>(&self, pwcsname: Param0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).OpenStorageEx)(::core::mem::transmute_copy(self), pwcsname.into_param().abi(), ::core::mem::transmute(grfmode), ::core::mem::transmute(stgfmt), ::core::mem::transmute(grfattrs), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: IOplockStorage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IOplockStorage> for ::windows::core::IUnknown {
    fn from(value: &IOplockStorage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IOplockStorage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IOplockStorage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IOplockStorage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOplockStorage {}
impl ::core::fmt::Debug for IOplockStorage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOplockStorage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IOplockStorage {
    type Vtable = IOplockStorage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPSFactoryBuffer(::windows::core::IUnknown);
impl IPSFactoryBuffer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateProxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkouter: Param0, riid: *const ::windows::core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateProxy)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppproxy), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateStub<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, riid: *const ::windows::core::GUID, punkserver: Param1) -> ::windows::core::Result<IRpcStubBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateStub)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), punkserver.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IRpcStubBuffer>(result__)
    }
}
impl ::core::convert::From<IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: IPSFactoryBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPSFactoryBuffer> for ::windows::core::IUnknown {
    fn from(value: &IPSFactoryBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPSFactoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPSFactoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPSFactoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPSFactoryBuffer {}
impl ::core::fmt::Debug for IPSFactoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSFactoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPSFactoryBuffer {
    type Vtable = IPSFactoryBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f569d0_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersist(::windows::core::IUnknown);
impl IPersist {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IPersist> for ::windows::core::IUnknown {
    fn from(value: IPersist) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersist> for ::windows::core::IUnknown {
    fn from(value: &IPersist) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersist {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersist {}
impl ::core::fmt::Debug for IPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersist").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersist {
    type Vtable = IPersist_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistFile(::windows::core::IUnknown);
impl IPersistFile {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszfilename: Param0, dwmode: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), ::core::mem::transmute(dwmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pszfilename: Param0, fremember: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), pszfilename.into_param().abi(), fremember.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SaveCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszfilename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveCompleted)(::core::mem::transmute_copy(self), pszfilename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCurFile(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurFile)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IPersistFile> for ::windows::core::IUnknown {
    fn from(value: IPersistFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistFile> for ::windows::core::IUnknown {
    fn from(value: &IPersistFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &'a IPersistFile {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistFile {}
impl ::core::fmt::Debug for IPersistFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistFile {
    type Vtable = IPersistFile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, dwmode: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SaveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszfilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistMemory(::windows::core::IUnknown);
impl IPersistMemory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Load(&self, pmem: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pmem)), pmem.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmem: &mut [u8], fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pmem)), fcleardirty.into_param().abi(), pmem.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitNew)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: IPersistMemory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistMemory> for ::windows::core::IUnknown {
    fn from(value: &IPersistMemory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &'a IPersistMemory {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistMemory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistMemory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMemory {}
impl ::core::fmt::Debug for IPersistMemory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMemory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistMemory {
    type Vtable = IPersistMemory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistMemory_Vtbl {
    pub base: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistStream(::windows::core::IUnknown);
impl IPersistStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IPersistStream> for ::windows::core::IUnknown {
    fn from(value: IPersistStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStream> for ::windows::core::IUnknown {
    fn from(value: &IPersistStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &'a IPersistStream {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStream {}
impl ::core::fmt::Debug for IPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStream {
    type Vtable = IPersistStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000109_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_Vtbl {
    pub base: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPersistStreamInit(::windows::core::IUnknown);
impl IPersistStreamInit {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsDirty(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDirty)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Load<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows::core::IntoParam<'a, IStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pstm: Param0, fcleardirty: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::core::mem::transmute_copy(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn InitNew(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitNew)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: IPersistStreamInit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPersistStreamInit> for ::windows::core::IUnknown {
    fn from(value: &IPersistStreamInit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPersist> for &'a IPersistStreamInit {
    fn into_param(self) -> ::windows::core::Param<'a, IPersist> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPersistStreamInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPersistStreamInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistStreamInit {}
impl ::core::fmt::Debug for IPersistStreamInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistStreamInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPersistStreamInit {
    type Vtable = IPersistStreamInit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fd52380_4e07_101b_ae2d_08002b2ec713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStreamInit_Vtbl {
    pub base: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT,
    pub InitNew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeByte(::windows::core::IUnknown);
impl IPipeByte {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Pull(&self, buf: &mut [u8], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Push(&self, buf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeByte> for ::windows::core::IUnknown {
    fn from(value: IPipeByte) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeByte> for ::windows::core::IUnknown {
    fn from(value: &IPipeByte) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeByte {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeByte {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeByte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeByte {}
impl ::core::fmt::Debug for IPipeByte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeByte").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeByte {
    type Vtable = IPipeByte_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeByte_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeDouble(::windows::core::IUnknown);
impl IPipeDouble {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Pull(&self, buf: &mut [f64], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Push(&self, buf: &[f64]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: IPipeDouble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeDouble> for ::windows::core::IUnknown {
    fn from(value: &IPipeDouble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeDouble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeDouble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeDouble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeDouble {}
impl ::core::fmt::Debug for IPipeDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeDouble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeDouble {
    type Vtable = IPipeDouble_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeDouble_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IPipeLong(::windows::core::IUnknown);
impl IPipeLong {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Pull(&self, buf: &mut [i32], pcreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pull)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(buf)), buf.len() as _, ::core::mem::transmute(pcreturned)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Push(&self, buf: &[i32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Push)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(buf)), buf.len() as _).ok()
    }
}
impl ::core::convert::From<IPipeLong> for ::windows::core::IUnknown {
    fn from(value: IPipeLong) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPipeLong> for ::windows::core::IUnknown {
    fn from(value: &IPipeLong) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPipeLong {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPipeLong {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPipeLong {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPipeLong {}
impl ::core::fmt::Debug for IPipeLong {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPipeLong").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPipeLong {
    type Vtable = IPipeLong_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPipeLong_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Pull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProcessInitControl(::windows::core::IUnknown);
impl IProcessInitControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetInitializerTimeout)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsecondsremaining)).ok()
    }
}
impl ::core::convert::From<IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: IProcessInitControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessInitControl> for ::windows::core::IUnknown {
    fn from(value: &IProcessInitControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProcessInitControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessInitControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessInitControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessInitControl {}
impl ::core::fmt::Debug for IProcessInitControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessInitControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessInitControl {
    type Vtable = IProcessInitControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessInitControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProcessLock(::windows::core::IUnknown);
impl IProcessLock {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).AddRefOnProcess)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).ReleaseRefOnProcess)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IProcessLock> for ::windows::core::IUnknown {
    fn from(value: IProcessLock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProcessLock> for ::windows::core::IUnknown {
    fn from(value: &IProcessLock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProcessLock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProcessLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProcessLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProcessLock {}
impl ::core::fmt::Debug for IProcessLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProcessLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProcessLock {
    type Vtable = IProcessLock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d5_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IProgressNotify(::windows::core::IUnknown);
impl IProgressNotify {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnProgress<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: Param2, fowner: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwprogresscurrent), ::core::mem::transmute(dwprogressmaximum), faccurate.into_param().abi(), fowner.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: IProgressNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProgressNotify> for ::windows::core::IUnknown {
    fn from(value: &IProgressNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProgressNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IProgressNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProgressNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProgressNotify {}
impl ::core::fmt::Debug for IProgressNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProgressNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProgressNotify {
    type Vtable = IProgressNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d758a0_4617_11cf_95fc_00aa00680db4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnProgress: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IROTData(::windows::core::IUnknown);
impl IROTData {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetComparisonData(&self, pbdata: &mut [u8], pcbdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComparisonData)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pbdata)), pbdata.len() as _, ::core::mem::transmute(pcbdata)).ok()
    }
}
impl ::core::convert::From<IROTData> for ::windows::core::IUnknown {
    fn from(value: IROTData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IROTData> for ::windows::core::IUnknown {
    fn from(value: &IROTData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IROTData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IROTData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IROTData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IROTData {}
impl ::core::fmt::Debug for IROTData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IROTData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IROTData {
    type Vtable = IROTData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf29f6bc0_5021_11ce_aa15_00006901293f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetComparisonData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IReleaseMarshalBuffers(::windows::core::IUnknown);
impl IReleaseMarshalBuffers {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseMarshalBuffer<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMarshalBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(dwflags), pchnl.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: IReleaseMarshalBuffers) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReleaseMarshalBuffers> for ::windows::core::IUnknown {
    fn from(value: &IReleaseMarshalBuffers) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReleaseMarshalBuffers {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReleaseMarshalBuffers {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReleaseMarshalBuffers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReleaseMarshalBuffers {}
impl ::core::fmt::Debug for IReleaseMarshalBuffers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReleaseMarshalBuffers").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReleaseMarshalBuffers {
    type Vtable = IReleaseMarshalBuffers_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb0cb9e8_7996_11d2_872e_0000f8080859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer(::windows::core::IUnknown);
impl IRpcChannelBuffer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendReceive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDestCtx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer {}
impl ::core::fmt::Debug for IRpcChannelBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer {
    type Vtable = IRpcChannelBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56b60_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer2(::windows::core::IUnknown);
impl IRpcChannelBuffer2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SendReceive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDestCtx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsConnected)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProtocolVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer2> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &'a IRpcChannelBuffer2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer2 {}
impl ::core::fmt::Debug for IRpcChannelBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer2 {
    type Vtable = IRpcChannelBuffer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcChannelBuffer3(::windows::core::IUnknown);
impl IRpcChannelBuffer3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(riid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SendReceive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.FreeBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmessage)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDestCtx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.IsConnected)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProtocolVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Send)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Receive)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(ulsize), ::core::mem::transmute(pulstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCallContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(riid), ::core::mem::transmute(pinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDestCtxEx)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(pdwdestcontext), ::core::mem::transmute(ppvdestcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RegisterAsync<'a, Param1: ::windows::core::IntoParam<'a, IAsyncManager>>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterAsync)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg), pasyncmgr.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: IRpcChannelBuffer3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcChannelBuffer3> for ::windows::core::IUnknown {
    fn from(value: &IRpcChannelBuffer3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IRpcChannelBuffer2> for &'a IRpcChannelBuffer3 {
    fn into_param(self) -> ::windows::core::Param<'a, IRpcChannelBuffer2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcChannelBuffer3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcChannelBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcChannelBuffer3 {}
impl ::core::fmt::Debug for IRpcChannelBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcChannelBuffer3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcChannelBuffer3 {
    type Vtable = IRpcChannelBuffer3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcChannelBuffer3_Vtbl {
    pub base: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Receive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT,
    pub RegisterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcHelper(::windows::core::IUnknown);
impl IRpcHelper {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetDCOMProtocolVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDCOMProtocolVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut ::windows::core::GUID> {
        let mut result__: *mut ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIIDFromOBJREF)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobjref), ::core::mem::transmute(&mut result__)).from_abi::<*mut ::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: IRpcHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcHelper> for ::windows::core::IUnknown {
    fn from(value: &IRpcHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcHelper {}
impl ::core::fmt::Debug for IRpcHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcHelper {
    type Vtable = IRpcHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000149_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcHelper_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcOptions(::windows::core::IUnknown);
impl IRpcOptions {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Set<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Set)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(dwvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pprx: Param0, dwproperty: RPCOPT_PROPERTIES) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Query)(::core::mem::transmute_copy(self), pprx.into_param().abi(), ::core::mem::transmute(dwproperty), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: IRpcOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcOptions> for ::windows::core::IUnknown {
    fn from(value: &IRpcOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcOptions {}
impl ::core::fmt::Debug for IRpcOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcOptions {
    type Vtable = IRpcOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000144_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcOptions_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Set: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcProxyBuffer(::windows::core::IUnknown);
impl IRpcProxyBuffer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, prpcchannelbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self))
    }
}
impl ::core::convert::From<IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcProxyBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcProxyBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcProxyBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcProxyBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcProxyBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcProxyBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcProxyBuffer {}
impl ::core::fmt::Debug for IRpcProxyBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcProxyBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcProxyBuffer {
    type Vtable = IRpcProxyBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56a34_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcStubBuffer(::windows::core::IUnknown);
impl IRpcStubBuffer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, punkserver: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Connect)(::core::mem::transmute_copy(self), punkserver.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Disconnect(&self) {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::core::IntoParam<'a, IRpcChannelBuffer>>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::core::mem::transmute_copy(self), ::core::mem::transmute(_prpcmsg), _prpcchannelbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsIIDSupported(&self, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsIIDSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CountRefs(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).CountRefs)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DebugServerQueryInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void) {
        (::windows::core::Interface::vtable(self).DebugServerRelease)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv))
    }
}
impl ::core::convert::From<IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: IRpcStubBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcStubBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRpcStubBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcStubBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcStubBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcStubBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcStubBuffer {}
impl ::core::fmt::Debug for IRpcStubBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcStubBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcStubBuffer {
    type Vtable = IRpcStubBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5f56afc_593b_101a_b569_08002b2dbf7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcStubBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsIIDSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer>,
    pub CountRefs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub DebugServerQueryInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRpcSyntaxNegotiate(::windows::core::IUnknown);
impl IRpcSyntaxNegotiate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NegotiateSyntax)(::core::mem::transmute_copy(self), ::core::mem::transmute(pmsg)).ok()
    }
}
impl ::core::convert::From<IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: IRpcSyntaxNegotiate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRpcSyntaxNegotiate> for ::windows::core::IUnknown {
    fn from(value: &IRpcSyntaxNegotiate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRpcSyntaxNegotiate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRpcSyntaxNegotiate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRpcSyntaxNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRpcSyntaxNegotiate {}
impl ::core::fmt::Debug for IRpcSyntaxNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRpcSyntaxNegotiate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRpcSyntaxNegotiate {
    type Vtable = IRpcSyntaxNegotiate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a08519_24c8_4935_b482_3fd823333a4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRunnableObject(::windows::core::IUnknown);
impl IRunnableObject {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRunningClass(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRunningClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Run<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>>(&self, pbc: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Run)(::core::mem::transmute_copy(self), pbc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsRunning)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LockRunning<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, flock: Param0, flastunlockcloses: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRunning)(::core::mem::transmute_copy(self), flock.into_param().abi(), flastunlockcloses.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContainedObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcontained: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContainedObject)(::core::mem::transmute_copy(self), fcontained.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: IRunnableObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunnableObject> for ::windows::core::IUnknown {
    fn from(value: &IRunnableObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRunnableObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunnableObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunnableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunnableObject {}
impl ::core::fmt::Debug for IRunnableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunnableObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRunnableObject {
    type Vtable = IRunnableObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000126_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetRunningClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsRunning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LockRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LockRunning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContainedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContainedObject: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IRunningObjectTable(::windows::core::IUnknown);
impl IRunningObjectTable {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Register<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, IMoniker>>(&self, grfflags: u32, punkobject: Param1, pmkobjectname: Param2) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Register)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfflags), punkobject.into_param().abi(), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Revoke(&self, dwregister: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revoke)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn IsRunning<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsRunning)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetObject<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetObject)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NoteChangeTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwregister), ::core::mem::transmute(pfiletime)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTimeOfLastChange<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>>(&self, pmkobjectname: Param0) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: super::super::Foundation::FILETIME = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTimeOfLastChange)(::core::mem::transmute_copy(self), pmkobjectname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumRunning(&self) -> ::windows::core::Result<IEnumMoniker> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumRunning)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumMoniker>(result__)
    }
}
impl ::core::convert::From<IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: IRunningObjectTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRunningObjectTable> for ::windows::core::IUnknown {
    fn from(value: &IRunningObjectTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRunningObjectTable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRunningObjectTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRunningObjectTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRunningObjectTable {}
impl ::core::fmt::Debug for IRunningObjectTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningObjectTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRunningObjectTable {
    type Vtable = IRunningObjectTable_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000010_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfflags: u32, punkobject: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pdwregister: *mut u32) -> ::windows::core::HRESULT,
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub NoteChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NoteChangeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTimeOfLastChange: usize,
    pub EnumRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISequentialStream(::windows::core::IUnknown);
impl ISequentialStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: ISequentialStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISequentialStream> for ::windows::core::IUnknown {
    fn from(value: &ISequentialStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISequentialStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISequentialStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISequentialStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISequentialStream {}
impl ::core::fmt::Debug for ISequentialStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISequentialStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISequentialStream {
    type Vtable = ISequentialStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IServerSecurity(::windows::core::IUnknown);
impl IServerSecurity {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryBlanket)(::core::mem::transmute_copy(self), ::core::mem::transmute(pauthnsvc), ::core::mem::transmute(pauthzsvc), ::core::mem::transmute(pserverprincname), ::core::mem::transmute(pauthnlevel), ::core::mem::transmute(pimplevel), ::core::mem::transmute(pprivs), ::core::mem::transmute(pcapabilities)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ImpersonateClient(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImpersonateClient)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RevertToSelf(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevertToSelf)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsImpersonating)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: IServerSecurity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServerSecurity> for ::windows::core::IUnknown {
    fn from(value: &IServerSecurity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IServerSecurity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServerSecurity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServerSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServerSecurity {}
impl ::core::fmt::Debug for IServerSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServerSecurity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServerSecurity {
    type Vtable = IServerSecurity_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000013e_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServerSecurity_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueryBlanket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT,
    pub ImpersonateClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsImpersonating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsImpersonating: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IServiceProvider(::windows::core::IUnknown);
impl IServiceProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryService)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidservice), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
}
impl ::core::convert::From<IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: IServiceProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IServiceProvider> for ::windows::core::IUnknown {
    fn from(value: &IServiceProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IServiceProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IServiceProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IServiceProvider {}
impl ::core::fmt::Debug for IServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IServiceProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IServiceProvider {
    type Vtable = IServiceProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5140c1_7436_11ce_8034_00aa006009fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub QueryService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IStdMarshalInfo(::windows::core::IUnknown);
impl IStdMarshalInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClassForHandler)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(pclsid)).ok()
    }
}
impl ::core::convert::From<IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: IStdMarshalInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStdMarshalInfo> for ::windows::core::IUnknown {
    fn from(value: &IStdMarshalInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStdMarshalInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStdMarshalInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStdMarshalInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStdMarshalInfo {}
impl ::core::fmt::Debug for IStdMarshalInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStdMarshalInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStdMarshalInfo {
    type Vtable = IStdMarshalInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000018_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStdMarshalInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetClassForHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IStream(::windows::core::IUnknown);
impl IStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyTo)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Commit(&self, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Revert)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LockRegion)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnlockRegion)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stat)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IStream>(result__)
    }
}
impl ::core::convert::From<IStream> for ::windows::core::IUnknown {
    fn from(value: IStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStream> for ::windows::core::IUnknown {
    fn from(value: &IStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISequentialStream> for &'a IStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStream {}
impl ::core::fmt::Debug for IStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStream {
    type Vtable = IStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base: ISequentialStream_Vtbl,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Commit: usize,
    pub Revert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Stat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Stat: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISupportErrorInfo(::windows::core::IUnknown);
impl ISupportErrorInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InterfaceSupportsErrorInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ISupportErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ISupportErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISupportErrorInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportErrorInfo {}
impl ::core::fmt::Debug for ISupportErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISupportErrorInfo {
    type Vtable = ISupportErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0b3d60_548f_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportErrorInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISurrogate(::windows::core::IUnknown);
impl ISurrogate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn LoadDllServer(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadDllServer)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FreeSurrogate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeSurrogate)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISurrogate> for ::windows::core::IUnknown {
    fn from(value: ISurrogate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogate> for ::windows::core::IUnknown {
    fn from(value: &ISurrogate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurrogate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogate {}
impl ::core::fmt::Debug for ISurrogate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurrogate {
    type Vtable = ISurrogate_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000022_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogate_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub LoadDllServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISurrogateService(::windows::core::IUnknown);
impl ISurrogateService {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<'a, Param1: ::windows::core::IntoParam<'a, IProcessLock>>(&self, rguidprocessid: *const ::windows::core::GUID, pprocesslock: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Init)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidprocessid), pprocesslock.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplicationLaunch)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid), ::core::mem::transmute(apptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplicationFree)(::core::mem::transmute_copy(self), ::core::mem::transmute(rguidapplid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CatalogRefresh)(::core::mem::transmute_copy(self), ::core::mem::transmute(ulreserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessShutdown)(::core::mem::transmute_copy(self), ::core::mem::transmute(shutdowntype)).ok()
    }
}
impl ::core::convert::From<ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: ISurrogateService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurrogateService> for ::windows::core::IUnknown {
    fn from(value: &ISurrogateService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISurrogateService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurrogateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurrogateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurrogateService {}
impl ::core::fmt::Debug for ISurrogateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurrogateService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISurrogateService {
    type Vtable = ISurrogateService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001d4_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: ::windows::core::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Init: usize,
    pub ApplicationLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronize(::windows::core::IUnknown);
impl ISynchronize {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Signal)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISynchronize> for ::windows::core::IUnknown {
    fn from(value: ISynchronize) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronize> for ::windows::core::IUnknown {
    fn from(value: &ISynchronize) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronize {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronize {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronize {}
impl ::core::fmt::Debug for ISynchronize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronize {
    type Vtable = ISynchronize_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000030_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronize_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeContainer(::windows::core::IUnknown);
impl ISynchronizeContainer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSynchronize)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitMultiple)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwtimeout), ::core::mem::transmute(&mut result__)).from_abi::<ISynchronize>(result__)
    }
}
impl ::core::convert::From<ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeContainer> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeContainer {}
impl ::core::fmt::Debug for ISynchronizeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeContainer {
    type Vtable = ISynchronizeContainer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000033_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeContainer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeEvent(::windows::core::IUnknown);
impl ISynchronizeEvent {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(ph)).ok()
    }
}
impl ::core::convert::From<ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeEvent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeEvent> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeEvent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISynchronizeHandle> for &'a ISynchronizeEvent {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronizeHandle> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeEvent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeEvent {}
impl ::core::fmt::Debug for ISynchronizeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeEvent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeEvent {
    type Vtable = ISynchronizeEvent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000032_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeEvent_Vtbl {
    pub base: ISynchronizeHandle_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeHandle(::windows::core::IUnknown);
impl ISynchronizeHandle {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
impl ::core::convert::From<ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeHandle) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeHandle> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeHandle) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeHandle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeHandle {}
impl ::core::fmt::Debug for ISynchronizeHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeHandle {
    type Vtable = ISynchronizeHandle_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000031_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeHandle_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHandle: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ISynchronizeMutex(::windows::core::IUnknown);
impl ISynchronizeMutex {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Wait)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwmilliseconds)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Signal(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Signal)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseMutex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMutex)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: ISynchronizeMutex) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizeMutex> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizeMutex) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISynchronize> for &'a ISynchronizeMutex {
    fn into_param(self) -> ::windows::core::Param<'a, ISynchronize> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizeMutex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizeMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizeMutex {}
impl ::core::fmt::Debug for ISynchronizeMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizeMutex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronizeMutex {
    type Vtable = ISynchronizeMutex_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000025_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizeMutex_Vtbl {
    pub base: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITimeAndNoticeControl(::windows::core::IUnknown);
impl ITimeAndNoticeControl {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuppressChanges)(::core::mem::transmute_copy(self), ::core::mem::transmute(res1), ::core::mem::transmute(res2)).ok()
    }
}
impl ::core::convert::From<ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: ITimeAndNoticeControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITimeAndNoticeControl> for ::windows::core::IUnknown {
    fn from(value: &ITimeAndNoticeControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITimeAndNoticeControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITimeAndNoticeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITimeAndNoticeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimeAndNoticeControl {}
impl ::core::fmt::Debug for ITimeAndNoticeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeAndNoticeControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITimeAndNoticeControl {
    type Vtable = ITimeAndNoticeControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SuppressChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeComp(::windows::core::IUnknown);
impl ITypeComp {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Bind<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, szname: Param0, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Bind)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(wflags), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pdesckind), ::core::mem::transmute(pbindptr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn BindType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, szname: Param0, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BindType)(::core::mem::transmute_copy(self), szname.into_param().abi(), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(pptcomp)).ok()
    }
}
impl ::core::convert::From<ITypeComp> for ::windows::core::IUnknown {
    fn from(value: ITypeComp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeComp> for ::windows::core::IUnknown {
    fn from(value: &ITypeComp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeComp {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeComp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeComp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeComp {}
impl ::core::fmt::Debug for ITypeComp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeComp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeComp {
    type Vtable = ITypeComp_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020403_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeComp_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows::core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeInfo(::windows::core::IUnknown);
impl ITypeInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: *mut TYPEATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TYPEATTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeComp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: *mut FUNCDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFuncDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut FUNCDESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: *mut VARDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVarDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut VARDESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRefTypeOfImplType)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImplTypeFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetIDsOfNames<'a, const PARAM1: usize>(&self, rgsznames: &[::windows::core::PWSTR; PARAM1], pmemid: &mut [i32; PARAM1]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIDsOfNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsznames)), PARAM1 as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pmemid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Invoke)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentation)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDllEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRefTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddressOfMember)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateInstance)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMops)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContainingTypeLib)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Interface::vtable(self).ReleaseTypeAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Interface::vtable(self).ReleaseFuncDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Interface::vtable(self).ReleaseVarDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc))
    }
}
impl ::core::convert::From<ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo {}
impl ::core::fmt::Debug for ITypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo {
    type Vtable = ITypeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020401_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetFuncDesc: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetVarDesc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNames: usize,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rgsznames: *const ::windows::core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Invoke: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDllEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDllEntry: usize,
    pub GetRefTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMops: usize,
    pub GetContainingTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub ReleaseTypeAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR),
    #[cfg(not(feature = "Win32_System_Ole"))]
    ReleaseTypeAttr: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub ReleaseFuncDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    ReleaseFuncDesc: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub ReleaseVarDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    ReleaseVarDesc: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeInfo2(::windows::core::IUnknown);
impl ITypeInfo2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR> {
        let mut result__: *mut TYPEATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TYPEATTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeComp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC> {
        let mut result__: *mut FUNCDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFuncDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut FUNCDESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC> {
        let mut result__: *mut VARDESC = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetVarDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut VARDESC>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(rgbstrnames), ::core::mem::transmute(cmaxnames), ::core::mem::transmute(pcnames)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRefTypeOfImplType)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetImplTypeFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetIDsOfNames<'a, const PARAM1: usize>(&self, rgsznames: &[::windows::core::PWSTR; PARAM1], pmemid: &mut [i32; PARAM1]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetIDsOfNames)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsznames)), PARAM1 as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pmemid))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Invoke)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvinstance), ::core::mem::transmute(memid), ::core::mem::transmute(wflags), ::core::mem::transmute(pdispparams), ::core::mem::transmute(pvarresult), ::core::mem::transmute(pexcepinfo), ::core::mem::transmute(puargerr)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDocumentation)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDllEntry)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(pbstrdllname), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pwordinal)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetRefTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(hreftype), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddressOfMember)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, punkouter: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.CreateInstance)(::core::mem::transmute_copy(self), punkouter.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMops)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetContainingTypeLib)(::core::mem::transmute_copy(self), ::core::mem::transmute(pptlib), ::core::mem::transmute(pindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (::windows::core::Interface::vtable(self).base.ReleaseTypeAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptypeattr))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (::windows::core::Interface::vtable(self).base.ReleaseFuncDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfuncdesc))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (::windows::core::Interface::vtable(self).base.ReleaseVarDesc)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvardesc))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeKind(&self) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeKind)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFuncIndexOfMemId)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(invkind), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVarIndexOfMemId)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFuncCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetParamCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVarCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetImplTypeCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentation2)(::core::mem::transmute_copy(self), ::core::mem::transmute(memid), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllFuncCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllParamCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(indexfunc), ::core::mem::transmute(indexparam), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllVarCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllImplTypeCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: ITypeInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeInfo2> for ::windows::core::IUnknown {
    fn from(value: &ITypeInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITypeInfo> for &'a ITypeInfo2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeInfo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeInfo2 {}
impl ::core::fmt::Debug for ITypeInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeInfo2 {
    type Vtable = ITypeInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020412_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeInfo2_Vtbl {
    pub base: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetImplTypeCustData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllFuncCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllFuncCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllParamCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllParamCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllVarCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllVarCustData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllImplTypeCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllImplTypeCustData: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLib(::windows::core::IUnknown);
impl ITypeLib {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetTypeInfoCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeInfoType)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeInfoOfGuid)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: *mut TLIBATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLibAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TLIBATTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTypeComp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentation)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsName)(::core::mem::transmute_copy(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FindName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindName)(::core::mem::transmute_copy(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Interface::vtable(self).ReleaseTLibAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr))
    }
}
impl ::core::convert::From<ITypeLib> for ::windows::core::IUnknown {
    fn from(value: ITypeLib) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLib {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib {}
impl ::core::fmt::Debug for ITypeLib {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLib {
    type Vtable = ITypeLib_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020402_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT,
    pub GetTypeComp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsName: usize,
    pub FindName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR),
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLib2(::windows::core::IUnknown);
impl ITypeLib2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetTypeInfoCount)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND> {
        let mut result__: TYPEKIND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeInfoType)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<TYPEKIND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeInfoOfGuid)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<ITypeInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR> {
        let mut result__: *mut TLIBATTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLibAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut TLIBATTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetTypeComp)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ITypeComp>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDocumentation)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrdocstring), ::core::mem::transmute(pdwhelpcontext), ::core::mem::transmute(pbstrhelpfile)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsName)(::core::mem::transmute_copy(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pfname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn FindName(&self, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.FindName)(::core::mem::transmute_copy(self), ::core::mem::transmute(sznamebuf), ::core::mem::transmute(lhashval), ::core::mem::transmute(pptinfo), ::core::mem::transmute(rgmemid), ::core::mem::transmute(pcfound)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (::windows::core::Interface::vtable(self).base.ReleaseTLibAttr)(::core::mem::transmute_copy(self), ::core::mem::transmute(ptlibattr))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(guid), ::core::mem::transmute(&mut result__)).from_abi::<VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLibStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcuniquenames), ::core::mem::transmute(pcchuniquenames)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDocumentation2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(lcid), ::core::mem::transmute(pbstrhelpstring), ::core::mem::transmute(pdwhelpstringcontext), ::core::mem::transmute(pbstrhelpstringdll)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA> {
        let mut result__: CUSTDATA = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAllCustData)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<CUSTDATA>(result__)
    }
}
impl ::core::convert::From<ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: ITypeLib2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLib2> for ::windows::core::IUnknown {
    fn from(value: &ITypeLib2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ITypeLib> for &'a ITypeLib2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITypeLib> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLib2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLib2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLib2 {}
impl ::core::fmt::Debug for ITypeLib2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLib2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLib2 {
    type Vtable = ITypeLib2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00020411_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLib2_Vtbl {
    pub base: ITypeLib_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetCustData: usize,
    pub GetLibStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDocumentation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDocumentation2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetAllCustData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetAllCustData: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLibRegistration(::windows::core::IUnknown);
impl ITypeLibRegistration {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGuid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetVersion)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetLcid(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLcid)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin32Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWin32Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWin64Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetWin64Path)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHelpDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHelpDir)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistration> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLibRegistration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistration {}
impl ::core::fmt::Debug for ITypeLibRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistration {
    type Vtable = ITypeLibRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76a3e735_02df_4a12_98eb_043ad3600af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistration_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVersion: usize,
    pub GetLcid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWin32Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin32path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWin32Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWin64Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwin64path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWin64Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayName: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHelpDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phelpdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHelpDir: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct ITypeLibRegistrationReader(::windows::core::IUnknown);
impl ITypeLibRegistrationReader {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn EnumTypeLibRegistrations(&self) -> ::windows::core::Result<IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumTypeLibRegistrations)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumUnknown>(result__)
    }
}
impl ::core::convert::From<ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: ITypeLibRegistrationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITypeLibRegistrationReader> for ::windows::core::IUnknown {
    fn from(value: &ITypeLibRegistrationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITypeLibRegistrationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITypeLibRegistrationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITypeLibRegistrationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITypeLibRegistrationReader {}
impl ::core::fmt::Debug for ITypeLibRegistrationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITypeLibRegistrationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITypeLibRegistrationReader {
    type Vtable = ITypeLibRegistrationReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUri(::windows::core::IUnknown);
impl IUri {
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyBSTR)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pbstrproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pcchproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyDWORD)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(pdwproperty), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(uriprop), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAbsoluteUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAuthority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAuthority)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDomain)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetExtension)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFragment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFragment)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHost)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPassword(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPassword)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPathAndQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRawUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetRawUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSchemeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetHostType(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHostType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPort)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetScheme(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetScheme)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetZone(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetZone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, puri: Param0) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsEqual)(::core::mem::transmute_copy(self), puri.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUri> for ::windows::core::IUnknown {
    fn from(value: IUri) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUri> for ::windows::core::IUnknown {
    fn from(value: &IUri) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUri {}
impl ::core::fmt::Debug for IUri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUri").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUri {
    type Vtable = IUri_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa39ee748_6a27_4817_a6f2_13914bef5890);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUri_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyBSTR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyBSTR: usize,
    pub GetPropertyLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAbsoluteUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAbsoluteUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAuthority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAuthority: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDomain: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrextension: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExtension: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfragment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFragment: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrhost: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHost: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPathAndQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPathAndQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRawUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRawUri: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrschemename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemeName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserName: usize,
    pub GetHostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT,
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT,
    pub GetZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqual: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUriBuilder(::windows::core::IUnknown);
impl IUriBuilder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateUriSimple)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateUriWithFlags)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcreateflags), ::core::mem::transmute(dwuribuilderflags), ::core::mem::transmute(dwallowencodingpropertymask), ::core::mem::transmute(dwreserved), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetIUri(&self) -> ::windows::core::Result<IUri> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIUri)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IUri>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetIUri<'a, Param0: ::windows::core::IntoParam<'a, IUri>>(&self, piuri: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIUri)(::core::mem::transmute_copy(self), piuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFragment)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchfragment), ::core::mem::transmute(ppwzfragment)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHost)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchhost), ::core::mem::transmute(ppwzhost)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPassword)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpassword), ::core::mem::transmute(ppwzpassword)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchpath), ::core::mem::transmute(ppwzpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPort)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfhasport), ::core::mem::transmute(pdwport)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchquery), ::core::mem::transmute(ppwzquery)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSchemeName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchschemename), ::core::mem::transmute(ppwzschemename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUserName)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcchusername), ::core::mem::transmute(ppwzusername)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetFragment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFragment)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetHost<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHost)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetPassword<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPassword)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPath)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fhasport: Param0, dwnewvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPort)(::core::mem::transmute_copy(self), fhasport.into_param().abi(), ::core::mem::transmute(dwnewvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetQuery<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetQuery)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetSchemeName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSchemeName)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwznewvalue: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserName)(::core::mem::transmute_copy(self), pwznewvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwpropertymask)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasBeenModified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).HasBeenModified)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: IUriBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUriBuilder> for ::windows::core::IUnknown {
    fn from(value: &IUriBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUriBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUriBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUriBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilder {}
impl ::core::fmt::Debug for IUriBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUriBuilder {
    type Vtable = IUriBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4221b2e1_8955_46c0_bd5b_de9897565de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriBuilder_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateUriSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateUriWithFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetIUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPort: usize,
    pub GetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPort: usize,
    pub SetQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasBeenModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasBeenModified: usize,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IUrlMon(::windows::core::IUnknown);
impl IUrlMon {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AsyncGetClassBits<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, IBindCtx>>(&self, rclsid: *const ::windows::core::GUID, psztype: Param1, pszext: Param2, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: Param5, pbc: Param6, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AsyncGetClassBits)(::core::mem::transmute_copy(self), ::core::mem::transmute(rclsid), psztype.into_param().abi(), pszext.into_param().abi(), ::core::mem::transmute(dwfileversionms), ::core::mem::transmute(dwfileversionls), pszcodebase.into_param().abi(), pbc.into_param().abi(), ::core::mem::transmute(dwclasscontext), ::core::mem::transmute(riid), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IUrlMon> for ::windows::core::IUnknown {
    fn from(value: IUrlMon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUrlMon> for ::windows::core::IUnknown {
    fn from(value: &IUrlMon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUrlMon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUrlMon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUrlMon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlMon {}
impl ::core::fmt::Debug for IUrlMon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlMon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUrlMon {
    type Vtable = IUrlMon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000026_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: ::windows::core::PCWSTR, pszext: ::windows::core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: ::windows::core::PCWSTR, pbc: ::windows::core::RawPtr, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
pub struct IWaitMultiple(::windows::core::IUnknown);
impl IWaitMultiple {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> ::windows::core::Result<ISynchronize> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).WaitMultiple)(::core::mem::transmute_copy(self), ::core::mem::transmute(timeout), ::core::mem::transmute(&mut result__)).from_abi::<ISynchronize>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    pub unsafe fn AddSynchronize<'a, Param0: ::windows::core::IntoParam<'a, ISynchronize>>(&self, psync: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSynchronize)(::core::mem::transmute_copy(self), psync.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: IWaitMultiple) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWaitMultiple> for ::windows::core::IUnknown {
    fn from(value: &IWaitMultiple) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWaitMultiple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWaitMultiple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWaitMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWaitMultiple {}
impl ::core::fmt::Debug for IWaitMultiple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWaitMultiple").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWaitMultiple {
    type Vtable = IWaitMultiple_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0000002b_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWaitMultiple_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub WaitMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct LONG_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl ::core::marker::Copy for LONG_SIZEDARR {}
impl ::core::clone::Clone for LONG_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LONG_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LONG_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for LONG_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LONG_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LONG_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for LONG_SIZEDARR {}
impl ::core::default::Default for LONG_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEXCEPFINO_DEFERRED_FILLIN = ::core::option::Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNCANUNLOADNOW = ::core::option::Option<unsafe extern "system" fn() -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type LPFNGETCLASSOBJECT = ::core::option::Option<unsafe extern "system" fn(param0: *const ::windows::core::GUID, param1: *const ::windows::core::GUID, param2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MAXLSN: u64 = 9223372036854775807u64;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEMCTX(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
impl ::core::marker::Copy for MEMCTX {}
impl ::core::clone::Clone for MEMCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEMCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEMCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMCTX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MKREDUCE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ONE: MKREDUCE = MKREDUCE(196608i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_TOUSER: MKREDUCE = MKREDUCE(131072i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_THROUGHUSER: MKREDUCE = MKREDUCE(65536i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKRREDUCE_ALL: MKREDUCE = MKREDUCE(0i32);
impl ::core::marker::Copy for MKREDUCE {}
impl ::core::clone::Clone for MKREDUCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKREDUCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MKREDUCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKREDUCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKREDUCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MKSYS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
impl ::core::marker::Copy for MKSYS {}
impl ::core::clone::Clone for MKSYS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MKSYS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MKSYS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MKSYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MKSYS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSHCTX(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
impl ::core::marker::Copy for MSHCTX {}
impl ::core::clone::Clone for MSHCTX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHCTX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSHCTX {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHCTX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHCTX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MSHLFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
impl ::core::marker::Copy for MSHLFLAGS {}
impl ::core::clone::Clone for MSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MSHLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSHLFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MULTI_QI {
    pub pIID: *const ::windows::core::GUID,
    pub pItf: ::core::option::Option<::windows::core::IUnknown>,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::clone::Clone for MULTI_QI {
    fn clone(&self) -> Self {
        Self { pIID: self.pIID, pItf: self.pItf.clone(), hr: self.hr }
    }
}
impl ::core::fmt::Debug for MULTI_QI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MULTI_QI").field("pIID", &self.pIID).field("pItf", &self.pItf).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows::core::Abi for MULTI_QI {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for MULTI_QI {
    fn eq(&self, other: &Self) -> bool {
        self.pIID == other.pIID && self.pItf == other.pItf && self.hr == other.hr
    }
}
impl ::core::cmp::Eq for MULTI_QI {}
impl ::core::default::Default for MULTI_QI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct MachineGlobalObjectTableRegistrationToken__ {
    pub unused: i32,
}
impl ::core::marker::Copy for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::clone::Clone for MachineGlobalObjectTableRegistrationToken__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MachineGlobalObjectTableRegistrationToken__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MachineGlobalObjectTableRegistrationToken__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for MachineGlobalObjectTableRegistrationToken__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MachineGlobalObjectTableRegistrationToken__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MachineGlobalObjectTableRegistrationToken__>()) == 0 }
    }
}
impl ::core::cmp::Eq for MachineGlobalObjectTableRegistrationToken__ {}
impl ::core::default::Default for MachineGlobalObjectTableRegistrationToken__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn MkParseDisplayName<'a, Param0: ::windows::core::IntoParam<'a, IBindCtx>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pbc: Param0, szusername: Param1, pcheaten: *mut u32, ppmk: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MkParseDisplayName(pbc: ::windows::core::RawPtr, szusername: ::windows::core::PCWSTR, pcheaten: *mut u32, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        MkParseDisplayName(pbc.into_param().abi(), szusername.into_param().abi(), ::core::mem::transmute(pcheaten), ::core::mem::transmute(ppmk)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn MonikerCommonPrefixWith<'a, Param0: ::windows::core::IntoParam<'a, IMoniker>, Param1: ::windows::core::IntoParam<'a, IMoniker>>(pmkthis: Param0, pmkother: Param1) -> ::windows::core::Result<IMoniker> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MonikerCommonPrefixWith(pmkthis: ::windows::core::RawPtr, pmkother: ::windows::core::RawPtr, ppmkcommon: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        MonikerCommonPrefixWith(pmkthis.into_param().abi(), pmkother.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMoniker>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PENDINGMSG(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
impl ::core::marker::Copy for PENDINGMSG {}
impl ::core::clone::Clone for PENDINGMSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGMSG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PENDINGMSG {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGMSG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PENDINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
impl ::core::marker::Copy for PENDINGTYPE {}
impl ::core::clone::Clone for PENDINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PENDINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PENDINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PENDINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDINGTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub type PFNCONTEXTCALL = ::core::option::Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProgIDFromCLSID(clsid: *const ::windows::core::GUID, lplpszprogid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        ProgIDFromCLSID(::core::mem::transmute(clsid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl ::core::marker::Copy for QUERYCONTEXT {}
impl ::core::clone::Clone for QUERYCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERYCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERYCONTEXT").field("dwContext", &self.dwContext).field("Platform", &self.Platform).field("Locale", &self.Locale).field("dwVersionHi", &self.dwVersionHi).field("dwVersionLo", &self.dwVersionLo).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERYCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERYCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERYCONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERYCONTEXT {}
impl ::core::default::Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REGCLS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
impl ::core::marker::Copy for REGCLS {}
impl ::core::clone::Clone for REGCLS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REGCLS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REGCLS {
    type Abi = Self;
}
impl ::core::fmt::Debug for REGCLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGCLS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut ::core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut ::core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl ::core::marker::Copy for RPCOLEMESSAGE {}
impl ::core::clone::Clone for RPCOLEMESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPCOLEMESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPCOLEMESSAGE").field("reserved1", &self.reserved1).field("dataRepresentation", &self.dataRepresentation).field("Buffer", &self.Buffer).field("cbBuffer", &self.cbBuffer).field("iMethod", &self.iMethod).field("reserved2", &self.reserved2).field("rpcFlags", &self.rpcFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPCOLEMESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPCOLEMESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPCOLEMESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPCOLEMESSAGE {}
impl ::core::default::Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPCOPT_PROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
impl ::core::marker::Copy for RPCOPT_PROPERTIES {}
impl ::core::clone::Clone for RPCOPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_PROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPCOPT_PROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
impl ::core::marker::Copy for RPCOPT_SERVER_LOCALITY_VALUES {}
impl ::core::clone::Clone for RPCOPT_SERVER_LOCALITY_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPCOPT_SERVER_LOCALITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPCOPT_SERVER_LOCALITY_VALUES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPCOPT_SERVER_LOCALITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPCOPT_SERVER_LOCALITY_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
impl ::core::marker::Copy for RPC_C_AUTHN_LEVEL {}
impl ::core::clone::Clone for RPC_C_AUTHN_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_AUTHN_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_AUTHN_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_IMP_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
impl ::core::marker::Copy for RPC_C_IMP_LEVEL {}
impl ::core::clone::Clone for RPC_C_IMP_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_IMP_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_IMP_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_IMP_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_IMP_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemSTGMEDIUM {}
impl ::core::clone::Clone for RemSTGMEDIUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RemSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemSTGMEDIUM").field("tymed", &self.tymed).field("dwHandleType", &self.dwHandleType).field("pData", &self.pData).field("pUnkForRelease", &self.pUnkForRelease).field("cbData", &self.cbData).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for RemSTGMEDIUM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RemSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RemSTGMEDIUM>()) == 0 }
    }
}
impl ::core::cmp::Eq for RemSTGMEDIUM {}
impl ::core::default::Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut ::core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl ::core::marker::Copy for SAFEARRAY {}
impl ::core::clone::Clone for SAFEARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAY").field("cDims", &self.cDims).field("fFeatures", &self.fFeatures).field("cbElements", &self.cbElements).field("cLocks", &self.cLocks).field("pvData", &self.pvData).field("rgsabound", &self.rgsabound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAY {}
impl ::core::default::Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl ::core::marker::Copy for SAFEARRAYBOUND {}
impl ::core::clone::Clone for SAFEARRAYBOUND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SAFEARRAYBOUND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SAFEARRAYBOUND").field("cElements", &self.cElements).field("lLbound", &self.lLbound).finish()
    }
}
unsafe impl ::windows::core::Abi for SAFEARRAYBOUND {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SAFEARRAYBOUND {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SAFEARRAYBOUND>()) == 0 }
    }
}
impl ::core::cmp::Eq for SAFEARRAYBOUND {}
impl ::core::default::Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SChannelHookCallInfo {
    pub iid: ::windows::core::GUID,
    pub cbSize: u32,
    pub uCausality: ::windows::core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SChannelHookCallInfo {}
impl ::core::clone::Clone for SChannelHookCallInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SChannelHookCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SChannelHookCallInfo").field("iid", &self.iid).field("cbSize", &self.cbSize).field("uCausality", &self.uCausality).field("dwServerPid", &self.dwServerPid).field("iMethod", &self.iMethod).field("pObject", &self.pObject).finish()
    }
}
unsafe impl ::windows::core::Abi for SChannelHookCallInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SChannelHookCallInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SChannelHookCallInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for SChannelHookCallInfo {}
impl ::core::default::Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVERCALL(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
impl ::core::marker::Copy for SERVERCALL {}
impl ::core::clone::Clone for SERVERCALL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVERCALL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVERCALL {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVERCALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVERCALL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SHORT_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl ::core::marker::Copy for SHORT_SIZEDARR {}
impl ::core::clone::Clone for SHORT_SIZEDARR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SHORT_SIZEDARR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHORT_SIZEDARR").field("clSize", &self.clSize).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for SHORT_SIZEDARR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SHORT_SIZEDARR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SHORT_SIZEDARR>()) == 0 }
    }
}
impl ::core::cmp::Eq for SHORT_SIZEDARR {}
impl ::core::default::Default for SHORT_SIZEDARR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_INFO {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_INFO").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pAuthInfo", &self.pAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_INFO {}
impl ::core::default::Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_LIST {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_LIST").field("cAuthInfo", &self.cAuthInfo).field("aAuthInfo", &self.aAuthInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_LIST {}
impl ::core::default::Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: ::windows::core::PWSTR,
    pub hr: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::clone::Clone for SOLE_AUTHENTICATION_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOLE_AUTHENTICATION_SERVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOLE_AUTHENTICATION_SERVICE").field("dwAuthnSvc", &self.dwAuthnSvc).field("dwAuthzSvc", &self.dwAuthzSvc).field("pPrincipalName", &self.pPrincipalName).field("hr", &self.hr).finish()
    }
}
unsafe impl ::windows::core::Abi for SOLE_AUTHENTICATION_SERVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOLE_AUTHENTICATION_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOLE_AUTHENTICATION_SERVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOLE_AUTHENTICATION_SERVICE {}
impl ::core::default::Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: ::core::option::Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl ::core::clone::Clone for STATDATA {
    fn clone(&self) -> Self {
        Self { formatetc: self.formatetc, advf: self.advf, pAdvSink: self.pAdvSink.clone(), dwConnection: self.dwConnection }
    }
}
impl ::core::fmt::Debug for STATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATDATA").field("formatetc", &self.formatetc).field("advf", &self.advf).field("pAdvSink", &self.pAdvSink).field("dwConnection", &self.dwConnection).finish()
    }
}
unsafe impl ::windows::core::Abi for STATDATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for STATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.formatetc == other.formatetc && self.advf == other.advf && self.pAdvSink == other.pAdvSink && self.dwConnection == other.dwConnection
    }
}
impl ::core::cmp::Eq for STATDATA {}
impl ::core::default::Default for STATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATSTG {
    pub pwcsName: ::windows::core::PWSTR,
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
impl ::core::marker::Copy for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATSTG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STATSTG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STATSTG").field("pwcsName", &self.pwcsName).field("type", &self.r#type).field("cbSize", &self.cbSize).field("mtime", &self.mtime).field("ctime", &self.ctime).field("atime", &self.atime).field("grfMode", &self.grfMode).field("grfLocksSupported", &self.grfLocksSupported).field("clsid", &self.clsid).field("grfStateBits", &self.grfStateBits).field("reserved", &self.reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STATSTG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STATSTG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATSTG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STATSTG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATSTG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub Anonymous: STGMEDIUM_0,
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        Self { tymed: self.tymed, Anonymous: self.Anonymous.clone(), pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.tymed == other.tymed && self.Anonymous == other.Anonymous && self.pUnkForRelease == other.pUnkForRelease
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut ::core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: isize,
    pub lpszFileName: ::windows::core::PWSTR,
    pub pstm: ::core::mem::ManuallyDrop<::core::option::Option<IStream>>,
    pub pstg: ::core::mem::ManuallyDrop<::core::option::Option<StructuredStorage::IStorage>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for STGMEDIUM_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for STGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for STGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STGTY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_STREAM: STGTY = STGTY(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
impl ::core::marker::Copy for STGTY {}
impl ::core::clone::Clone for STGTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STGTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STGTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for STGTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STGTY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STGTY_REPEAT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STG_TOEND: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STREAM_SEEK(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
impl ::core::marker::Copy for STREAM_SEEK {}
impl ::core::clone::Clone for STREAM_SEEK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STREAM_SEEK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STREAM_SEEK {
    type Abi = Self;
}
impl ::core::fmt::Debug for STREAM_SEEK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STREAM_SEEK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SYSKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
impl ::core::marker::Copy for SYSKIND {}
impl ::core::clone::Clone for SYSKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYSKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYSKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYSKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ShutdownType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
impl ::core::marker::Copy for ShutdownType {}
impl ::core::clone::Clone for ShutdownType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ShutdownType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ShutdownType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ShutdownType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShutdownType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: ::windows::core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl ::core::marker::Copy for StorageLayout {}
impl ::core::clone::Clone for StorageLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for StorageLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StorageLayout").field("LayoutType", &self.LayoutType).field("pwcsElementName", &self.pwcsElementName).field("cOffset", &self.cOffset).field("cBytes", &self.cBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for StorageLayout {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for StorageLayout {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<StorageLayout>()) == 0 }
    }
}
impl ::core::cmp::Eq for StorageLayout {}
impl ::core::default::Default for StorageLayout {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromCLSID(rclsid: *const ::windows::core::GUID, lplpsz: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        StringFromCLSID(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: &mut [u16]) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromGUID2(rguid: *const ::windows::core::GUID, lpsz: ::windows::core::PWSTR, cchmax: i32) -> i32;
        }
        ::core::mem::transmute(StringFromGUID2(::core::mem::transmute(rguid), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpsz)), lpsz.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[inline]
pub unsafe fn StringFromIID(rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StringFromIID(rclsid: *const ::windows::core::GUID, lplpsz: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        StringFromIID(::core::mem::transmute(rclsid), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
impl ::core::marker::Copy for THDTYPE {}
impl ::core::clone::Clone for THDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THDTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for THDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THDTYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct TLIBATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl ::core::marker::Copy for TLIBATTR {}
impl ::core::clone::Clone for TLIBATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TLIBATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TLIBATTR").field("guid", &self.guid).field("lcid", &self.lcid).field("syskind", &self.syskind).field("wMajorVerNum", &self.wMajorVerNum).field("wMinorVerNum", &self.wMinorVerNum).field("wLibFlags", &self.wLibFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for TLIBATTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TLIBATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TLIBATTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for TLIBATTR {}
impl ::core::default::Default for TLIBATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYMED(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_FILE: TYMED = TYMED(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_GDI: TYMED = TYMED(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYMED_NULL: TYMED = TYMED(0i32);
impl ::core::marker::Copy for TYMED {}
impl ::core::clone::Clone for TYMED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYMED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYMED {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYMED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYMED").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEATTR {
    pub guid: ::windows::core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: ::windows::core::PWSTR,
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
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEATTR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEATTR {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEATTR>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEATTR {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: u16,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::marker::Copy for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::clone::Clone for TYPEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ole")]
unsafe impl ::windows::core::Abi for TYPEDESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::PartialEq for TYPEDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TYPEDESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::cmp::Eq for TYPEDESC_0 {}
#[cfg(feature = "Win32_System_Ole")]
impl ::core::default::Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYPEKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
impl ::core::marker::Copy for TYPEKIND {}
impl ::core::clone::Clone for TYPEKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYPEKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYPEKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYPEKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYPEKIND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TYSPEC(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
impl ::core::marker::Copy for TYSPEC {}
impl ::core::clone::Clone for TYSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TYSPEC {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TYSPEC {
    type Abi = Self;
}
impl ::core::fmt::Debug for TYSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TYSPEC").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct URI_CREATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
impl ::core::marker::Copy for URI_CREATE_FLAGS {}
impl ::core::clone::Clone for URI_CREATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for URI_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for URI_CREATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for URI_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URI_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Uri_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
impl ::core::marker::Copy for Uri_PROPERTY {}
impl ::core::clone::Clone for Uri_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Uri_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Uri_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for Uri_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: ::windows::core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: u16,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARDESC_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARDESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARDESC_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARDESC_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT {
    fn clone(&self) -> Self {
        Self { Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT {
    fn eq(&self, other: &Self) -> bool {
        self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub union VARIANT_0 {
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: super::super::Foundation::DECIMAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0 {
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        Self { vt: self.vt, wReserved1: self.wReserved1, wReserved2: self.wReserved2, wReserved3: self.wReserved3, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.vt == other.vt && self.wReserved1 == other.wReserved1 && self.wReserved2 == other.wReserved2 && self.wReserved3 == other.wReserved3 && self.Anonymous == other.Anonymous
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
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
    pub punkVal: ::core::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub pdispVal: ::core::mem::ManuallyDrop<::core::option::Option<IDispatch>>,
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
    pub pbstrVal: *mut super::super::Foundation::BSTR,
    pub ppunkVal: *mut ::core::option::Option<::windows::core::IUnknown>,
    pub ppdispVal: *mut ::core::option::Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut ::core::ffi::c_void,
    pub cVal: super::super::Foundation::CHAR,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: ::windows::core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: ::core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for VARIANT_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VARIANT_0_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for VARIANT_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::default::Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut ::core::ffi::c_void,
    pub pRecInfo: ::core::option::Option<super::Ole::IRecordInfo>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for VARIANT_0_0_0_0 {
    fn clone(&self) -> Self {
        Self { pvRecord: self.pvRecord, pRecInfo: self.pRecInfo.clone() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for VARIANT_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VARIANT_0_0_0_0").field("pvRecord", &self.pvRecord).field("pRecInfo", &self.pRecInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
unsafe impl ::windows::core::Abi for VARIANT_0_0_0_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for VARIANT_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VARKIND(pub i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_CONST: VARKIND = VARKIND(2i32);
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
impl ::core::marker::Copy for VARKIND {}
impl ::core::clone::Clone for VARKIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VARKIND {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VARKIND {
    type Abi = Self;
}
impl ::core::fmt::Debug for VARKIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VARKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl ::core::marker::Copy for WORD_BLOB {}
impl ::core::clone::Clone for WORD_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WORD_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WORD_BLOB").field("clSize", &self.clSize).field("asData", &self.asData).finish()
    }
}
unsafe impl ::windows::core::Abi for WORD_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WORD_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WORD_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for WORD_BLOB {}
impl ::core::default::Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub const _CRT_INTERNAL_COMBASE_SYMBOL_PREFIX: &'static str = "_";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl ::core::marker::Copy for uCLSSPEC {}
impl ::core::clone::Clone for uCLSSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC {}
impl ::core::default::Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub union uCLSSPEC_0 {
    pub clsid: ::windows::core::GUID,
    pub pFileExt: ::windows::core::PWSTR,
    pub pMimeType: ::windows::core::PWSTR,
    pub pProgId: ::windows::core::PWSTR,
    pub pFileName: ::windows::core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl ::core::marker::Copy for uCLSSPEC_0 {}
impl ::core::clone::Clone for uCLSSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0 {}
impl ::core::default::Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: ::windows::core::PWSTR,
    pub PolicyId: ::windows::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_0 {}
impl ::core::clone::Clone for uCLSSPEC_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_0").field("pPackageName", &self.pPackageName).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_0 {}
impl ::core::default::Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: ::windows::core::GUID,
    pub PolicyId: ::windows::core::GUID,
}
impl ::core::marker::Copy for uCLSSPEC_0_1 {}
impl ::core::clone::Clone for uCLSSPEC_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for uCLSSPEC_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("uCLSSPEC_0_1").field("ObjectId", &self.ObjectId).field("PolicyId", &self.PolicyId).finish()
    }
}
unsafe impl ::windows::core::Abi for uCLSSPEC_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for uCLSSPEC_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<uCLSSPEC_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for uCLSSPEC_0_1 {}
impl ::core::default::Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
impl ::core::clone::Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        Self { ContextFlags: self.ContextFlags, fPassOwnership: self.fPassOwnership, Stgmed: self.Stgmed.clone() }
    }
}
impl ::core::fmt::Debug for userFLAG_STGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userFLAG_STGMEDIUM").field("ContextFlags", &self.ContextFlags).field("fPassOwnership", &self.fPassOwnership).field("Stgmed", &self.Stgmed).finish()
    }
}
unsafe impl ::windows::core::Abi for userFLAG_STGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userFLAG_STGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.fPassOwnership == other.fPassOwnership && self.Stgmed == other.Stgmed
    }
}
impl ::core::cmp::Eq for userFLAG_STGMEDIUM {}
impl ::core::default::Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`*"]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: ::core::option::Option<::windows::core::IUnknown>,
}
impl ::core::clone::Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        Self { pUnkForRelease: self.pUnkForRelease.clone() }
    }
}
impl ::core::fmt::Debug for userSTGMEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("userSTGMEDIUM").field("pUnkForRelease", &self.pUnkForRelease).finish()
    }
}
unsafe impl ::windows::core::Abi for userSTGMEDIUM {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
impl ::core::cmp::PartialEq for userSTGMEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.pUnkForRelease == other.pUnkForRelease
    }
}
impl ::core::cmp::Eq for userSTGMEDIUM {}
impl ::core::default::Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: ::windows::core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for userSTGMEDIUM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::core::Abi for userSTGMEDIUM_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::PartialEq for userSTGMEDIUM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<userSTGMEDIUM_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::cmp::Eq for userSTGMEDIUM_0_0 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl ::core::default::Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
