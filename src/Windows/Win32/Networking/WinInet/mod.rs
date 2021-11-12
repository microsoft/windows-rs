#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ANY_CACHE_ENTRY: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_DOWNLOAD_ENTRY {
    pub pwszUrl: super::super::Foundation::PWSTR,
    pub dwEntryType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl APP_CACHE_DOWNLOAD_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_DOWNLOAD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APP_CACHE_DOWNLOAD_ENTRY").field("pwszUrl", &self.pwszUrl).field("dwEntryType", &self.dwEntryType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszUrl == other.pwszUrl && self.dwEntryType == other.dwEntryType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APP_CACHE_DOWNLOAD_ENTRY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_DOWNLOAD_LIST {
    pub dwEntryCount: u32,
    pub pEntries: *mut APP_CACHE_DOWNLOAD_ENTRY,
}
#[cfg(feature = "Win32_Foundation")]
impl APP_CACHE_DOWNLOAD_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_DOWNLOAD_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_DOWNLOAD_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APP_CACHE_DOWNLOAD_LIST").field("dwEntryCount", &self.dwEntryCount).field("pEntries", &self.pEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_DOWNLOAD_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwEntryCount == other.dwEntryCount && self.pEntries == other.pEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_DOWNLOAD_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APP_CACHE_DOWNLOAD_LIST {
    type Abi = Self;
}
pub const APP_CACHE_ENTRY_TYPE_EXPLICIT: u32 = 2u32;
pub const APP_CACHE_ENTRY_TYPE_FALLBACK: u32 = 4u32;
pub const APP_CACHE_ENTRY_TYPE_FOREIGN: u32 = 8u32;
pub const APP_CACHE_ENTRY_TYPE_MANIFEST: u32 = 16u32;
pub const APP_CACHE_ENTRY_TYPE_MASTER: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APP_CACHE_FINALIZE_STATE(pub i32);
pub const AppCacheFinalizeStateIncomplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(0i32);
pub const AppCacheFinalizeStateManifestChange: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(1i32);
pub const AppCacheFinalizeStateComplete: APP_CACHE_FINALIZE_STATE = APP_CACHE_FINALIZE_STATE(2i32);
impl ::core::convert::From<i32> for APP_CACHE_FINALIZE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APP_CACHE_FINALIZE_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_GROUP_INFO {
    pub pwszManifestUrl: super::super::Foundation::PWSTR,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ullSize: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl APP_CACHE_GROUP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APP_CACHE_GROUP_INFO").field("pwszManifestUrl", &self.pwszManifestUrl).field("ftLastAccessTime", &self.ftLastAccessTime).field("ullSize", &self.ullSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszManifestUrl == other.pwszManifestUrl && self.ftLastAccessTime == other.ftLastAccessTime && self.ullSize == other.ullSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APP_CACHE_GROUP_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APP_CACHE_GROUP_LIST {
    pub dwAppCacheGroupCount: u32,
    pub pAppCacheGroups: *mut APP_CACHE_GROUP_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl APP_CACHE_GROUP_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for APP_CACHE_GROUP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for APP_CACHE_GROUP_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("APP_CACHE_GROUP_LIST").field("dwAppCacheGroupCount", &self.dwAppCacheGroupCount).field("pAppCacheGroups", &self.pAppCacheGroups).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for APP_CACHE_GROUP_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppCacheGroupCount == other.dwAppCacheGroupCount && self.pAppCacheGroups == other.pAppCacheGroups
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for APP_CACHE_GROUP_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for APP_CACHE_GROUP_LIST {
    type Abi = Self;
}
pub const APP_CACHE_LOOKUP_NO_MASTER_ONLY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct APP_CACHE_STATE(pub i32);
pub const AppCacheStateNoUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(0i32);
pub const AppCacheStateUpdateNeeded: APP_CACHE_STATE = APP_CACHE_STATE(1i32);
pub const AppCacheStateUpdateNeededNew: APP_CACHE_STATE = APP_CACHE_STATE(2i32);
pub const AppCacheStateUpdateNeededMasterOnly: APP_CACHE_STATE = APP_CACHE_STATE(3i32);
impl ::core::convert::From<i32> for APP_CACHE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for APP_CACHE_STATE {
    type Abi = Self;
}
pub const AUTH_FLAG_DISABLE_BASIC_CLEARCHANNEL: u32 = 4u32;
pub const AUTH_FLAG_DISABLE_NEGOTIATE: u32 = 1u32;
pub const AUTH_FLAG_DISABLE_SERVER_AUTH: u32 = 8u32;
pub const AUTH_FLAG_ENABLE_NEGOTIATE: u32 = 2u32;
pub const AUTH_FLAG_RESET: u32 = 0u32;
pub const AUTODIAL_MODE_ALWAYS: u32 = 2u32;
pub const AUTODIAL_MODE_NEVER: u32 = 1u32;
pub const AUTODIAL_MODE_NO_NETWORK_PRESENT: u32 = 4u32;
pub const AUTO_PROXY_FLAG_ALWAYS_DETECT: u32 = 2u32;
pub const AUTO_PROXY_FLAG_CACHE_INIT_RUN: u32 = 32u32;
pub const AUTO_PROXY_FLAG_DETECTION_RUN: u32 = 4u32;
pub const AUTO_PROXY_FLAG_DETECTION_SUSPECT: u32 = 64u32;
pub const AUTO_PROXY_FLAG_DONT_CACHE_PROXY_RESULT: u32 = 16u32;
pub const AUTO_PROXY_FLAG_MIGRATED: u32 = 8u32;
pub const AUTO_PROXY_FLAG_USER_SET: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTO_PROXY_SCRIPT_BUFFER {
    pub dwStructSize: u32,
    pub lpszScriptBuffer: super::super::Foundation::PSTR,
    pub dwScriptBufferSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl AUTO_PROXY_SCRIPT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTO_PROXY_SCRIPT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTO_PROXY_SCRIPT_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUTO_PROXY_SCRIPT_BUFFER").field("dwStructSize", &self.dwStructSize).field("lpszScriptBuffer", &self.lpszScriptBuffer).field("dwScriptBufferSize", &self.dwScriptBufferSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTO_PROXY_SCRIPT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszScriptBuffer == other.lpszScriptBuffer && self.dwScriptBufferSize == other.dwScriptBufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTO_PROXY_SCRIPT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUTO_PROXY_SCRIPT_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheCheckManifest<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszmasterurl: Param0, pwszmanifesturl: Param1, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pbmanifestresponseheaders: *const u8, dwmanifestresponseheaderssize: u32, pestate: *mut APP_CACHE_STATE, phnewappcache: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheCheckManifest(pwszmasterurl: super::super::Foundation::PWSTR, pwszmanifesturl: super::super::Foundation::PWSTR, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pbmanifestresponseheaders: *const u8, dwmanifestresponseheaderssize: u32, pestate: *mut APP_CACHE_STATE, phnewappcache: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(AppCacheCheckManifest(
            pwszmasterurl.into_param().abi(),
            pwszmanifesturl.into_param().abi(),
            ::core::mem::transmute(pbmanifestdata),
            ::core::mem::transmute(dwmanifestdatasize),
            ::core::mem::transmute(pbmanifestresponseheaders),
            ::core::mem::transmute(dwmanifestresponseheaderssize),
            ::core::mem::transmute(pestate),
            ::core::mem::transmute(phnewappcache),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AppCacheCloseHandle(happcache: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheCloseHandle(happcache: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(AppCacheCloseHandle(::core::mem::transmute(happcache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheCreateAndCommitFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(happcache: *const ::core::ffi::c_void, pwszsourcefilepath: Param1, pwszurl: Param2, pbresponseheaders: *const u8, dwresponseheaderssize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheCreateAndCommitFile(happcache: *const ::core::ffi::c_void, pwszsourcefilepath: super::super::Foundation::PWSTR, pwszurl: super::super::Foundation::PWSTR, pbresponseheaders: *const u8, dwresponseheaderssize: u32) -> u32;
        }
        ::core::mem::transmute(AppCacheCreateAndCommitFile(::core::mem::transmute(happcache), pwszsourcefilepath.into_param().abi(), pwszurl.into_param().abi(), ::core::mem::transmute(pbresponseheaders), ::core::mem::transmute(dwresponseheaderssize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheDeleteGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszmanifesturl: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheDeleteGroup(pwszmanifesturl: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(AppCacheDeleteGroup(pwszmanifesturl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheDeleteIEGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszmanifesturl: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheDeleteIEGroup(pwszmanifesturl: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(AppCacheDeleteIEGroup(pwszmanifesturl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AppCacheDuplicateHandle(happcache: *const ::core::ffi::c_void, phduplicatedappcache: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheDuplicateHandle(happcache: *const ::core::ffi::c_void, phduplicatedappcache: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(AppCacheDuplicateHandle(::core::mem::transmute(happcache), ::core::mem::transmute(phduplicatedappcache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AppCacheFinalize(happcache: *const ::core::ffi::c_void, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pestate: *mut APP_CACHE_FINALIZE_STATE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheFinalize(happcache: *const ::core::ffi::c_void, pbmanifestdata: *const u8, dwmanifestdatasize: u32, pestate: *mut APP_CACHE_FINALIZE_STATE) -> u32;
        }
        ::core::mem::transmute(AppCacheFinalize(::core::mem::transmute(happcache), ::core::mem::transmute(pbmanifestdata), ::core::mem::transmute(dwmanifestdatasize), ::core::mem::transmute(pestate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeDownloadList(pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheFreeDownloadList(pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST);
        }
        ::core::mem::transmute(AppCacheFreeDownloadList(::core::mem::transmute(pdownloadlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheFreeGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST);
        }
        ::core::mem::transmute(AppCacheFreeGroupList(::core::mem::transmute(pappcachegrouplist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeIESpace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ftcutoff: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheFreeIESpace(ftcutoff: super::super::Foundation::FILETIME) -> u32;
        }
        ::core::mem::transmute(AppCacheFreeIESpace(ftcutoff.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheFreeSpace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(ftcutoff: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheFreeSpace(ftcutoff: super::super::Foundation::FILETIME) -> u32;
        }
        ::core::mem::transmute(AppCacheFreeSpace(ftcutoff.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetDownloadList(happcache: *const ::core::ffi::c_void, pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetDownloadList(happcache: *const ::core::ffi::c_void, pdownloadlist: *mut APP_CACHE_DOWNLOAD_LIST) -> u32;
        }
        ::core::mem::transmute(AppCacheGetDownloadList(::core::mem::transmute(happcache), ::core::mem::transmute(pdownloadlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetFallbackUrl<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(happcache: *const ::core::ffi::c_void, pwszurl: Param1, ppwszfallbackurl: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetFallbackUrl(happcache: *const ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, ppwszfallbackurl: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(AppCacheGetFallbackUrl(::core::mem::transmute(happcache), pwszurl.into_param().abi(), ::core::mem::transmute(ppwszfallbackurl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32;
        }
        ::core::mem::transmute(AppCacheGetGroupList(::core::mem::transmute(pappcachegrouplist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetIEGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetIEGroupList(pappcachegrouplist: *mut APP_CACHE_GROUP_LIST) -> u32;
        }
        ::core::mem::transmute(AppCacheGetIEGroupList(::core::mem::transmute(pappcachegrouplist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetInfo(happcache: *const ::core::ffi::c_void, pappcacheinfo: *mut APP_CACHE_GROUP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetInfo(happcache: *const ::core::ffi::c_void, pappcacheinfo: *mut APP_CACHE_GROUP_INFO) -> u32;
        }
        ::core::mem::transmute(AppCacheGetInfo(::core::mem::transmute(happcache), ::core::mem::transmute(pappcacheinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheGetManifestUrl(happcache: *const ::core::ffi::c_void, ppwszmanifesturl: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheGetManifestUrl(happcache: *const ::core::ffi::c_void, ppwszmanifesturl: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(AppCacheGetManifestUrl(::core::mem::transmute(happcache), ::core::mem::transmute(ppwszmanifesturl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppCacheLookup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszurl: Param0, dwflags: u32, phappcache: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AppCacheLookup(pwszurl: super::super::Foundation::PWSTR, dwflags: u32, phappcache: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(AppCacheLookup(pwszurl.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(phappcache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AutoProxyHelperFunctions {
    pub lpVtbl: *mut AutoProxyHelperVtbl,
}
impl AutoProxyHelperFunctions {}
impl ::core::default::Default for AutoProxyHelperFunctions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AutoProxyHelperFunctions {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AutoProxyHelperFunctions").field("lpVtbl", &self.lpVtbl).finish()
    }
}
impl ::core::cmp::PartialEq for AutoProxyHelperFunctions {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl
    }
}
impl ::core::cmp::Eq for AutoProxyHelperFunctions {}
unsafe impl ::windows::core::Abi for AutoProxyHelperFunctions {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AutoProxyHelperVtbl {
    pub IsResolvable: isize,
    pub GetIPAddress: isize,
    pub ResolveHostName: isize,
    pub IsInNet: isize,
    pub IsResolvableEx: isize,
    pub GetIPAddressEx: isize,
    pub ResolveHostNameEx: isize,
    pub IsInNetEx: isize,
    pub SortIpList: isize,
}
impl AutoProxyHelperVtbl {}
impl ::core::default::Default for AutoProxyHelperVtbl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AutoProxyHelperVtbl {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AutoProxyHelperVtbl")
            .field("IsResolvable", &self.IsResolvable)
            .field("GetIPAddress", &self.GetIPAddress)
            .field("ResolveHostName", &self.ResolveHostName)
            .field("IsInNet", &self.IsInNet)
            .field("IsResolvableEx", &self.IsResolvableEx)
            .field("GetIPAddressEx", &self.GetIPAddressEx)
            .field("ResolveHostNameEx", &self.ResolveHostNameEx)
            .field("IsInNetEx", &self.IsInNetEx)
            .field("SortIpList", &self.SortIpList)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AutoProxyHelperVtbl {
    fn eq(&self, other: &Self) -> bool {
        self.IsResolvable == other.IsResolvable && self.GetIPAddress == other.GetIPAddress && self.ResolveHostName == other.ResolveHostName && self.IsInNet == other.IsInNet && self.IsResolvableEx == other.IsResolvableEx && self.GetIPAddressEx == other.GetIPAddressEx && self.ResolveHostNameEx == other.ResolveHostNameEx && self.IsInNetEx == other.IsInNetEx && self.SortIpList == other.SortIpList
    }
}
impl ::core::cmp::Eq for AutoProxyHelperVtbl {}
unsafe impl ::windows::core::Abi for AutoProxyHelperVtbl {
    type Abi = Self;
}
pub const CACHEGROUP_ATTRIBUTE_BASIC: u32 = 1u32;
pub const CACHEGROUP_ATTRIBUTE_FLAG: u32 = 2u32;
pub const CACHEGROUP_ATTRIBUTE_GET_ALL: u32 = 4294967295u32;
pub const CACHEGROUP_ATTRIBUTE_GROUPNAME: u32 = 16u32;
pub const CACHEGROUP_ATTRIBUTE_QUOTA: u32 = 8u32;
pub const CACHEGROUP_ATTRIBUTE_STORAGE: u32 = 32u32;
pub const CACHEGROUP_ATTRIBUTE_TYPE: u32 = 4u32;
pub const CACHEGROUP_FLAG_FLUSHURL_ONDELETE: u32 = 2u32;
pub const CACHEGROUP_FLAG_GIDONLY: u32 = 4u32;
pub const CACHEGROUP_FLAG_NONPURGEABLE: u32 = 1u32;
pub const CACHEGROUP_FLAG_VALID: u32 = 7u32;
pub const CACHEGROUP_ID_BUILTIN_STICKY: u64 = 1152921504606846983u64;
pub const CACHEGROUP_SEARCH_ALL: u32 = 0u32;
pub const CACHEGROUP_SEARCH_BYURL: u32 = 1u32;
pub const CACHEGROUP_TYPE_INVALID: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CACHE_CONFIG(pub u32);
pub const CACHE_CONFIG_FORCE_CLEANUP_FC: CACHE_CONFIG = CACHE_CONFIG(32u32);
pub const CACHE_CONFIG_DISK_CACHE_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(64u32);
pub const CACHE_CONFIG_SYNC_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(128u32);
pub const CACHE_CONFIG_CONTENT_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(256u32);
pub const CACHE_CONFIG_HISTORY_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(1024u32);
pub const CACHE_CONFIG_COOKIES_PATHS_FC: CACHE_CONFIG = CACHE_CONFIG(512u32);
pub const CACHE_CONFIG_QUOTA_FC: CACHE_CONFIG = CACHE_CONFIG(2048u32);
pub const CACHE_CONFIG_USER_MODE_FC: CACHE_CONFIG = CACHE_CONFIG(4096u32);
pub const CACHE_CONFIG_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(8192u32);
pub const CACHE_CONFIG_STICKY_CONTENT_USAGE_FC: CACHE_CONFIG = CACHE_CONFIG(16384u32);
impl ::core::convert::From<u32> for CACHE_CONFIG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CACHE_CONFIG {
    type Abi = Self;
}
impl ::core::ops::BitOr for CACHE_CONFIG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CACHE_CONFIG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CACHE_CONFIG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CACHE_CONFIG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CACHE_CONFIG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CACHE_CONFIG_APPCONTAINER_CONTENT_QUOTA_FC: u32 = 131072u32;
pub const CACHE_CONFIG_APPCONTAINER_TOTAL_CONTENT_QUOTA_FC: u32 = 262144u32;
pub const CACHE_CONFIG_CONTENT_QUOTA_FC: u32 = 32768u32;
pub const CACHE_CONFIG_TOTAL_CONTENT_QUOTA_FC: u32 = 65536u32;
pub const CACHE_ENTRY_ACCTIME_FC: u32 = 256u32;
pub const CACHE_ENTRY_ATTRIBUTE_FC: u32 = 4u32;
pub const CACHE_ENTRY_EXEMPT_DELTA_FC: u32 = 2048u32;
pub const CACHE_ENTRY_EXPTIME_FC: u32 = 128u32;
pub const CACHE_ENTRY_HEADERINFO_FC: u32 = 1024u32;
pub const CACHE_ENTRY_HITRATE_FC: u32 = 16u32;
pub const CACHE_ENTRY_MODIFY_DATA_FC: u32 = 2147483648u32;
pub const CACHE_ENTRY_MODTIME_FC: u32 = 64u32;
pub const CACHE_ENTRY_SYNCTIME_FC: u32 = 512u32;
pub const CACHE_ENTRY_TYPE_FC: u32 = 4096u32;
pub const CACHE_FIND_CONTAINER_RETURN_NOCHANGE: u32 = 1u32;
pub const CACHE_HEADER_DATA_CACHE_READ_COUNT_SINCE_LAST_SCAVENGE: u32 = 9u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_12: u32 = 12u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_13: u32 = 13u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_15: u32 = 15u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_16: u32 = 16u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_17: u32 = 17u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_18: u32 = 18u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_19: u32 = 19u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_20: u32 = 20u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_23: u32 = 23u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_24: u32 = 24u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_25: u32 = 25u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_26: u32 = 26u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_28: u32 = 28u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_29: u32 = 29u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_30: u32 = 30u32;
pub const CACHE_HEADER_DATA_CACHE_RESERVED_31: u32 = 31u32;
pub const CACHE_HEADER_DATA_CACHE_WRITE_COUNT_SINCE_LAST_SCAVENGE: u32 = 10u32;
pub const CACHE_HEADER_DATA_CONLIST_CHANGE_COUNT: u32 = 1u32;
pub const CACHE_HEADER_DATA_COOKIE_CHANGE_COUNT: u32 = 2u32;
pub const CACHE_HEADER_DATA_CURRENT_SETTINGS_VERSION: u32 = 0u32;
pub const CACHE_HEADER_DATA_DOWNLOAD_PARTIAL: u32 = 14u32;
pub const CACHE_HEADER_DATA_GID_HIGH: u32 = 7u32;
pub const CACHE_HEADER_DATA_GID_LOW: u32 = 6u32;
pub const CACHE_HEADER_DATA_HSTS_CHANGE_COUNT: u32 = 11u32;
pub const CACHE_HEADER_DATA_LAST: u32 = 31u32;
pub const CACHE_HEADER_DATA_LAST_SCAVENGE_TIMESTAMP: u32 = 8u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_FILTER: u32 = 21u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_HWND: u32 = 3u32;
pub const CACHE_HEADER_DATA_NOTIFICATION_MESG: u32 = 4u32;
pub const CACHE_HEADER_DATA_ROOTGROUP_OFFSET: u32 = 5u32;
pub const CACHE_HEADER_DATA_ROOT_GROUPLIST_OFFSET: u32 = 27u32;
pub const CACHE_HEADER_DATA_ROOT_LEAK_OFFSET: u32 = 22u32;
pub const CACHE_HEADER_DATA_SSL_STATE_COUNT: u32 = 14u32;
pub const CACHE_NOTIFY_ADD_URL: u32 = 1u32;
pub const CACHE_NOTIFY_DELETE_ALL: u32 = 8u32;
pub const CACHE_NOTIFY_DELETE_URL: u32 = 2u32;
pub const CACHE_NOTIFY_FILTER_CHANGED: u32 = 268435456u32;
pub const CACHE_NOTIFY_SET_OFFLINE: u32 = 512u32;
pub const CACHE_NOTIFY_SET_ONLINE: u32 = 256u32;
pub const CACHE_NOTIFY_UPDATE_URL: u32 = 4u32;
pub const CACHE_NOTIFY_URL_SET_STICKY: u32 = 16u32;
pub const CACHE_NOTIFY_URL_UNSET_STICKY: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
pub type CACHE_OPERATOR = unsafe extern "system" fn(pcei: *mut INTERNET_CACHE_ENTRY_INFOA, pcbcei: *mut u32, popdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub const COOKIE_ACCEPTED_CACHE_ENTRY: u32 = 4096u32;
pub const COOKIE_ALLOW: u32 = 2u32;
pub const COOKIE_ALLOW_ALL: u32 = 4u32;
pub const COOKIE_CACHE_ENTRY: u32 = 1048576u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COOKIE_DLG_INFO {
    pub pszServer: super::super::Foundation::PWSTR,
    pub pic: *mut INTERNET_COOKIE,
    pub dwStopWarning: u32,
    pub cx: i32,
    pub cy: i32,
    pub pszHeader: super::super::Foundation::PWSTR,
    pub dwOperation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl COOKIE_DLG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COOKIE_DLG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COOKIE_DLG_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COOKIE_DLG_INFO").field("pszServer", &self.pszServer).field("pic", &self.pic).field("dwStopWarning", &self.dwStopWarning).field("cx", &self.cx).field("cy", &self.cy).field("pszHeader", &self.pszHeader).field("dwOperation", &self.dwOperation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COOKIE_DLG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszServer == other.pszServer && self.pic == other.pic && self.dwStopWarning == other.dwStopWarning && self.cx == other.cx && self.cy == other.cy && self.pszHeader == other.pszHeader && self.dwOperation == other.dwOperation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COOKIE_DLG_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COOKIE_DLG_INFO {
    type Abi = Self;
}
pub const COOKIE_DONT_ALLOW: u32 = 1u32;
pub const COOKIE_DONT_ALLOW_ALL: u32 = 8u32;
pub const COOKIE_DOWNGRADED_CACHE_ENTRY: u32 = 16384u32;
pub const COOKIE_LEASHED_CACHE_ENTRY: u32 = 8192u32;
pub const COOKIE_OP_3RD_PARTY: u32 = 32u32;
pub const COOKIE_OP_GET: u32 = 4u32;
pub const COOKIE_OP_MODIFY: u32 = 2u32;
pub const COOKIE_OP_PERSISTENT: u32 = 16u32;
pub const COOKIE_OP_SESSION: u32 = 8u32;
pub const COOKIE_OP_SET: u32 = 1u32;
pub const COOKIE_REJECTED_CACHE_ENTRY: u32 = 32768u32;
pub const COOKIE_STATE_LB: u32 = 0u32;
pub const COOKIE_STATE_UB: u32 = 5u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
    lpszurlname: Param0,
    lpszlocalfilename: Param1,
    expiretime: Param2,
    lastmodifiedtime: Param3,
    cacheentrytype: u32,
    lpheaderinfo: *const u8,
    cchheaderinfo: u32,
    lpszfileextension: Param7,
    lpszoriginalurl: Param8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR, lpszlocalfilename: super::super::Foundation::PSTR, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpheaderinfo: *const u8, cchheaderinfo: u32, lpszfileextension: super::super::Foundation::PSTR, lpszoriginalurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CommitUrlCacheEntryA(
            lpszurlname.into_param().abi(),
            lpszlocalfilename.into_param().abi(),
            expiretime.into_param().abi(),
            lastmodifiedtime.into_param().abi(),
            ::core::mem::transmute(cacheentrytype),
            ::core::mem::transmute(lpheaderinfo),
            ::core::mem::transmute(cchheaderinfo),
            lpszfileextension.into_param().abi(),
            lpszoriginalurl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryBinaryBlob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>>(pwszurlname: Param0, dwtype: u32, ftexpiretime: Param2, ftmodifiedtime: Param3, pbblob: *const u8, cbblob: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitUrlCacheEntryBinaryBlob(pwszurlname: super::super::Foundation::PWSTR, dwtype: u32, ftexpiretime: super::super::Foundation::FILETIME, ftmodifiedtime: super::super::Foundation::FILETIME, pbblob: *const u8, cbblob: u32) -> u32;
        }
        ::core::mem::transmute(CommitUrlCacheEntryBinaryBlob(pwszurlname.into_param().abi(), ::core::mem::transmute(dwtype), ftexpiretime.into_param().abi(), ftmodifiedtime.into_param().abi(), ::core::mem::transmute(pbblob), ::core::mem::transmute(cbblob)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommitUrlCacheEntryW<
    'a,
    Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>,
    Param3: ::windows::core::IntoParam<'a, super::super::Foundation::FILETIME>,
    Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszurlname: Param0,
    lpszlocalfilename: Param1,
    expiretime: Param2,
    lastmodifiedtime: Param3,
    cacheentrytype: u32,
    lpszheaderinfo: Param5,
    cchheaderinfo: u32,
    lpszfileextension: Param7,
    lpszoriginalurl: Param8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommitUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR, lpszlocalfilename: super::super::Foundation::PWSTR, expiretime: super::super::Foundation::FILETIME, lastmodifiedtime: super::super::Foundation::FILETIME, cacheentrytype: u32, lpszheaderinfo: super::super::Foundation::PWSTR, cchheaderinfo: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszoriginalurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CommitUrlCacheEntryW(
            lpszurlname.into_param().abi(),
            lpszlocalfilename.into_param().abi(),
            expiretime.into_param().abi(),
            lastmodifiedtime.into_param().abi(),
            ::core::mem::transmute(cacheentrytype),
            lpszheaderinfo.into_param().abi(),
            ::core::mem::transmute(cchheaderinfo),
            lpszfileextension.into_param().abi(),
            lpszoriginalurl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CookieDecision {
    pub dwCookieState: u32,
    pub fAllowSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl CookieDecision {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CookieDecision {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CookieDecision {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CookieDecision").field("dwCookieState", &self.dwCookieState).field("fAllowSession", &self.fAllowSession).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CookieDecision {
    fn eq(&self, other: &Self) -> bool {
        self.dwCookieState == other.dwCookieState && self.fAllowSession == other.fAllowSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CookieDecision {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CookieDecision {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMD5SSOHash<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszchallengeinfo: Param0, pwszrealm: Param1, pwsztarget: Param2, pbhexhash: *mut u8) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMD5SSOHash(pszchallengeinfo: super::super::Foundation::PWSTR, pwszrealm: super::super::Foundation::PWSTR, pwsztarget: super::super::Foundation::PWSTR, pbhexhash: *mut u8) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateMD5SSOHash(pszchallengeinfo.into_param().abi(), pwszrealm.into_param().abi(), pwsztarget.into_param().abi(), ::core::mem::transmute(pbhexhash)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheContainerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(name: Param0, lpcacheprefix: Param1, lpszcachepath: Param2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheContainerA(name: super::super::Foundation::PSTR, lpcacheprefix: super::super::Foundation::PSTR, lpszcachepath: super::super::Foundation::PSTR, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUrlCacheContainerA(name.into_param().abi(), lpcacheprefix.into_param().abi(), lpszcachepath.into_param().abi(), ::core::mem::transmute(kbcachelimit), ::core::mem::transmute(dwcontainertype), ::core::mem::transmute(dwoptions), ::core::mem::transmute(pvbuffer), ::core::mem::transmute(cbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheContainerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(name: Param0, lpcacheprefix: Param1, lpszcachepath: Param2, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheContainerW(name: super::super::Foundation::PWSTR, lpcacheprefix: super::super::Foundation::PWSTR, lpszcachepath: super::super::Foundation::PWSTR, kbcachelimit: u32, dwcontainertype: u32, dwoptions: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUrlCacheContainerW(name.into_param().abi(), lpcacheprefix.into_param().abi(), lpszcachepath.into_param().abi(), ::core::mem::transmute(kbcachelimit), ::core::mem::transmute(dwcontainertype), ::core::mem::transmute(dwoptions), ::core::mem::transmute(pvbuffer), ::core::mem::transmute(cbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwexpectedfilesize: u32, lpszfileextension: Param2, lpszfilename: Param3, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PSTR, lpszfilename: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUrlCacheEntryA(lpszurlname.into_param().abi(), ::core::mem::transmute(dwexpectedfilesize), lpszfileextension.into_param().abi(), lpszfilename.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(
    lpszurlname: Param0,
    dwexpectedfilesize: u32,
    lpszfileextension: Param2,
    lpszfilename: Param3,
    dwreserved: u32,
    fpreserveincomingfilename: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheEntryExW(lpszurlname: super::super::Foundation::PWSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszfilename: super::super::Foundation::PWSTR, dwreserved: u32, fpreserveincomingfilename: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUrlCacheEntryExW(lpszurlname.into_param().abi(), ::core::mem::transmute(dwexpectedfilesize), lpszfileextension.into_param().abi(), lpszfilename.into_param().abi(), ::core::mem::transmute(dwreserved), fpreserveincomingfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUrlCacheEntryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, dwexpectedfilesize: u32, lpszfileextension: Param2, lpszfilename: Param3, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR, dwexpectedfilesize: u32, lpszfileextension: super::super::Foundation::PWSTR, lpszfilename: super::super::Foundation::PWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUrlCacheEntryW(lpszurlname.into_param().abi(), ::core::mem::transmute(dwexpectedfilesize), lpszfileextension.into_param().abi(), lpszfilename.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateUrlCacheGroup(dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> i64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUrlCacheGroup(dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> i64;
        }
        ::core::mem::transmute(CreateUrlCacheGroup(::core::mem::transmute(dwflags), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DIALENG_OperationComplete: u32 = 65536u32;
pub const DIALENG_RedialAttempt: u32 = 65537u32;
pub const DIALENG_RedialWait: u32 = 65538u32;
pub const DLG_FLAGS_INSECURE_FALLBACK: u32 = 4194304u32;
pub const DLG_FLAGS_INVALID_CA: u32 = 16777216u32;
pub const DLG_FLAGS_SEC_CERT_CN_INVALID: u32 = 33554432u32;
pub const DLG_FLAGS_SEC_CERT_DATE_INVALID: u32 = 67108864u32;
pub const DLG_FLAGS_SEC_CERT_REV_FAILED: u32 = 8388608u32;
pub const DLG_FLAGS_WEAK_SIGNATURE: u32 = 2097152u32;
pub const DOWNLOAD_CACHE_ENTRY: u32 = 1024u32;
pub const DUO_PROTOCOL_FLAG_SPDY3: u32 = 1u32;
pub const DUO_PROTOCOL_MASK: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteIE3Cache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, hinst: Param1, lpszcmd: Param2, ncmdshow: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteIE3Cache(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: super::super::Foundation::PSTR, ncmdshow: i32) -> u32;
        }
        ::core::mem::transmute(DeleteIE3Cache(hwnd.into_param().abi(), hinst.into_param().abi(), lpszcmd.into_param().abi(), ::core::mem::transmute(ncmdshow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheContainerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(name: Param0, dwoptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheContainerA(name: super::super::Foundation::PSTR, dwoptions: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheContainerA(name.into_param().abi(), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheContainerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(name: Param0, dwoptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheContainerW(name: super::super::Foundation::PWSTR, dwoptions: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheContainerW(name.into_param().abi(), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheEntry(lpszurlname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheEntry(lpszurlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheEntryA(lpszurlname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheEntryA(lpszurlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheEntryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheEntryW(lpszurlname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheEntryW(lpszurlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUrlCacheGroup(groupid: i64, dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUrlCacheGroup(groupid: i64, dwflags: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUrlCacheGroup(::core::mem::transmute(groupid), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteWpadCacheForNetworks(param0: WPAD_CACHE_DELETE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteWpadCacheForNetworks(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DetectAutoProxyUrl(pszautoproxyurl: super::super::Foundation::PSTR, cchautoproxyurl: u32, dwdetectflags: PROXY_AUTO_DETECT_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DetectAutoProxyUrl(pszautoproxyurl: super::super::Foundation::PSTR, cchautoproxyurl: u32, dwdetectflags: PROXY_AUTO_DETECT_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DetectAutoProxyUrl(::core::mem::transmute(pszautoproxyurl), ::core::mem::transmute(cchautoproxyurl), ::core::mem::transmute(dwdetectflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DoConnectoidsExist() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DoConnectoidsExist() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DoConnectoidsExist())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const EDITED_CACHE_ENTRY: u32 = 8u32;
pub const ERROR_FTP_DROPPED: u32 = 12111u32;
pub const ERROR_FTP_NO_PASSIVE_MODE: u32 = 12112u32;
pub const ERROR_FTP_TRANSFER_IN_PROGRESS: u32 = 12110u32;
pub const ERROR_GOPHER_ATTRIBUTE_NOT_FOUND: u32 = 12137u32;
pub const ERROR_GOPHER_DATA_ERROR: u32 = 12132u32;
pub const ERROR_GOPHER_END_OF_DATA: u32 = 12133u32;
pub const ERROR_GOPHER_INCORRECT_LOCATOR_TYPE: u32 = 12135u32;
pub const ERROR_GOPHER_INVALID_LOCATOR: u32 = 12134u32;
pub const ERROR_GOPHER_NOT_FILE: u32 = 12131u32;
pub const ERROR_GOPHER_NOT_GOPHER_PLUS: u32 = 12136u32;
pub const ERROR_GOPHER_PROTOCOL_ERROR: u32 = 12130u32;
pub const ERROR_GOPHER_UNKNOWN_LOCATOR: u32 = 12138u32;
pub const ERROR_HTTP_COOKIE_DECLINED: u32 = 12162u32;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION: u32 = 12161u32;
pub const ERROR_HTTP_COOKIE_NEEDS_CONFIRMATION_EX: u32 = 12907u32;
pub const ERROR_HTTP_DOWNLEVEL_SERVER: u32 = 12151u32;
pub const ERROR_HTTP_HEADER_ALREADY_EXISTS: u32 = 12155u32;
pub const ERROR_HTTP_HEADER_NOT_FOUND: u32 = 12150u32;
pub const ERROR_HTTP_HSTS_REDIRECT_REQUIRED: u32 = 12060u32;
pub const ERROR_HTTP_INVALID_HEADER: u32 = 12153u32;
pub const ERROR_HTTP_INVALID_QUERY_REQUEST: u32 = 12154u32;
pub const ERROR_HTTP_INVALID_SERVER_RESPONSE: u32 = 12152u32;
pub const ERROR_HTTP_NOT_REDIRECTED: u32 = 12160u32;
pub const ERROR_HTTP_PUSH_ENABLE_FAILED: u32 = 12149u32;
pub const ERROR_HTTP_PUSH_RETRY_NOT_SUPPORTED: u32 = 12148u32;
pub const ERROR_HTTP_PUSH_STATUS_CODE_NOT_SUPPORTED: u32 = 12147u32;
pub const ERROR_HTTP_REDIRECT_FAILED: u32 = 12156u32;
pub const ERROR_HTTP_REDIRECT_NEEDS_CONFIRMATION: u32 = 12168u32;
pub const ERROR_INTERNET_ASYNC_THREAD_FAILED: u32 = 12047u32;
pub const ERROR_INTERNET_BAD_AUTO_PROXY_SCRIPT: u32 = 12166u32;
pub const ERROR_INTERNET_BAD_OPTION_LENGTH: u32 = 12010u32;
pub const ERROR_INTERNET_BAD_REGISTRY_PARAMETER: u32 = 12022u32;
pub const ERROR_INTERNET_CACHE_SUCCESS: u32 = 12906u32;
pub const ERROR_INTERNET_CANNOT_CONNECT: u32 = 12029u32;
pub const ERROR_INTERNET_CHG_POST_IS_NON_SECURE: u32 = 12042u32;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED: u32 = 12044u32;
pub const ERROR_INTERNET_CLIENT_AUTH_CERT_NEEDED_PROXY: u32 = 12187u32;
pub const ERROR_INTERNET_CLIENT_AUTH_NOT_SETUP: u32 = 12046u32;
pub const ERROR_INTERNET_CONNECTION_ABORTED: u32 = 12030u32;
pub const ERROR_INTERNET_CONNECTION_AVAILABLE: u32 = 12902u32;
pub const ERROR_INTERNET_CONNECTION_RESET: u32 = 12031u32;
pub const ERROR_INTERNET_DECODING_FAILED: u32 = 12175u32;
pub const ERROR_INTERNET_DIALOG_PENDING: u32 = 12049u32;
pub const ERROR_INTERNET_DISALLOW_INPRIVATE: u32 = 12189u32;
pub const ERROR_INTERNET_DISCONNECTED: u32 = 12163u32;
pub const ERROR_INTERNET_EXTENDED_ERROR: u32 = 12003u32;
pub const ERROR_INTERNET_FAILED_DUETOSECURITYCHECK: u32 = 12171u32;
pub const ERROR_INTERNET_FEATURE_DISABLED: u32 = 12192u32;
pub const ERROR_INTERNET_FORCE_RETRY: u32 = 12032u32;
pub const ERROR_INTERNET_FORTEZZA_LOGIN_NEEDED: u32 = 12054u32;
pub const ERROR_INTERNET_GLOBAL_CALLBACK_FAILED: u32 = 12191u32;
pub const ERROR_INTERNET_HANDLE_EXISTS: u32 = 12036u32;
pub const ERROR_INTERNET_HTTPS_HTTP_SUBMIT_REDIR: u32 = 12052u32;
pub const ERROR_INTERNET_HTTPS_TO_HTTP_ON_REDIR: u32 = 12040u32;
pub const ERROR_INTERNET_HTTP_PROTOCOL_MISMATCH: u32 = 12190u32;
pub const ERROR_INTERNET_HTTP_TO_HTTPS_ON_REDIR: u32 = 12039u32;
pub const ERROR_INTERNET_INCORRECT_FORMAT: u32 = 12027u32;
pub const ERROR_INTERNET_INCORRECT_HANDLE_STATE: u32 = 12019u32;
pub const ERROR_INTERNET_INCORRECT_HANDLE_TYPE: u32 = 12018u32;
pub const ERROR_INTERNET_INCORRECT_PASSWORD: u32 = 12014u32;
pub const ERROR_INTERNET_INCORRECT_USER_NAME: u32 = 12013u32;
pub const ERROR_INTERNET_INSECURE_FALLBACK_REQUIRED: u32 = 12059u32;
pub const ERROR_INTERNET_INSERT_CDROM: u32 = 12053u32;
pub const ERROR_INTERNET_INTERNAL_ERROR: u32 = 12004u32;
pub const ERROR_INTERNET_INTERNAL_SOCKET_ERROR: u32 = 12901u32;
pub const ERROR_INTERNET_INVALID_CA: u32 = 12045u32;
pub const ERROR_INTERNET_INVALID_OPERATION: u32 = 12016u32;
pub const ERROR_INTERNET_INVALID_OPTION: u32 = 12009u32;
pub const ERROR_INTERNET_INVALID_PROXY_REQUEST: u32 = 12033u32;
pub const ERROR_INTERNET_INVALID_URL: u32 = 12005u32;
pub const ERROR_INTERNET_ITEM_NOT_FOUND: u32 = 12028u32;
pub const ERROR_INTERNET_LOGIN_FAILURE: u32 = 12015u32;
pub const ERROR_INTERNET_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 12174u32;
pub const ERROR_INTERNET_MIXED_SECURITY: u32 = 12041u32;
pub const ERROR_INTERNET_NAME_NOT_RESOLVED: u32 = 12007u32;
pub const ERROR_INTERNET_NEED_MSN_SSPI_PKG: u32 = 12173u32;
pub const ERROR_INTERNET_NEED_UI: u32 = 12034u32;
pub const ERROR_INTERNET_NOT_INITIALIZED: u32 = 12172u32;
pub const ERROR_INTERNET_NOT_PROXY_REQUEST: u32 = 12020u32;
pub const ERROR_INTERNET_NO_CALLBACK: u32 = 12025u32;
pub const ERROR_INTERNET_NO_CM_CONNECTION: u32 = 12080u32;
pub const ERROR_INTERNET_NO_CONTEXT: u32 = 12024u32;
pub const ERROR_INTERNET_NO_DIRECT_ACCESS: u32 = 12023u32;
pub const ERROR_INTERNET_NO_KNOWN_SERVERS: u32 = 12903u32;
pub const ERROR_INTERNET_NO_NEW_CONTAINERS: u32 = 12051u32;
pub const ERROR_INTERNET_NO_PING_SUPPORT: u32 = 12905u32;
pub const ERROR_INTERNET_OFFLINE: u32 = 12163u32;
pub const ERROR_INTERNET_OPERATION_CANCELLED: u32 = 12017u32;
pub const ERROR_INTERNET_OPTION_NOT_SETTABLE: u32 = 12011u32;
pub const ERROR_INTERNET_OUT_OF_HANDLES: u32 = 12001u32;
pub const ERROR_INTERNET_PING_FAILED: u32 = 12904u32;
pub const ERROR_INTERNET_POST_IS_NON_SECURE: u32 = 12043u32;
pub const ERROR_INTERNET_PROTOCOL_NOT_FOUND: u32 = 12008u32;
pub const ERROR_INTERNET_PROXY_ALERT: u32 = 12061u32;
pub const ERROR_INTERNET_PROXY_SERVER_UNREACHABLE: u32 = 12165u32;
pub const ERROR_INTERNET_REDIRECT_SCHEME_CHANGE: u32 = 12048u32;
pub const ERROR_INTERNET_REGISTRY_VALUE_NOT_FOUND: u32 = 12021u32;
pub const ERROR_INTERNET_REQUEST_PENDING: u32 = 12026u32;
pub const ERROR_INTERNET_RETRY_DIALOG: u32 = 12050u32;
pub const ERROR_INTERNET_SECURE_FAILURE_PROXY: u32 = 12188u32;
pub const ERROR_INTERNET_SECURITY_CHANNEL_ERROR: u32 = 12157u32;
pub const ERROR_INTERNET_SEC_CERT_CN_INVALID: u32 = 12038u32;
pub const ERROR_INTERNET_SEC_CERT_DATE_INVALID: u32 = 12037u32;
pub const ERROR_INTERNET_SEC_CERT_ERRORS: u32 = 12055u32;
pub const ERROR_INTERNET_SEC_CERT_NO_REV: u32 = 12056u32;
pub const ERROR_INTERNET_SEC_CERT_REVOKED: u32 = 12170u32;
pub const ERROR_INTERNET_SEC_CERT_REV_FAILED: u32 = 12057u32;
pub const ERROR_INTERNET_SEC_CERT_WEAK_SIGNATURE: u32 = 12062u32;
pub const ERROR_INTERNET_SEC_INVALID_CERT: u32 = 12169u32;
pub const ERROR_INTERNET_SERVER_UNREACHABLE: u32 = 12164u32;
pub const ERROR_INTERNET_SHUTDOWN: u32 = 12012u32;
pub const ERROR_INTERNET_SOURCE_PORT_IN_USE: u32 = 12058u32;
pub const ERROR_INTERNET_TCPIP_NOT_INSTALLED: u32 = 12159u32;
pub const ERROR_INTERNET_TIMEOUT: u32 = 12002u32;
pub const ERROR_INTERNET_UNABLE_TO_CACHE_FILE: u32 = 12158u32;
pub const ERROR_INTERNET_UNABLE_TO_DOWNLOAD_SCRIPT: u32 = 12167u32;
pub const ERROR_INTERNET_UNRECOGNIZED_SCHEME: u32 = 12006u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExportCookieFileA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szfilename: Param0, fappend: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExportCookieFileA(szfilename: super::super::Foundation::PSTR, fappend: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ExportCookieFileA(szfilename.into_param().abi(), fappend.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExportCookieFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(szfilename: Param0, fappend: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExportCookieFileW(szfilename: super::super::Foundation::PWSTR, fappend: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ExportCookieFileW(szfilename.into_param().abi(), fappend.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FLAGS_ERROR_UI_FILTER_FOR_ERRORS: u32 = 1u32;
pub const FLAGS_ERROR_UI_FLAGS_CHANGE_OPTIONS: u32 = 2u32;
pub const FLAGS_ERROR_UI_FLAGS_GENERATE_DATA: u32 = 4u32;
pub const FLAGS_ERROR_UI_FLAGS_NO_UI: u32 = 8u32;
pub const FLAGS_ERROR_UI_SERIALIZE_DIALOGS: u32 = 16u32;
pub const FLAGS_ERROR_UI_SHOW_IDN_HOSTNAME: u32 = 32u32;
pub const FLAG_ICC_FORCE_CONNECTION: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FORTCMD(pub i32);
pub const FORTCMD_LOGON: FORTCMD = FORTCMD(1i32);
pub const FORTCMD_LOGOFF: FORTCMD = FORTCMD(2i32);
pub const FORTCMD_CHG_PERSONALITY: FORTCMD = FORTCMD(3i32);
impl ::core::convert::From<i32> for FORTCMD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FORTCMD {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FORTSTAT(pub i32);
pub const FORTSTAT_INSTALLED: FORTSTAT = FORTSTAT(1i32);
pub const FORTSTAT_LOGGEDON: FORTSTAT = FORTSTAT(2i32);
impl ::core::convert::From<i32> for FORTSTAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FORTSTAT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct FTP_FLAGS(pub u32);
pub const FTP_TRANSFER_TYPE_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
pub const FTP_TRANSFER_TYPE_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
pub const FTP_TRANSFER_TYPE_UNKNOWN: FTP_FLAGS = FTP_FLAGS(0u32);
pub const INTERNET_FLAG_TRANSFER_ASCII: FTP_FLAGS = FTP_FLAGS(1u32);
pub const INTERNET_FLAG_TRANSFER_BINARY: FTP_FLAGS = FTP_FLAGS(2u32);
impl ::core::convert::From<u32> for FTP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for FTP_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for FTP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for FTP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for FTP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for FTP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for FTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindCloseUrlCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindCloseUrlCache(henumhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindCloseUrlCache(henumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheContainerA(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheContainerA(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheContainerA(::core::mem::transmute(pdwmodified), ::core::mem::transmute(lpcontainerinfo), ::core::mem::transmute(lpcbcontainerinfo), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheContainerW(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheContainerW(pdwmodified: *mut u32, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32, dwoptions: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheContainerW(::core::mem::transmute(pdwmodified), ::core::mem::transmute(lpcontainerinfo), ::core::mem::transmute(lpcbcontainerinfo), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlsearchpattern: Param0, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheEntryA(lpszurlsearchpattern: super::super::Foundation::PSTR, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheEntryA(lpszurlsearchpattern.into_param().abi(), ::core::mem::transmute(lpfirstcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlsearchpattern: Param0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheEntryExA(lpszurlsearchpattern: super::super::Foundation::PSTR, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheEntryExA(
            lpszurlsearchpattern.into_param().abi(),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwfilter),
            ::core::mem::transmute(groupid),
            ::core::mem::transmute(lpfirstcacheentryinfo),
            ::core::mem::transmute(lpcbcacheentryinfo),
            ::core::mem::transmute(lpgroupattributes),
            ::core::mem::transmute(lpcbgroupattributes),
            ::core::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlsearchpattern: Param0, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheEntryExW(lpszurlsearchpattern: super::super::Foundation::PWSTR, dwflags: u32, dwfilter: u32, groupid: i64, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheEntryExW(
            lpszurlsearchpattern.into_param().abi(),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwfilter),
            ::core::mem::transmute(groupid),
            ::core::mem::transmute(lpfirstcacheentryinfo),
            ::core::mem::transmute(lpcbcacheentryinfo),
            ::core::mem::transmute(lpgroupattributes),
            ::core::mem::transmute(lpcbgroupattributes),
            ::core::mem::transmute(lpreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheEntryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlsearchpattern: Param0, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheEntryW(lpszurlsearchpattern: super::super::Foundation::PWSTR, lpfirstcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheEntryW(lpszurlsearchpattern.into_param().abi(), ::core::mem::transmute(lpfirstcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: *mut ::core::ffi::c_void, dwsearchcondition: u32, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindFirstUrlCacheGroup(dwflags: u32, dwfilter: u32, lpsearchcondition: *mut ::core::ffi::c_void, dwsearchcondition: u32, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(FindFirstUrlCacheGroup(::core::mem::transmute(dwflags), ::core::mem::transmute(dwfilter), ::core::mem::transmute(lpsearchcondition), ::core::mem::transmute(dwsearchcondition), ::core::mem::transmute(lpgroupid), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheContainerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheContainerA(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOA, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheContainerA(henumhandle.into_param().abi(), ::core::mem::transmute(lpcontainerinfo), ::core::mem::transmute(lpcbcontainerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheContainerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheContainerW(henumhandle: super::super::Foundation::HANDLE, lpcontainerinfo: *mut INTERNET_CACHE_CONTAINER_INFOW, lpcbcontainerinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheContainerW(henumhandle.into_param().abi(), ::core::mem::transmute(lpcontainerinfo), ::core::mem::transmute(lpcbcontainerinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheEntryA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheEntryA(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheEntryExA(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheEntryExA(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), ::core::mem::transmute(lpgroupattributes), ::core::mem::transmute(lpcbgroupattributes), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheEntryExW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpgroupattributes: *mut ::core::ffi::c_void, lpcbgroupattributes: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheEntryExW(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), ::core::mem::transmute(lpgroupattributes), ::core::mem::transmute(lpcbgroupattributes), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheEntryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(henumhandle: Param0, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheEntryW(henumhandle: super::super::Foundation::HANDLE, lpnextcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheEntryW(henumhandle.into_param().abi(), ::core::mem::transmute(lpnextcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindNextUrlCacheGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfind: Param0, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindNextUrlCacheGroup(hfind: super::super::Foundation::HANDLE, lpgroupid: *mut i64, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FindNextUrlCacheGroup(hfind.into_param().abi(), ::core::mem::transmute(lpgroupid), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindP3PPolicySymbol<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszsymbol: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindP3PPolicySymbol(pszsymbol: super::super::Foundation::PSTR) -> i32;
        }
        ::core::mem::transmute(FindP3PPolicySymbol(pszsymbol.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUrlCacheSpaceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszcachepath: Param0, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeUrlCacheSpaceA(lpszcachepath: super::super::Foundation::PSTR, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeUrlCacheSpaceA(lpszcachepath.into_param().abi(), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUrlCacheSpaceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszcachepath: Param0, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeUrlCacheSpaceW(lpszcachepath: super::super::Foundation::PWSTR, dwsize: u32, dwfilter: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeUrlCacheSpaceW(lpszcachepath.into_param().abi(), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCommandA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, fexpectresponse: Param1, dwflags: FTP_FLAGS, lpszcommand: Param3, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpCommandA(hconnect: *const ::core::ffi::c_void, fexpectresponse: super::super::Foundation::BOOL, dwflags: FTP_FLAGS, lpszcommand: super::super::Foundation::PSTR, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpCommandA(::core::mem::transmute(hconnect), fexpectresponse.into_param().abi(), ::core::mem::transmute(dwflags), lpszcommand.into_param().abi(), ::core::mem::transmute(dwcontext), ::core::mem::transmute(phftpcommand)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCommandW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, fexpectresponse: Param1, dwflags: FTP_FLAGS, lpszcommand: Param3, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpCommandW(hconnect: *const ::core::ffi::c_void, fexpectresponse: super::super::Foundation::BOOL, dwflags: FTP_FLAGS, lpszcommand: super::super::Foundation::PWSTR, dwcontext: usize, phftpcommand: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpCommandW(::core::mem::transmute(hconnect), fexpectresponse.into_param().abi(), ::core::mem::transmute(dwflags), lpszcommand.into_param().abi(), ::core::mem::transmute(dwcontext), ::core::mem::transmute(phftpcommand)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCreateDirectoryA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpCreateDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpCreateDirectoryA(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpCreateDirectoryW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpCreateDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpCreateDirectoryW(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpDeleteFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpDeleteFileA(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpDeleteFileA(::core::mem::transmute(hconnect), lpszfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpDeleteFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszfilename: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpDeleteFileW(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpDeleteFileW(::core::mem::transmute(hconnect), lpszfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn FtpFindFirstFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszsearchfile: Param1, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpFindFirstFileA(hconnect: *const ::core::ffi::c_void, lpszsearchfile: super::super::Foundation::PSTR, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FtpFindFirstFileA(::core::mem::transmute(hconnect), lpszsearchfile.into_param().abi(), ::core::mem::transmute(lpfindfiledata), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn FtpFindFirstFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszsearchfile: Param1, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpFindFirstFileW(hconnect: *const ::core::ffi::c_void, lpszsearchfile: super::super::Foundation::PWSTR, lpfindfiledata: *mut super::super::Storage::FileSystem::WIN32_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FtpFindFirstFileW(::core::mem::transmute(hconnect), lpszsearchfile.into_param().abi(), ::core::mem::transmute(lpfindfiledata), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpGetCurrentDirectoryA(::core::mem::transmute(hconnect), ::core::mem::transmute(lpszcurrentdirectory), ::core::mem::transmute(lpdwcurrentdirectory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PWSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszcurrentdirectory: super::super::Foundation::PWSTR, lpdwcurrentdirectory: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpGetCurrentDirectoryW(::core::mem::transmute(hconnect), ::core::mem::transmute(lpszcurrentdirectory), ::core::mem::transmute(lpdwcurrentdirectory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hconnect: *const ::core::ffi::c_void, lpszremotefile: Param1, lpsznewfile: Param2, ffailifexists: Param3, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetFileA(hconnect: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PSTR, lpsznewfile: super::super::Foundation::PSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpGetFileA(::core::mem::transmute(hconnect), lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), ::core::mem::transmute(dwflagsandattributes), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hftpsession: *const ::core::ffi::c_void, lpszremotefile: Param1, lpsznewfile: Param2, ffailifexists: Param3, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetFileEx(hftpsession: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PSTR, lpsznewfile: super::super::Foundation::PWSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpGetFileEx(::core::mem::transmute(hftpsession), lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), ::core::mem::transmute(dwflagsandattributes), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FtpGetFileSize(hfile: *const ::core::ffi::c_void, lpdwfilesizehigh: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetFileSize(hfile: *const ::core::ffi::c_void, lpdwfilesizehigh: *mut u32) -> u32;
        }
        ::core::mem::transmute(FtpGetFileSize(::core::mem::transmute(hfile), ::core::mem::transmute(lpdwfilesizehigh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpGetFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hconnect: *const ::core::ffi::c_void, lpszremotefile: Param1, lpsznewfile: Param2, ffailifexists: Param3, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpGetFileW(hconnect: *const ::core::ffi::c_void, lpszremotefile: super::super::Foundation::PWSTR, lpsznewfile: super::super::Foundation::PWSTR, ffailifexists: super::super::Foundation::BOOL, dwflagsandattributes: u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpGetFileW(::core::mem::transmute(hconnect), lpszremotefile.into_param().abi(), lpsznewfile.into_param().abi(), ffailifexists.into_param().abi(), ::core::mem::transmute(dwflagsandattributes), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpOpenFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszfilename: Param1, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpOpenFileA(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PSTR, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FtpOpenFileA(::core::mem::transmute(hconnect), lpszfilename.into_param().abi(), ::core::mem::transmute(dwaccess), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpOpenFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszfilename: Param1, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpOpenFileW(hconnect: *const ::core::ffi::c_void, lpszfilename: super::super::Foundation::PWSTR, dwaccess: u32, dwflags: FTP_FLAGS, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FtpOpenFileW(::core::mem::transmute(hconnect), lpszfilename.into_param().abi(), ::core::mem::transmute(dwaccess), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocalfile: Param1, lpsznewremotefile: Param2, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpPutFileA(hconnect: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PSTR, lpsznewremotefile: super::super::Foundation::PSTR, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpPutFileA(::core::mem::transmute(hconnect), lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hftpsession: *const ::core::ffi::c_void, lpszlocalfile: Param1, lpsznewremotefile: Param2, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpPutFileEx(hftpsession: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PWSTR, lpsznewremotefile: super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpPutFileEx(::core::mem::transmute(hftpsession), lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpPutFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocalfile: Param1, lpsznewremotefile: Param2, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpPutFileW(hconnect: *const ::core::ffi::c_void, lpszlocalfile: super::super::Foundation::PWSTR, lpsznewremotefile: super::super::Foundation::PWSTR, dwflags: FTP_FLAGS, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpPutFileW(::core::mem::transmute(hconnect), lpszlocalfile.into_param().abi(), lpsznewremotefile.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRemoveDirectoryA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpRemoveDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpRemoveDirectoryA(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRemoveDirectoryW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpRemoveDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpRemoveDirectoryW(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRenameFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszexisting: Param1, lpsznew: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpRenameFileA(hconnect: *const ::core::ffi::c_void, lpszexisting: super::super::Foundation::PSTR, lpsznew: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpRenameFileA(::core::mem::transmute(hconnect), lpszexisting.into_param().abi(), lpsznew.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpRenameFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszexisting: Param1, lpsznew: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpRenameFileW(hconnect: *const ::core::ffi::c_void, lpszexisting: super::super::Foundation::PWSTR, lpsznew: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpRenameFileW(::core::mem::transmute(hconnect), lpszexisting.into_param().abi(), lpsznew.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpSetCurrentDirectoryA(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpSetCurrentDirectoryA(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FtpSetCurrentDirectoryW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszdirectory: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FtpSetCurrentDirectoryW(hconnect: *const ::core::ffi::c_void, lpszdirectory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FtpSetCurrentDirectoryW(::core::mem::transmute(hconnect), lpszdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    pub ShortAbstract: *mut i8,
    pub AbstractFile: *mut i8,
}
impl GOPHER_ABSTRACT_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_ABSTRACT_ATTRIBUTE_TYPE").field("ShortAbstract", &self.ShortAbstract).field("AbstractFile", &self.AbstractFile).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ShortAbstract == other.ShortAbstract && self.AbstractFile == other.AbstractFile
    }
}
impl ::core::cmp::Eq for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_ABSTRACT_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_ADMIN_ATTRIBUTE_TYPE {
    pub Comment: *mut i8,
    pub EmailAddress: *mut i8,
}
impl GOPHER_ADMIN_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_ADMIN_ATTRIBUTE_TYPE").field("Comment", &self.Comment).field("EmailAddress", &self.EmailAddress).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment && self.EmailAddress == other.EmailAddress
    }
}
impl ::core::cmp::Eq for GOPHER_ADMIN_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_ADMIN_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_ASK_ATTRIBUTE_TYPE {
    pub QuestionType: *mut i8,
    pub QuestionText: *mut i8,
}
impl GOPHER_ASK_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_ASK_ATTRIBUTE_TYPE").field("QuestionType", &self.QuestionType).field("QuestionText", &self.QuestionText).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_ASK_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.QuestionType == other.QuestionType && self.QuestionText == other.QuestionText
    }
}
impl ::core::cmp::Eq for GOPHER_ASK_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_ASK_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type GOPHER_ATTRIBUTE_ENUMERATOR = unsafe extern "system" fn(lpattributeinfo: *const GOPHER_ATTRIBUTE_TYPE, dwerror: u32) -> super::super::Foundation::BOOL;
pub const GOPHER_ATTRIBUTE_ID_ABSTRACT: u32 = 2882325526u32;
pub const GOPHER_ATTRIBUTE_ID_ADMIN: u32 = 2882325514u32;
pub const GOPHER_ATTRIBUTE_ID_ALL: u32 = 2882325513u32;
pub const GOPHER_ATTRIBUTE_ID_BASE: u32 = 2882325504u32;
pub const GOPHER_ATTRIBUTE_ID_GEOG: u32 = 2882325522u32;
pub const GOPHER_ATTRIBUTE_ID_LOCATION: u32 = 2882325521u32;
pub const GOPHER_ATTRIBUTE_ID_MOD_DATE: u32 = 2882325515u32;
pub const GOPHER_ATTRIBUTE_ID_ORG: u32 = 2882325520u32;
pub const GOPHER_ATTRIBUTE_ID_PROVIDER: u32 = 2882325524u32;
pub const GOPHER_ATTRIBUTE_ID_RANGE: u32 = 2882325518u32;
pub const GOPHER_ATTRIBUTE_ID_SCORE: u32 = 2882325517u32;
pub const GOPHER_ATTRIBUTE_ID_SITE: u32 = 2882325519u32;
pub const GOPHER_ATTRIBUTE_ID_TIMEZONE: u32 = 2882325523u32;
pub const GOPHER_ATTRIBUTE_ID_TREEWALK: u32 = 2882325528u32;
pub const GOPHER_ATTRIBUTE_ID_TTL: u32 = 2882325516u32;
pub const GOPHER_ATTRIBUTE_ID_UNKNOWN: u32 = 2882325529u32;
pub const GOPHER_ATTRIBUTE_ID_VERSION: u32 = 2882325525u32;
pub const GOPHER_ATTRIBUTE_ID_VIEW: u32 = 2882325527u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_ATTRIBUTE_TYPE {
    pub CategoryId: u32,
    pub AttributeId: u32,
    pub AttributeType: GOPHER_ATTRIBUTE_TYPE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_ATTRIBUTE_TYPE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union GOPHER_ATTRIBUTE_TYPE_0 {
    pub Admin: GOPHER_ADMIN_ATTRIBUTE_TYPE,
    pub ModDate: GOPHER_MOD_DATE_ATTRIBUTE_TYPE,
    pub Ttl: GOPHER_TTL_ATTRIBUTE_TYPE,
    pub Score: GOPHER_SCORE_ATTRIBUTE_TYPE,
    pub ScoreRange: GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE,
    pub Site: GOPHER_SITE_ATTRIBUTE_TYPE,
    pub Organization: GOPHER_ORGANIZATION_ATTRIBUTE_TYPE,
    pub Location: GOPHER_LOCATION_ATTRIBUTE_TYPE,
    pub GeographicalLocation: GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE,
    pub TimeZone: GOPHER_TIMEZONE_ATTRIBUTE_TYPE,
    pub Provider: GOPHER_PROVIDER_ATTRIBUTE_TYPE,
    pub Version: GOPHER_VERSION_ATTRIBUTE_TYPE,
    pub Abstract: GOPHER_ABSTRACT_ATTRIBUTE_TYPE,
    pub View: GOPHER_VIEW_ATTRIBUTE_TYPE,
    pub Veronica: GOPHER_VERONICA_ATTRIBUTE_TYPE,
    pub Ask: GOPHER_ASK_ATTRIBUTE_TYPE,
    pub Unknown: GOPHER_UNKNOWN_ATTRIBUTE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_ATTRIBUTE_TYPE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_ATTRIBUTE_TYPE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_ATTRIBUTE_TYPE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_ATTRIBUTE_TYPE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_ATTRIBUTE_TYPE_0 {
    type Abi = Self;
}
pub const GOPHER_CATEGORY_ID_ABSTRACT: u32 = 2882325509u32;
pub const GOPHER_CATEGORY_ID_ADMIN: u32 = 2882325507u32;
pub const GOPHER_CATEGORY_ID_ALL: u32 = 2882325505u32;
pub const GOPHER_CATEGORY_ID_ASK: u32 = 2882325511u32;
pub const GOPHER_CATEGORY_ID_INFO: u32 = 2882325506u32;
pub const GOPHER_CATEGORY_ID_UNKNOWN: u32 = 2882325512u32;
pub const GOPHER_CATEGORY_ID_VERONICA: u32 = 2882325510u32;
pub const GOPHER_CATEGORY_ID_VIEWS: u32 = 2882325508u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_FIND_DATAA {
    pub DisplayString: [super::super::Foundation::CHAR; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [super::super::Foundation::CHAR; 654],
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_FIND_DATAA").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAA {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_FIND_DATAA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_FIND_DATAW {
    pub DisplayString: [u16; 129],
    pub GopherType: GOPHER_TYPE,
    pub SizeLow: u32,
    pub SizeHigh: u32,
    pub LastModificationTime: super::super::Foundation::FILETIME,
    pub Locator: [u16; 654],
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_FIND_DATAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_FIND_DATAW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_FIND_DATAW").field("DisplayString", &self.DisplayString).field("GopherType", &self.GopherType).field("SizeLow", &self.SizeLow).field("SizeHigh", &self.SizeHigh).field("LastModificationTime", &self.LastModificationTime).field("Locator", &self.Locator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_FIND_DATAW {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayString == other.DisplayString && self.GopherType == other.GopherType && self.SizeLow == other.SizeLow && self.SizeHigh == other.SizeHigh && self.LastModificationTime == other.LastModificationTime && self.Locator == other.Locator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_FIND_DATAW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_FIND_DATAW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    pub DegreesNorth: i32,
    pub MinutesNorth: i32,
    pub SecondsNorth: i32,
    pub DegreesEast: i32,
    pub MinutesEast: i32,
    pub SecondsEast: i32,
}
impl GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE")
            .field("DegreesNorth", &self.DegreesNorth)
            .field("MinutesNorth", &self.MinutesNorth)
            .field("SecondsNorth", &self.SecondsNorth)
            .field("DegreesEast", &self.DegreesEast)
            .field("MinutesEast", &self.MinutesEast)
            .field("SecondsEast", &self.SecondsEast)
            .finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DegreesNorth == other.DegreesNorth && self.MinutesNorth == other.MinutesNorth && self.SecondsNorth == other.SecondsNorth && self.DegreesEast == other.DegreesEast && self.MinutesEast == other.MinutesEast && self.SecondsEast == other.SecondsEast
    }
}
impl ::core::cmp::Eq for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_GEOGRAPHICAL_LOCATION_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_LOCATION_ATTRIBUTE_TYPE {
    pub Location: *mut i8,
}
impl GOPHER_LOCATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_LOCATION_ATTRIBUTE_TYPE").field("Location", &self.Location).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
    }
}
impl ::core::cmp::Eq for GOPHER_LOCATION_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_LOCATION_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    pub DateAndTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_MOD_DATE_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_MOD_DATE_ATTRIBUTE_TYPE").field("DateAndTime", &self.DateAndTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DateAndTime == other.DateAndTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_MOD_DATE_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    pub Organization: *mut i8,
}
impl GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_ORGANIZATION_ATTRIBUTE_TYPE").field("Organization", &self.Organization).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Organization == other.Organization
    }
}
impl ::core::cmp::Eq for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_ORGANIZATION_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    pub Provider: *mut i8,
}
impl GOPHER_PROVIDER_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_PROVIDER_ATTRIBUTE_TYPE").field("Provider", &self.Provider).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Provider == other.Provider
    }
}
impl ::core::cmp::Eq for GOPHER_PROVIDER_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_PROVIDER_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_SCORE_ATTRIBUTE_TYPE {
    pub Score: i32,
}
impl GOPHER_SCORE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_SCORE_ATTRIBUTE_TYPE").field("Score", &self.Score).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Score == other.Score
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_SCORE_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    pub LowerBound: i32,
    pub UpperBound: i32,
}
impl GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE").field("LowerBound", &self.LowerBound).field("UpperBound", &self.UpperBound).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.LowerBound == other.LowerBound && self.UpperBound == other.UpperBound
    }
}
impl ::core::cmp::Eq for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_SCORE_RANGE_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_SITE_ATTRIBUTE_TYPE {
    pub Site: *mut i8,
}
impl GOPHER_SITE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_SITE_ATTRIBUTE_TYPE").field("Site", &self.Site).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_SITE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Site == other.Site
    }
}
impl ::core::cmp::Eq for GOPHER_SITE_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_SITE_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    pub Zone: i32,
}
impl GOPHER_TIMEZONE_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_TIMEZONE_ATTRIBUTE_TYPE").field("Zone", &self.Zone).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Zone == other.Zone
    }
}
impl ::core::cmp::Eq for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_TIMEZONE_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_TTL_ATTRIBUTE_TYPE {
    pub Ttl: u32,
}
impl GOPHER_TTL_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_TTL_ATTRIBUTE_TYPE").field("Ttl", &self.Ttl).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_TTL_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Ttl == other.Ttl
    }
}
impl ::core::cmp::Eq for GOPHER_TTL_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_TTL_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GOPHER_TYPE(pub u32);
pub const GOPHER_TYPE_ASK: GOPHER_TYPE = GOPHER_TYPE(1073741824u32);
pub const GOPHER_TYPE_BINARY: GOPHER_TYPE = GOPHER_TYPE(512u32);
pub const GOPHER_TYPE_BITMAP: GOPHER_TYPE = GOPHER_TYPE(16384u32);
pub const GOPHER_TYPE_CALENDAR: GOPHER_TYPE = GOPHER_TYPE(524288u32);
pub const GOPHER_TYPE_CSO: GOPHER_TYPE = GOPHER_TYPE(4u32);
pub const GOPHER_TYPE_DIRECTORY: GOPHER_TYPE = GOPHER_TYPE(2u32);
pub const GOPHER_TYPE_DOS_ARCHIVE: GOPHER_TYPE = GOPHER_TYPE(32u32);
pub const GOPHER_TYPE_ERROR: GOPHER_TYPE = GOPHER_TYPE(8u32);
pub const GOPHER_TYPE_GIF: GOPHER_TYPE = GOPHER_TYPE(4096u32);
pub const GOPHER_TYPE_GOPHER_PLUS: GOPHER_TYPE = GOPHER_TYPE(2147483648u32);
pub const GOPHER_TYPE_HTML: GOPHER_TYPE = GOPHER_TYPE(131072u32);
pub const GOPHER_TYPE_IMAGE: GOPHER_TYPE = GOPHER_TYPE(8192u32);
pub const GOPHER_TYPE_INDEX_SERVER: GOPHER_TYPE = GOPHER_TYPE(128u32);
pub const GOPHER_TYPE_INLINE: GOPHER_TYPE = GOPHER_TYPE(1048576u32);
pub const GOPHER_TYPE_MAC_BINHEX: GOPHER_TYPE = GOPHER_TYPE(16u32);
pub const GOPHER_TYPE_MOVIE: GOPHER_TYPE = GOPHER_TYPE(32768u32);
pub const GOPHER_TYPE_PDF: GOPHER_TYPE = GOPHER_TYPE(262144u32);
pub const GOPHER_TYPE_REDUNDANT: GOPHER_TYPE = GOPHER_TYPE(1024u32);
pub const GOPHER_TYPE_SOUND: GOPHER_TYPE = GOPHER_TYPE(65536u32);
pub const GOPHER_TYPE_TELNET: GOPHER_TYPE = GOPHER_TYPE(256u32);
pub const GOPHER_TYPE_TEXT_FILE: GOPHER_TYPE = GOPHER_TYPE(1u32);
pub const GOPHER_TYPE_TN3270: GOPHER_TYPE = GOPHER_TYPE(2048u32);
pub const GOPHER_TYPE_UNIX_UUENCODED: GOPHER_TYPE = GOPHER_TYPE(64u32);
pub const GOPHER_TYPE_UNKNOWN: GOPHER_TYPE = GOPHER_TYPE(536870912u32);
impl ::core::convert::From<u32> for GOPHER_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GOPHER_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for GOPHER_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for GOPHER_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for GOPHER_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for GOPHER_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for GOPHER_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    pub Text: *mut i8,
}
impl GOPHER_UNKNOWN_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_UNKNOWN_ATTRIBUTE_TYPE").field("Text", &self.Text).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
impl ::core::cmp::Eq for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_UNKNOWN_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GOPHER_VERONICA_ATTRIBUTE_TYPE {
    pub TreeWalk: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl GOPHER_VERONICA_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_VERONICA_ATTRIBUTE_TYPE").field("TreeWalk", &self.TreeWalk).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.TreeWalk == other.TreeWalk
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GOPHER_VERONICA_ATTRIBUTE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GOPHER_VERONICA_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_VERSION_ATTRIBUTE_TYPE {
    pub Version: *mut i8,
}
impl GOPHER_VERSION_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_VERSION_ATTRIBUTE_TYPE").field("Version", &self.Version).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_VERSION_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
    }
}
impl ::core::cmp::Eq for GOPHER_VERSION_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_VERSION_ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct GOPHER_VIEW_ATTRIBUTE_TYPE {
    pub ContentType: *mut i8,
    pub Language: *mut i8,
    pub Size: u32,
}
impl GOPHER_VIEW_ATTRIBUTE_TYPE {}
impl ::core::default::Default for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GOPHER_VIEW_ATTRIBUTE_TYPE").field("ContentType", &self.ContentType).field("Language", &self.Language).field("Size", &self.Size).finish()
    }
}
impl ::core::cmp::PartialEq for GOPHER_VIEW_ATTRIBUTE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.ContentType == other.ContentType && self.Language == other.Language && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for GOPHER_VIEW_ATTRIBUTE_TYPE {}
unsafe impl ::windows::core::Abi for GOPHER_VIEW_ATTRIBUTE_TYPE {
    type Abi = Self;
}
pub const GROUPNAME_MAX_LENGTH: u32 = 120u32;
pub const GROUP_OWNER_STORAGE_SIZE: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDiskInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszpath: Param0, pdwclustersize: *mut u32, pdlavail: *mut u64, pdltotal: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDiskInfoA(pszpath: super::super::Foundation::PSTR, pdwclustersize: *mut u32, pdlavail: *mut u64, pdltotal: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetDiskInfoA(pszpath.into_param().abi(), ::core::mem::transmute(pdwclustersize), ::core::mem::transmute(pdlavail), ::core::mem::transmute(pdltotal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheConfigInfoA(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheConfigInfoA(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOA, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheConfigInfoA(::core::mem::transmute(lpcacheconfiginfo), ::core::mem::transmute(lpcbcacheconfiginfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheConfigInfoW(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheConfigInfoW(lpcacheconfiginfo: *mut INTERNET_CACHE_CONFIG_INFOW, lpcbcacheconfiginfo: *mut u32, dwfieldcontrol: CACHE_CONFIG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheConfigInfoW(::core::mem::transmute(lpcacheconfiginfo), ::core::mem::transmute(lpcbcacheconfiginfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryBinaryBlob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszurlname: Param0, dwtype: *mut u32, pftexpiretime: *mut super::super::Foundation::FILETIME, pftaccesstime: *mut super::super::Foundation::FILETIME, pftmodifiedtime: *mut super::super::Foundation::FILETIME, ppbblob: *mut *mut u8, pcbblob: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheEntryBinaryBlob(pwszurlname: super::super::Foundation::PWSTR, dwtype: *mut u32, pftexpiretime: *mut super::super::Foundation::FILETIME, pftaccesstime: *mut super::super::Foundation::FILETIME, pftmodifiedtime: *mut super::super::Foundation::FILETIME, ppbblob: *mut *mut u8, pcbblob: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetUrlCacheEntryBinaryBlob(pwszurlname.into_param().abi(), ::core::mem::transmute(dwtype), ::core::mem::transmute(pftexpiretime), ::core::mem::transmute(pftaccesstime), ::core::mem::transmute(pftmodifiedtime), ::core::mem::transmute(ppbblob), ::core::mem::transmute(pcbblob)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheEntryInfoA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheEntryInfoA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpszredirecturl: Param3, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheEntryInfoExA(lpszurl: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, lpszredirecturl: super::super::Foundation::PSTR, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheEntryInfoExA(lpszurl.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), lpszredirecturl.into_param().abi(), ::core::mem::transmute(lpcbredirecturl), ::core::mem::transmute(lpreserved), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpszredirecturl: Param3, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheEntryInfoExW(lpszurl: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, lpszredirecturl: super::super::Foundation::PWSTR, lpcbredirecturl: *mut u32, lpreserved: *mut ::core::ffi::c_void, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheEntryInfoExW(lpszurl.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), lpszredirecturl.into_param().abi(), ::core::mem::transmute(lpcbredirecturl), ::core::mem::transmute(lpreserved), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheEntryInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheEntryInfoW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheEntryInfoW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOA, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheGroupAttributeA(::core::mem::transmute(gid), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwattributes), ::core::mem::transmute(lpgroupinfo), ::core::mem::transmute(lpcbgroupinfo), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *mut INTERNET_CACHE_GROUP_INFOW, lpcbgroupinfo: *mut u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheGroupAttributeW(::core::mem::transmute(gid), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwattributes), ::core::mem::transmute(lpgroupinfo), ::core::mem::transmute(lpcbgroupinfo), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUrlCacheHeaderData(::core::mem::transmute(nidx), ::core::mem::transmute(lpdwdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherCreateLocatorA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszhost: Param0, nserverport: u16, lpszdisplaystring: Param2, lpszselectorstring: Param3, dwgophertype: u32, lpszlocator: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherCreateLocatorA(lpszhost: super::super::Foundation::PSTR, nserverport: u16, lpszdisplaystring: super::super::Foundation::PSTR, lpszselectorstring: super::super::Foundation::PSTR, dwgophertype: u32, lpszlocator: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherCreateLocatorA(lpszhost.into_param().abi(), ::core::mem::transmute(nserverport), lpszdisplaystring.into_param().abi(), lpszselectorstring.into_param().abi(), ::core::mem::transmute(dwgophertype), ::core::mem::transmute(lpszlocator), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherCreateLocatorW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszhost: Param0, nserverport: u16, lpszdisplaystring: Param2, lpszselectorstring: Param3, dwgophertype: u32, lpszlocator: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherCreateLocatorW(lpszhost: super::super::Foundation::PWSTR, nserverport: u16, lpszdisplaystring: super::super::Foundation::PWSTR, lpszselectorstring: super::super::Foundation::PWSTR, dwgophertype: u32, lpszlocator: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherCreateLocatorW(lpszhost.into_param().abi(), ::core::mem::transmute(nserverport), lpszdisplaystring.into_param().abi(), lpszselectorstring.into_param().abi(), ::core::mem::transmute(dwgophertype), ::core::mem::transmute(lpszlocator), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherFindFirstFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszsearchstring: Param2, lpfinddata: *mut GOPHER_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherFindFirstFileA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszsearchstring: super::super::Foundation::PSTR, lpfinddata: *mut GOPHER_FIND_DATAA, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GopherFindFirstFileA(::core::mem::transmute(hconnect), lpszlocator.into_param().abi(), lpszsearchstring.into_param().abi(), ::core::mem::transmute(lpfinddata), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherFindFirstFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszsearchstring: Param2, lpfinddata: *mut GOPHER_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherFindFirstFileW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszsearchstring: super::super::Foundation::PWSTR, lpfinddata: *mut GOPHER_FIND_DATAW, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GopherFindFirstFileW(::core::mem::transmute(hconnect), lpszlocator.into_param().abi(), lpszsearchstring.into_param().abi(), ::core::mem::transmute(lpfinddata), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetAttributeA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszattributename: Param2, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::core::option::Option<GOPHER_ATTRIBUTE_ENUMERATOR>, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherGetAttributeA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszattributename: super::super::Foundation::PSTR, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::windows::core::RawPtr, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherGetAttributeA(
            ::core::mem::transmute(hconnect),
            lpszlocator.into_param().abi(),
            lpszattributename.into_param().abi(),
            ::core::mem::transmute(lpbuffer),
            ::core::mem::transmute(dwbufferlength),
            ::core::mem::transmute(lpdwcharactersreturned),
            ::core::mem::transmute(lpfnenumerator),
            ::core::mem::transmute(dwcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetAttributeW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszattributename: Param2, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::core::option::Option<GOPHER_ATTRIBUTE_ENUMERATOR>, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherGetAttributeW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszattributename: super::super::Foundation::PWSTR, lpbuffer: *mut u8, dwbufferlength: u32, lpdwcharactersreturned: *mut u32, lpfnenumerator: ::windows::core::RawPtr, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherGetAttributeW(
            ::core::mem::transmute(hconnect),
            lpszlocator.into_param().abi(),
            lpszattributename.into_param().abi(),
            ::core::mem::transmute(lpbuffer),
            ::core::mem::transmute(dwbufferlength),
            ::core::mem::transmute(lpdwcharactersreturned),
            ::core::mem::transmute(lpfnenumerator),
            ::core::mem::transmute(dwcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetLocatorTypeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszlocator: Param0, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherGetLocatorTypeA(lpszlocator: super::super::Foundation::PSTR, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherGetLocatorTypeA(lpszlocator.into_param().abi(), ::core::mem::transmute(lpdwgophertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherGetLocatorTypeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszlocator: Param0, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherGetLocatorTypeW(lpszlocator: super::super::Foundation::PWSTR, lpdwgophertype: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GopherGetLocatorTypeW(lpszlocator.into_param().abi(), ::core::mem::transmute(lpdwgophertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherOpenFileA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszview: Param2, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherOpenFileA(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PSTR, lpszview: super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GopherOpenFileA(::core::mem::transmute(hconnect), lpszlocator.into_param().abi(), lpszview.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GopherOpenFileW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hconnect: *const ::core::ffi::c_void, lpszlocator: Param1, lpszview: Param2, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GopherOpenFileW(hconnect: *const ::core::ffi::c_void, lpszlocator: super::super::Foundation::PWSTR, lpszview: super::super::Foundation::PWSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GopherOpenFileW(::core::mem::transmute(hconnect), lpszlocator.into_param().abi(), lpszview.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HSR_ASYNC: u32 = 1u32;
pub const HSR_CHUNKED: u32 = 32u32;
pub const HSR_DOWNLOAD: u32 = 16u32;
pub const HSR_INITIATE: u32 = 8u32;
pub const HSR_SYNC: u32 = 4u32;
pub const HSR_USE_CONTEXT: u32 = 8u32;
pub const HTTP_1_1_CACHE_ENTRY: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_ADDREQ_FLAG(pub u32);
pub const HTTP_ADDREQ_FLAG_ADD: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(536870912u32);
pub const HTTP_ADDREQ_FLAG_ADD_IF_NEW: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(268435456u32);
pub const HTTP_ADDREQ_FLAG_COALESCE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(1073741824u32);
pub const HTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(16777216u32);
pub const HTTP_ADDREQ_FLAG_REPLACE: HTTP_ADDREQ_FLAG = HTTP_ADDREQ_FLAG(2147483648u32);
impl ::core::convert::From<u32> for HTTP_ADDREQ_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_ADDREQ_FLAG {
    type Abi = Self;
}
impl ::core::ops::BitOr for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for HTTP_ADDREQ_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for HTTP_ADDREQ_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for HTTP_ADDREQ_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HTTP_ADDREQ_FLAGS_MASK: u32 = 4294901760u32;
pub const HTTP_ADDREQ_FLAG_ALLOW_EMPTY_VALUES: u32 = 67108864u32;
pub const HTTP_ADDREQ_FLAG_RESPONSE_HEADERS: u32 = 33554432u32;
pub const HTTP_ADDREQ_INDEX_MASK: u32 = 65535u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE: u32 = 3u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_CROSS_SITE_LAX: u32 = 2u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_MAX: u32 = 3u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_SAME_SITE: u32 = 1u32;
pub const HTTP_COOKIES_SAME_SITE_LEVEL_UNKNOWN: u32 = 0u32;
pub const HTTP_MAJOR_VERSION: u32 = 1u32;
pub const HTTP_MINOR_VERSION: u32 = 0u32;
pub type HTTP_POLICY_EXTENSION_INIT = unsafe extern "system" fn(version: HTTP_POLICY_EXTENSION_VERSION, r#type: HTTP_POLICY_EXTENSION_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> u32;
pub type HTTP_POLICY_EXTENSION_SHUTDOWN = unsafe extern "system" fn(r#type: HTTP_POLICY_EXTENSION_TYPE) -> u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_POLICY_EXTENSION_TYPE(pub i32);
pub const POLICY_EXTENSION_TYPE_NONE: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(0i32);
pub const POLICY_EXTENSION_TYPE_WINHTTP: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(1i32);
pub const POLICY_EXTENSION_TYPE_WININET: HTTP_POLICY_EXTENSION_TYPE = HTTP_POLICY_EXTENSION_TYPE(2i32);
impl ::core::convert::From<i32> for HTTP_POLICY_EXTENSION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_POLICY_EXTENSION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_POLICY_EXTENSION_VERSION(pub i32);
pub const POLICY_EXTENSION_VERSION1: HTTP_POLICY_EXTENSION_VERSION = HTTP_POLICY_EXTENSION_VERSION(1i32);
impl ::core::convert::From<i32> for HTTP_POLICY_EXTENSION_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_POLICY_EXTENSION_VERSION {
    type Abi = Self;
}
pub const HTTP_PROTOCOL_FLAG_HTTP2: u32 = 2u32;
pub const HTTP_PROTOCOL_MASK: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_PUSH_NOTIFICATION_STATUS {
    pub ChannelStatusValid: super::super::Foundation::BOOL,
    pub ChannelStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl HTTP_PUSH_NOTIFICATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_PUSH_NOTIFICATION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_PUSH_NOTIFICATION_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HTTP_PUSH_NOTIFICATION_STATUS").field("ChannelStatusValid", &self.ChannelStatusValid).field("ChannelStatus", &self.ChannelStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_PUSH_NOTIFICATION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ChannelStatusValid == other.ChannelStatusValid && self.ChannelStatus == other.ChannelStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_PUSH_NOTIFICATION_STATUS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_PUSH_NOTIFICATION_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HTTP_PUSH_TRANSPORT_SETTING {
    pub TransportSettingId: ::windows::core::GUID,
    pub BrokerEventId: ::windows::core::GUID,
}
impl HTTP_PUSH_TRANSPORT_SETTING {}
impl ::core::default::Default for HTTP_PUSH_TRANSPORT_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HTTP_PUSH_TRANSPORT_SETTING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HTTP_PUSH_TRANSPORT_SETTING").field("TransportSettingId", &self.TransportSettingId).field("BrokerEventId", &self.BrokerEventId).finish()
    }
}
impl ::core::cmp::PartialEq for HTTP_PUSH_TRANSPORT_SETTING {
    fn eq(&self, other: &Self) -> bool {
        self.TransportSettingId == other.TransportSettingId && self.BrokerEventId == other.BrokerEventId
    }
}
impl ::core::cmp::Eq for HTTP_PUSH_TRANSPORT_SETTING {}
unsafe impl ::windows::core::Abi for HTTP_PUSH_TRANSPORT_SETTING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HTTP_PUSH_WAIT_HANDLE(pub isize);
impl ::core::default::Default for HTTP_PUSH_WAIT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HTTP_PUSH_WAIT_HANDLE {}
unsafe impl ::windows::core::Abi for HTTP_PUSH_WAIT_HANDLE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_PUSH_WAIT_TYPE(pub i32);
pub const HttpPushWaitEnableComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(0i32);
pub const HttpPushWaitReceiveComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(1i32);
pub const HttpPushWaitSendComplete: HTTP_PUSH_WAIT_TYPE = HTTP_PUSH_WAIT_TYPE(2i32);
impl ::core::convert::From<i32> for HTTP_PUSH_WAIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_PUSH_WAIT_TYPE {
    type Abi = Self;
}
pub const HTTP_QUERY_ACCEPT: u32 = 24u32;
pub const HTTP_QUERY_ACCEPT_CHARSET: u32 = 25u32;
pub const HTTP_QUERY_ACCEPT_ENCODING: u32 = 26u32;
pub const HTTP_QUERY_ACCEPT_LANGUAGE: u32 = 27u32;
pub const HTTP_QUERY_ACCEPT_RANGES: u32 = 42u32;
pub const HTTP_QUERY_AGE: u32 = 48u32;
pub const HTTP_QUERY_ALLOW: u32 = 7u32;
pub const HTTP_QUERY_AUTHENTICATION_INFO: u32 = 76u32;
pub const HTTP_QUERY_AUTHORIZATION: u32 = 28u32;
pub const HTTP_QUERY_CACHE_CONTROL: u32 = 49u32;
pub const HTTP_QUERY_CONNECTION: u32 = 23u32;
pub const HTTP_QUERY_CONTENT_BASE: u32 = 50u32;
pub const HTTP_QUERY_CONTENT_DESCRIPTION: u32 = 4u32;
pub const HTTP_QUERY_CONTENT_DISPOSITION: u32 = 47u32;
pub const HTTP_QUERY_CONTENT_ENCODING: u32 = 29u32;
pub const HTTP_QUERY_CONTENT_ID: u32 = 3u32;
pub const HTTP_QUERY_CONTENT_LANGUAGE: u32 = 6u32;
pub const HTTP_QUERY_CONTENT_LENGTH: u32 = 5u32;
pub const HTTP_QUERY_CONTENT_LOCATION: u32 = 51u32;
pub const HTTP_QUERY_CONTENT_MD5: u32 = 52u32;
pub const HTTP_QUERY_CONTENT_RANGE: u32 = 53u32;
pub const HTTP_QUERY_CONTENT_TRANSFER_ENCODING: u32 = 2u32;
pub const HTTP_QUERY_CONTENT_TYPE: u32 = 1u32;
pub const HTTP_QUERY_COOKIE: u32 = 44u32;
pub const HTTP_QUERY_COST: u32 = 15u32;
pub const HTTP_QUERY_CUSTOM: u32 = 65535u32;
pub const HTTP_QUERY_DATE: u32 = 9u32;
pub const HTTP_QUERY_DEFAULT_STYLE: u32 = 84u32;
pub const HTTP_QUERY_DERIVED_FROM: u32 = 14u32;
pub const HTTP_QUERY_DO_NOT_TRACK: u32 = 88u32;
pub const HTTP_QUERY_ECHO_HEADERS: u32 = 73u32;
pub const HTTP_QUERY_ECHO_HEADERS_CRLF: u32 = 74u32;
pub const HTTP_QUERY_ECHO_REPLY: u32 = 72u32;
pub const HTTP_QUERY_ECHO_REQUEST: u32 = 71u32;
pub const HTTP_QUERY_ETAG: u32 = 54u32;
pub const HTTP_QUERY_EXPECT: u32 = 68u32;
pub const HTTP_QUERY_EXPIRES: u32 = 10u32;
pub const HTTP_QUERY_FLAG_COALESCE: u32 = 268435456u32;
pub const HTTP_QUERY_FLAG_COALESCE_WITH_COMMA: u32 = 67108864u32;
pub const HTTP_QUERY_FLAG_NUMBER: u32 = 536870912u32;
pub const HTTP_QUERY_FLAG_NUMBER64: u32 = 134217728u32;
pub const HTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 2147483648u32;
pub const HTTP_QUERY_FLAG_SYSTEMTIME: u32 = 1073741824u32;
pub const HTTP_QUERY_FORWARDED: u32 = 30u32;
pub const HTTP_QUERY_FROM: u32 = 31u32;
pub const HTTP_QUERY_HOST: u32 = 55u32;
pub const HTTP_QUERY_HTTP2_SETTINGS: u32 = 90u32;
pub const HTTP_QUERY_IF_MATCH: u32 = 56u32;
pub const HTTP_QUERY_IF_MODIFIED_SINCE: u32 = 32u32;
pub const HTTP_QUERY_IF_NONE_MATCH: u32 = 57u32;
pub const HTTP_QUERY_IF_RANGE: u32 = 58u32;
pub const HTTP_QUERY_IF_UNMODIFIED_SINCE: u32 = 59u32;
pub const HTTP_QUERY_INCLUDE_REFERER_TOKEN_BINDING_ID: u32 = 93u32;
pub const HTTP_QUERY_INCLUDE_REFERRED_TOKEN_BINDING_ID: u32 = 93u32;
pub const HTTP_QUERY_KEEP_ALIVE: u32 = 89u32;
pub const HTTP_QUERY_LAST_MODIFIED: u32 = 11u32;
pub const HTTP_QUERY_LINK: u32 = 16u32;
pub const HTTP_QUERY_LOCATION: u32 = 33u32;
pub const HTTP_QUERY_MAX: u32 = 95u32;
pub const HTTP_QUERY_MAX_FORWARDS: u32 = 60u32;
pub const HTTP_QUERY_MESSAGE_ID: u32 = 12u32;
pub const HTTP_QUERY_MIME_VERSION: u32 = 0u32;
pub const HTTP_QUERY_ORIG_URI: u32 = 34u32;
pub const HTTP_QUERY_P3P: u32 = 80u32;
pub const HTTP_QUERY_PASSPORT_CONFIG: u32 = 78u32;
pub const HTTP_QUERY_PASSPORT_URLS: u32 = 77u32;
pub const HTTP_QUERY_PRAGMA: u32 = 17u32;
pub const HTTP_QUERY_PROXY_AUTHENTICATE: u32 = 41u32;
pub const HTTP_QUERY_PROXY_AUTHORIZATION: u32 = 61u32;
pub const HTTP_QUERY_PROXY_CONNECTION: u32 = 69u32;
pub const HTTP_QUERY_PROXY_SUPPORT: u32 = 75u32;
pub const HTTP_QUERY_PUBLIC: u32 = 8u32;
pub const HTTP_QUERY_PUBLIC_KEY_PINS: u32 = 94u32;
pub const HTTP_QUERY_PUBLIC_KEY_PINS_REPORT_ONLY: u32 = 95u32;
pub const HTTP_QUERY_RANGE: u32 = 62u32;
pub const HTTP_QUERY_RAW_HEADERS: u32 = 21u32;
pub const HTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22u32;
pub const HTTP_QUERY_REFERER: u32 = 35u32;
pub const HTTP_QUERY_REFRESH: u32 = 46u32;
pub const HTTP_QUERY_REQUEST_METHOD: u32 = 45u32;
pub const HTTP_QUERY_RETRY_AFTER: u32 = 36u32;
pub const HTTP_QUERY_SERVER: u32 = 37u32;
pub const HTTP_QUERY_SET_COOKIE: u32 = 43u32;
pub const HTTP_QUERY_SET_COOKIE2: u32 = 87u32;
pub const HTTP_QUERY_STATUS_CODE: u32 = 19u32;
pub const HTTP_QUERY_STATUS_TEXT: u32 = 20u32;
pub const HTTP_QUERY_STRICT_TRANSPORT_SECURITY: u32 = 91u32;
pub const HTTP_QUERY_TITLE: u32 = 38u32;
pub const HTTP_QUERY_TOKEN_BINDING: u32 = 92u32;
pub const HTTP_QUERY_TRANSFER_ENCODING: u32 = 63u32;
pub const HTTP_QUERY_TRANSLATE: u32 = 82u32;
pub const HTTP_QUERY_UNLESS_MODIFIED_SINCE: u32 = 70u32;
pub const HTTP_QUERY_UPGRADE: u32 = 64u32;
pub const HTTP_QUERY_URI: u32 = 13u32;
pub const HTTP_QUERY_USER_AGENT: u32 = 39u32;
pub const HTTP_QUERY_VARY: u32 = 65u32;
pub const HTTP_QUERY_VERSION: u32 = 18u32;
pub const HTTP_QUERY_VIA: u32 = 66u32;
pub const HTTP_QUERY_WARNING: u32 = 67u32;
pub const HTTP_QUERY_WWW_AUTHENTICATE: u32 = 40u32;
pub const HTTP_QUERY_X_CONTENT_TYPE_OPTIONS: u32 = 79u32;
pub const HTTP_QUERY_X_FRAME_OPTIONS: u32 = 85u32;
pub const HTTP_QUERY_X_P2P_PEERDIST: u32 = 81u32;
pub const HTTP_QUERY_X_UA_COMPATIBLE: u32 = 83u32;
pub const HTTP_QUERY_X_XSS_PROTECTION: u32 = 86u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HTTP_REQUEST_TIMES {
    pub cTimes: u32,
    pub rgTimes: [u64; 32],
}
impl HTTP_REQUEST_TIMES {}
impl ::core::default::Default for HTTP_REQUEST_TIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HTTP_REQUEST_TIMES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HTTP_REQUEST_TIMES").field("cTimes", &self.cTimes).field("rgTimes", &self.rgTimes).finish()
    }
}
impl ::core::cmp::PartialEq for HTTP_REQUEST_TIMES {
    fn eq(&self, other: &Self) -> bool {
        self.cTimes == other.cTimes && self.rgTimes == other.rgTimes
    }
}
impl ::core::cmp::Eq for HTTP_REQUEST_TIMES {}
unsafe impl ::windows::core::Abi for HTTP_REQUEST_TIMES {
    type Abi = Self;
}
pub const HTTP_STATUS_MISDIRECTED_REQUEST: u32 = 421u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HTTP_WEB_SOCKET_ASYNC_RESULT {
    pub AsyncResult: INTERNET_ASYNC_RESULT,
    pub Operation: HTTP_WEB_SOCKET_OPERATION,
    pub BufferType: HTTP_WEB_SOCKET_BUFFER_TYPE,
    pub dwBytesTransferred: u32,
}
impl HTTP_WEB_SOCKET_ASYNC_RESULT {}
impl ::core::default::Default for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HTTP_WEB_SOCKET_ASYNC_RESULT").field("AsyncResult", &self.AsyncResult).field("Operation", &self.Operation).field("BufferType", &self.BufferType).field("dwBytesTransferred", &self.dwBytesTransferred).finish()
    }
}
impl ::core::cmp::PartialEq for HTTP_WEB_SOCKET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.AsyncResult == other.AsyncResult && self.Operation == other.Operation && self.BufferType == other.BufferType && self.dwBytesTransferred == other.dwBytesTransferred
    }
}
impl ::core::cmp::Eq for HTTP_WEB_SOCKET_ASYNC_RESULT {}
unsafe impl ::windows::core::Abi for HTTP_WEB_SOCKET_ASYNC_RESULT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_WEB_SOCKET_BUFFER_TYPE(pub i32);
pub const HTTP_WEB_SOCKET_BINARY_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(0i32);
pub const HTTP_WEB_SOCKET_BINARY_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(1i32);
pub const HTTP_WEB_SOCKET_UTF8_MESSAGE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(2i32);
pub const HTTP_WEB_SOCKET_UTF8_FRAGMENT_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(3i32);
pub const HTTP_WEB_SOCKET_CLOSE_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(4i32);
pub const HTTP_WEB_SOCKET_PING_TYPE: HTTP_WEB_SOCKET_BUFFER_TYPE = HTTP_WEB_SOCKET_BUFFER_TYPE(5i32);
impl ::core::convert::From<i32> for HTTP_WEB_SOCKET_BUFFER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_WEB_SOCKET_BUFFER_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_WEB_SOCKET_CLOSE_STATUS(pub i32);
pub const HTTP_WEB_SOCKET_SUCCESS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1000i32);
pub const HTTP_WEB_SOCKET_ENDPOINT_TERMINATED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1001i32);
pub const HTTP_WEB_SOCKET_PROTOCOL_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1002i32);
pub const HTTP_WEB_SOCKET_INVALID_DATA_TYPE_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1003i32);
pub const HTTP_WEB_SOCKET_EMPTY_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1005i32);
pub const HTTP_WEB_SOCKET_ABORTED_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1006i32);
pub const HTTP_WEB_SOCKET_INVALID_PAYLOAD_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1007i32);
pub const HTTP_WEB_SOCKET_POLICY_VIOLATION_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1008i32);
pub const HTTP_WEB_SOCKET_MESSAGE_TOO_BIG_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1009i32);
pub const HTTP_WEB_SOCKET_UNSUPPORTED_EXTENSIONS_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1010i32);
pub const HTTP_WEB_SOCKET_SERVER_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1011i32);
pub const HTTP_WEB_SOCKET_SECURE_HANDSHAKE_ERROR_CLOSE_STATUS: HTTP_WEB_SOCKET_CLOSE_STATUS = HTTP_WEB_SOCKET_CLOSE_STATUS(1015i32);
impl ::core::convert::From<i32> for HTTP_WEB_SOCKET_CLOSE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_WEB_SOCKET_CLOSE_STATUS {
    type Abi = Self;
}
pub const HTTP_WEB_SOCKET_MAX_CLOSE_REASON_LENGTH: u32 = 123u32;
pub const HTTP_WEB_SOCKET_MIN_KEEPALIVE_VALUE: u32 = 10000u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HTTP_WEB_SOCKET_OPERATION(pub i32);
pub const HTTP_WEB_SOCKET_SEND_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(0i32);
pub const HTTP_WEB_SOCKET_RECEIVE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(1i32);
pub const HTTP_WEB_SOCKET_CLOSE_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(2i32);
pub const HTTP_WEB_SOCKET_SHUTDOWN_OPERATION: HTTP_WEB_SOCKET_OPERATION = HTTP_WEB_SOCKET_OPERATION(3i32);
impl ::core::convert::From<i32> for HTTP_WEB_SOCKET_OPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HTTP_WEB_SOCKET_OPERATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpAddRequestHeadersA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hrequest: *const ::core::ffi::c_void, lpszheaders: Param1, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddRequestHeadersA(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpAddRequestHeadersA(::core::mem::transmute(hrequest), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(dwmodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpAddRequestHeadersW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hrequest: *const ::core::ffi::c_void, lpszheaders: Param1, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpAddRequestHeadersW(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, dwmodifiers: HTTP_ADDREQ_FLAG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpAddRequestHeadersW(::core::mem::transmute(hrequest), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(dwmodifiers)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCheckDavComplianceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, lpszcompliancetoken: Param1, lpffound: *mut i32, hwnd: Param3, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCheckDavComplianceA(lpszurl: super::super::Foundation::PSTR, lpszcompliancetoken: super::super::Foundation::PSTR, lpffound: *mut i32, hwnd: super::super::Foundation::HWND, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpCheckDavComplianceA(lpszurl.into_param().abi(), lpszcompliancetoken.into_param().abi(), ::core::mem::transmute(lpffound), hwnd.into_param().abi(), ::core::mem::transmute(lpvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpCheckDavComplianceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, lpszcompliancetoken: Param1, lpffound: *mut i32, hwnd: Param3, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCheckDavComplianceW(lpszurl: super::super::Foundation::PWSTR, lpszcompliancetoken: super::super::Foundation::PWSTR, lpffound: *mut i32, hwnd: super::super::Foundation::HWND, lpvreserved: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpCheckDavComplianceW(lpszurl.into_param().abi(), lpszcompliancetoken.into_param().abi(), ::core::mem::transmute(lpffound), hwnd.into_param().abi(), ::core::mem::transmute(lpvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpCloseDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpCloseDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(HttpCloseDependencyHandle(::core::mem::transmute(hdependencyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpDuplicateDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void, phduplicateddependencyhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpDuplicateDependencyHandle(hdependencyhandle: *const ::core::ffi::c_void, phduplicateddependencyhandle: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpDuplicateDependencyHandle(::core::mem::transmute(hdependencyhandle), ::core::mem::transmute(phduplicateddependencyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpEndRequestA(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpEndRequestA(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpEndRequestA(::core::mem::transmute(hrequest), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpEndRequestW(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpEndRequestW(hrequest: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpEndRequestW(::core::mem::transmute(hrequest), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpGetServerCredentials<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszurl: Param0, ppwszusername: *mut super::super::Foundation::PWSTR, ppwszpassword: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpGetServerCredentials(pwszurl: super::super::Foundation::PWSTR, ppwszusername: *mut super::super::Foundation::PWSTR, ppwszpassword: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(HttpGetServerCredentials(pwszurl.into_param().abi(), ::core::mem::transmute(ppwszusername), ::core::mem::transmute(ppwszpassword)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpIndicatePageLoadComplete(hdependencyhandle: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpIndicatePageLoadComplete(hdependencyhandle: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpIndicatePageLoadComplete(::core::mem::transmute(hdependencyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpIsHostHstsEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcwszurl: Param0, pfishsts: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpIsHostHstsEnabled(pcwszurl: super::super::Foundation::PWSTR, pfishsts: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(HttpIsHostHstsEnabled(pcwszurl.into_param().abi(), ::core::mem::transmute(pfishsts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpOpenDependencyHandle<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hrequesthandle: *const ::core::ffi::c_void, fbackground: Param1, phdependencyhandle: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpOpenDependencyHandle(hrequesthandle: *const ::core::ffi::c_void, fbackground: super::super::Foundation::BOOL, phdependencyhandle: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpOpenDependencyHandle(::core::mem::transmute(hrequesthandle), fbackground.into_param().abi(), ::core::mem::transmute(phdependencyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpOpenRequestA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
    hconnect: *const ::core::ffi::c_void,
    lpszverb: Param1,
    lpszobjectname: Param2,
    lpszversion: Param3,
    lpszreferrer: Param4,
    lplpszaccepttypes: *const super::super::Foundation::PSTR,
    dwflags: u32,
    dwcontext: usize,
) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpOpenRequestA(hconnect: *const ::core::ffi::c_void, lpszverb: super::super::Foundation::PSTR, lpszobjectname: super::super::Foundation::PSTR, lpszversion: super::super::Foundation::PSTR, lpszreferrer: super::super::Foundation::PSTR, lplpszaccepttypes: *const super::super::Foundation::PSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HttpOpenRequestA(::core::mem::transmute(hconnect), lpszverb.into_param().abi(), lpszobjectname.into_param().abi(), lpszversion.into_param().abi(), lpszreferrer.into_param().abi(), ::core::mem::transmute(lplpszaccepttypes), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpOpenRequestW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hconnect: *const ::core::ffi::c_void,
    lpszverb: Param1,
    lpszobjectname: Param2,
    lpszversion: Param3,
    lpszreferrer: Param4,
    lplpszaccepttypes: *const super::super::Foundation::PWSTR,
    dwflags: u32,
    dwcontext: usize,
) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpOpenRequestW(hconnect: *const ::core::ffi::c_void, lpszverb: super::super::Foundation::PWSTR, lpszobjectname: super::super::Foundation::PWSTR, lpszversion: super::super::Foundation::PWSTR, lpszreferrer: super::super::Foundation::PWSTR, lplpszaccepttypes: *const super::super::Foundation::PWSTR, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HttpOpenRequestW(::core::mem::transmute(hconnect), lpszverb.into_param().abi(), lpszobjectname.into_param().abi(), lpszversion.into_param().abi(), lpszreferrer.into_param().abi(), ::core::mem::transmute(lplpszaccepttypes), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpPushClose<'a, Param0: ::windows::core::IntoParam<'a, HTTP_PUSH_WAIT_HANDLE>>(hwait: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpPushClose(hwait: HTTP_PUSH_WAIT_HANDLE);
        }
        ::core::mem::transmute(HttpPushClose(hwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpPushEnable(hrequest: *const ::core::ffi::c_void, ptransportsetting: *const HTTP_PUSH_TRANSPORT_SETTING, phwait: *mut HTTP_PUSH_WAIT_HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpPushEnable(hrequest: *const ::core::ffi::c_void, ptransportsetting: *const HTTP_PUSH_TRANSPORT_SETTING, phwait: *mut HTTP_PUSH_WAIT_HANDLE) -> u32;
        }
        ::core::mem::transmute(HttpPushEnable(::core::mem::transmute(hrequest), ::core::mem::transmute(ptransportsetting), ::core::mem::transmute(phwait)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpPushWait<'a, Param0: ::windows::core::IntoParam<'a, HTTP_PUSH_WAIT_HANDLE>>(hwait: Param0, etype: HTTP_PUSH_WAIT_TYPE, pnotificationstatus: *mut HTTP_PUSH_NOTIFICATION_STATUS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpPushWait(hwait: HTTP_PUSH_WAIT_HANDLE, etype: HTTP_PUSH_WAIT_TYPE, pnotificationstatus: *mut HTTP_PUSH_NOTIFICATION_STATUS) -> u32;
        }
        ::core::mem::transmute(HttpPushWait(hwait.into_param().abi(), ::core::mem::transmute(etype), ::core::mem::transmute(pnotificationstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpQueryInfoA(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryInfoA(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpQueryInfoA(::core::mem::transmute(hrequest), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(lpdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpQueryInfoW(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpQueryInfoW(hrequest: *const ::core::ffi::c_void, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpdwindex: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpQueryInfoW(::core::mem::transmute(hrequest), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(lpdwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hrequest: *const ::core::ffi::c_void, lpszheaders: Param1, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendRequestA(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpSendRequestA(::core::mem::transmute(hrequest), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(lpoptional), ::core::mem::transmute(dwoptionallength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestExA(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendRequestExA(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpSendRequestExA(::core::mem::transmute(hrequest), ::core::mem::transmute(lpbuffersin), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestExW(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendRequestExW(hrequest: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpSendRequestExW(::core::mem::transmute(hrequest), ::core::mem::transmute(lpbuffersin), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpSendRequestW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hrequest: *const ::core::ffi::c_void, lpszheaders: Param1, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpSendRequestW(hrequest: *const ::core::ffi::c_void, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, lpoptional: *const ::core::ffi::c_void, dwoptionallength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpSendRequestW(::core::mem::transmute(hrequest), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(lpoptional), ::core::mem::transmute(dwoptionallength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketClose(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpWebSocketClose(::core::mem::transmute(hwebsocket), ::core::mem::transmute(usstatus), ::core::mem::transmute(pvreason), ::core::mem::transmute(dwreasonlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketCompleteUpgrade(hrequest: *const ::core::ffi::c_void, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HttpWebSocketCompleteUpgrade(::core::mem::transmute(hrequest), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: *mut ::core::ffi::c_void, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketQueryCloseStatus(hwebsocket: *const ::core::ffi::c_void, pusstatus: *mut u16, pvreason: *mut ::core::ffi::c_void, dwreasonlength: u32, pdwreasonlengthconsumed: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpWebSocketQueryCloseStatus(::core::mem::transmute(hwebsocket), ::core::mem::transmute(pusstatus), ::core::mem::transmute(pvreason), ::core::mem::transmute(dwreasonlength), ::core::mem::transmute(pdwreasonlengthconsumed)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pbuffertype: *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketReceive(hwebsocket: *const ::core::ffi::c_void, pvbuffer: *mut ::core::ffi::c_void, dwbufferlength: u32, pdwbytesread: *mut u32, pbuffertype: *mut HTTP_WEB_SOCKET_BUFFER_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpWebSocketReceive(::core::mem::transmute(hwebsocket), ::core::mem::transmute(pvbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(pdwbytesread), ::core::mem::transmute(pbuffertype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, buffertype: HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketSend(hwebsocket: *const ::core::ffi::c_void, buffertype: HTTP_WEB_SOCKET_BUFFER_TYPE, pvbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpWebSocketSend(::core::mem::transmute(hwebsocket), ::core::mem::transmute(buffertype), ::core::mem::transmute(pvbuffer), ::core::mem::transmute(dwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpWebSocketShutdown(hwebsocket: *const ::core::ffi::c_void, usstatus: u16, pvreason: *const ::core::ffi::c_void, dwreasonlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HttpWebSocketShutdown(::core::mem::transmute(hwebsocket), ::core::mem::transmute(usstatus), ::core::mem::transmute(pvreason), ::core::mem::transmute(dwreasonlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ICU_USERNAME: u32 = 1073741824u32;
pub const IDENTITY_CACHE_ENTRY: u32 = 2147483648u32;
pub const IDSI_FLAG_KEEP_ALIVE: u32 = 1u32;
pub const IDSI_FLAG_PROXY: u32 = 4u32;
pub const IDSI_FLAG_SECURE: u32 = 2u32;
pub const IDSI_FLAG_TUNNEL: u32 = 8u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDialBranding(pub ::windows::core::IUnknown);
impl IDialBranding {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzconnectoid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzconnectoid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetBitmap(&self, dwindex: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP> {
        let mut result__: <super::super::Graphics::Gdi::HBITMAP as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDialBranding {
    type Vtable = IDialBranding_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aecafa9_4306_43cc_8c5a_765f2979cc16);
}
impl ::core::convert::From<IDialBranding> for ::windows::core::IUnknown {
    fn from(value: IDialBranding) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDialBranding> for ::windows::core::IUnknown {
    fn from(value: &IDialBranding) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDialBranding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDialBranding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialBranding_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDialEngine(pub ::windows::core::IUnknown);
impl IDialEngine {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IDialEventSink>>(&self, pwzconnectoid: Param0, pides: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwzconnectoid.into_param().abi(), pides.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzproperty: Param0, pwzvalue: Param1, dwbufsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwzproperty.into_param().abi(), pwzvalue.into_param().abi(), ::core::mem::transmute(dwbufsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwzproperty: Param0, pwzvalue: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwzproperty.into_param().abi(), pwzvalue.into_param().abi()).ok()
    }
    pub unsafe fn Dial(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn HangUp(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetConnectedState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnectHandle(&self) -> ::windows::core::Result<usize> {
        let mut result__: <usize as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDialEngine {
    type Vtable = IDialEngine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39fd782b_7905_40d5_9148_3c9b190423d5);
}
impl ::core::convert::From<IDialEngine> for ::windows::core::IUnknown {
    fn from(value: IDialEngine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDialEngine> for ::windows::core::IUnknown {
    fn from(value: &IDialEngine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDialEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDialEngine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEngine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstate: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwhandle: *mut usize) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDialEventSink(pub ::windows::core::IUnknown);
impl IDialEventSink {
    pub unsafe fn OnEvent(&self, dwevent: u32, dwstatus: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwevent), ::core::mem::transmute(dwstatus)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDialEventSink {
    type Vtable = IDialEventSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d86f4ff_6e2d_4488_b2e9_6934afd41bea);
}
impl ::core::convert::From<IDialEventSink> for ::windows::core::IUnknown {
    fn from(value: IDialEventSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDialEventSink> for ::windows::core::IUnknown {
    fn from(value: &IDialEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDialEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDialEventSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT,
);
pub const IMMUTABLE_CACHE_ENTRY: u32 = 524288u32;
pub const INSTALLED_CACHE_ENTRY: u32 = 268435456u32;
pub const INTERENT_GOONLINE_MASK: u32 = 3u32;
pub const INTERENT_GOONLINE_NOPROMPT: u32 = 2u32;
pub const INTERENT_GOONLINE_REFRESH: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_ACCESS_TYPE(pub u32);
pub const INTERNET_OPEN_TYPE_DIRECT: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(1u32);
pub const INTERNET_OPEN_TYPE_PRECONFIG: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(0u32);
pub const INTERNET_OPEN_TYPE_PROXY: INTERNET_ACCESS_TYPE = INTERNET_ACCESS_TYPE(3u32);
impl ::core::convert::From<u32> for INTERNET_ACCESS_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_ACCESS_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_ACCESS_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_ACCESS_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_ACCESS_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_ACCESS_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_ACCESS_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_ASYNC_RESULT {
    pub dwResult: usize,
    pub dwError: u32,
}
impl INTERNET_ASYNC_RESULT {}
impl ::core::default::Default for INTERNET_ASYNC_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_ASYNC_RESULT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_ASYNC_RESULT").field("dwResult", &self.dwResult).field("dwError", &self.dwError).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_ASYNC_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwResult == other.dwResult && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for INTERNET_ASYNC_RESULT {}
unsafe impl ::windows::core::Abi for INTERNET_ASYNC_RESULT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
pub struct INTERNET_AUTH_NOTIFY_DATA {
    pub cbStruct: u32,
    pub dwOptions: u32,
    pub pfnNotify: ::core::option::Option<PFN_AUTH_NOTIFY>,
    pub dwContext: usize,
}
impl INTERNET_AUTH_NOTIFY_DATA {}
impl ::core::default::Default for INTERNET_AUTH_NOTIFY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_AUTH_NOTIFY_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_AUTH_NOTIFY_DATA").field("cbStruct", &self.cbStruct).field("dwOptions", &self.dwOptions).field("dwContext", &self.dwContext).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_AUTH_NOTIFY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.dwOptions == other.dwOptions && self.pfnNotify.map(|f| f as usize) == other.pfnNotify.map(|f| f as usize) && self.dwContext == other.dwContext
    }
}
impl ::core::cmp::Eq for INTERNET_AUTH_NOTIFY_DATA {}
unsafe impl ::windows::core::Abi for INTERNET_AUTH_NOTIFY_DATA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const INTERNET_AUTH_SCHEME_BASIC: u32 = 0u32;
pub const INTERNET_AUTH_SCHEME_DIGEST: u32 = 1u32;
pub const INTERNET_AUTH_SCHEME_KERBEROS: u32 = 3u32;
pub const INTERNET_AUTH_SCHEME_NEGOTIATE: u32 = 4u32;
pub const INTERNET_AUTH_SCHEME_NTLM: u32 = 2u32;
pub const INTERNET_AUTH_SCHEME_PASSPORT: u32 = 5u32;
pub const INTERNET_AUTH_SCHEME_UNKNOWN: u32 = 6u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_AUTODIAL(pub u32);
pub const INTERNET_AUTODIAL_FAILIFSECURITYCHECK: INTERNET_AUTODIAL = INTERNET_AUTODIAL(4u32);
pub const INTERNET_AUTODIAL_FORCE_ONLINE: INTERNET_AUTODIAL = INTERNET_AUTODIAL(1u32);
pub const INTERNET_AUTODIAL_FORCE_UNATTENDED: INTERNET_AUTODIAL = INTERNET_AUTODIAL(2u32);
pub const INTERNET_AUTODIAL_OVERRIDE_NET_PRESENT: INTERNET_AUTODIAL = INTERNET_AUTODIAL(8u32);
impl ::core::convert::From<u32> for INTERNET_AUTODIAL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_AUTODIAL {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_AUTODIAL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_AUTODIAL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_AUTODIAL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_AUTODIAL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_AUTODIAL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_AUTOPROXY_INIT_DEFAULT: u32 = 1u32;
pub const INTERNET_AUTOPROXY_INIT_DOWNLOADSYNC: u32 = 2u32;
pub const INTERNET_AUTOPROXY_INIT_ONLYQUERY: u32 = 8u32;
pub const INTERNET_AUTOPROXY_INIT_QUERYSTATE: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_BUFFERSA {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSA,
    pub lpcszHeader: super::super::Foundation::PSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_BUFFERSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_BUFFERSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_BUFFERSA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_BUFFERSA")
            .field("dwStructSize", &self.dwStructSize)
            .field("Next", &self.Next)
            .field("lpcszHeader", &self.lpcszHeader)
            .field("dwHeadersLength", &self.dwHeadersLength)
            .field("dwHeadersTotal", &self.dwHeadersTotal)
            .field("lpvBuffer", &self.lpvBuffer)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBufferTotal", &self.dwBufferTotal)
            .field("dwOffsetLow", &self.dwOffsetLow)
            .field("dwOffsetHigh", &self.dwOffsetHigh)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_BUFFERSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_BUFFERSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_BUFFERSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_BUFFERSW {
    pub dwStructSize: u32,
    pub Next: *mut INTERNET_BUFFERSW,
    pub lpcszHeader: super::super::Foundation::PWSTR,
    pub dwHeadersLength: u32,
    pub dwHeadersTotal: u32,
    pub lpvBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
    pub dwBufferTotal: u32,
    pub dwOffsetLow: u32,
    pub dwOffsetHigh: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_BUFFERSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_BUFFERSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_BUFFERSW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_BUFFERSW")
            .field("dwStructSize", &self.dwStructSize)
            .field("Next", &self.Next)
            .field("lpcszHeader", &self.lpcszHeader)
            .field("dwHeadersLength", &self.dwHeadersLength)
            .field("dwHeadersTotal", &self.dwHeadersTotal)
            .field("lpvBuffer", &self.lpvBuffer)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBufferTotal", &self.dwBufferTotal)
            .field("dwOffsetLow", &self.dwOffsetLow)
            .field("dwOffsetHigh", &self.dwOffsetHigh)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_BUFFERSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.Next == other.Next && self.lpcszHeader == other.lpcszHeader && self.dwHeadersLength == other.dwHeadersLength && self.dwHeadersTotal == other.dwHeadersTotal && self.lpvBuffer == other.lpvBuffer && self.dwBufferLength == other.dwBufferLength && self.dwBufferTotal == other.dwBufferTotal && self.dwOffsetLow == other.dwOffsetLow && self.dwOffsetHigh == other.dwOffsetHigh
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_BUFFERSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_BUFFERSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOA {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: super::super::Foundation::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_CONFIG_INFOA_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOA_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOA_0_0 {
    pub CachePath: [super::super::Foundation::CHAR; 260],
    pub dwCacheSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOA_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOW {
    pub dwStructSize: u32,
    pub dwContainer: u32,
    pub dwQuota: u32,
    pub dwReserved4: u32,
    pub fPerUser: super::super::Foundation::BOOL,
    pub dwSyncMode: u32,
    pub dwNumCachePaths: u32,
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0,
    pub dwNormalUsage: u32,
    pub dwExemptUsage: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_CONFIG_INFOW_0 {
    pub Anonymous: INTERNET_CACHE_CONFIG_INFOW_0_0,
    pub CachePaths: [INTERNET_CACHE_CONFIG_PATH_ENTRYW; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_INFOW_0_0 {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_INFOW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_INFOW_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_INFOW_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    pub CachePath: [super::super::Foundation::CHAR; 260],
    pub dwCacheSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONFIG_PATH_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYA").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_PATH_ENTRYA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    pub CachePath: [u16; 260],
    pub dwCacheSize: u32,
}
impl INTERNET_CACHE_CONFIG_PATH_ENTRYW {}
impl ::core::default::Default for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_CONFIG_PATH_ENTRYW").field("CachePath", &self.CachePath).field("dwCacheSize", &self.dwCacheSize).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.CachePath == other.CachePath && self.dwCacheSize == other.dwCacheSize
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_CONFIG_PATH_ENTRYW {}
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONFIG_PATH_ENTRYW {
    type Abi = Self;
}
pub const INTERNET_CACHE_CONTAINER_AUTODELETE: u32 = 2u32;
pub const INTERNET_CACHE_CONTAINER_BLOOM_FILTER: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONTAINER_INFOA {
    pub dwCacheVersion: u32,
    pub lpszName: super::super::Foundation::PSTR,
    pub lpszCachePrefix: super::super::Foundation::PSTR,
    pub lpszVolumeLabel: super::super::Foundation::PSTR,
    pub lpszVolumeTitle: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONTAINER_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_CONTAINER_INFOA").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONTAINER_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_CONTAINER_INFOW {
    pub dwCacheVersion: u32,
    pub lpszName: super::super::Foundation::PWSTR,
    pub lpszCachePrefix: super::super::Foundation::PWSTR,
    pub lpszVolumeLabel: super::super::Foundation::PWSTR,
    pub lpszVolumeTitle: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_CONTAINER_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_CONTAINER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_CONTAINER_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_CONTAINER_INFOW").field("dwCacheVersion", &self.dwCacheVersion).field("lpszName", &self.lpszName).field("lpszCachePrefix", &self.lpszCachePrefix).field("lpszVolumeLabel", &self.lpszVolumeLabel).field("lpszVolumeTitle", &self.lpszVolumeTitle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_CONTAINER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwCacheVersion == other.dwCacheVersion && self.lpszName == other.lpszName && self.lpszCachePrefix == other.lpszCachePrefix && self.lpszVolumeLabel == other.lpszVolumeLabel && self.lpszVolumeTitle == other.lpszVolumeTitle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_CONTAINER_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_CONTAINER_INFOW {
    type Abi = Self;
}
pub const INTERNET_CACHE_CONTAINER_MAP_ENABLED: u32 = 16u32;
pub const INTERNET_CACHE_CONTAINER_NODESKTOPINIT: u32 = 8u32;
pub const INTERNET_CACHE_CONTAINER_NOSUBDIRS: u32 = 1u32;
pub const INTERNET_CACHE_CONTAINER_RESERVED1: u32 = 4u32;
pub const INTERNET_CACHE_CONTAINER_SHARE_READ: u32 = 256u32;
pub const INTERNET_CACHE_CONTAINER_SHARE_READ_WRITE: u32 = 768u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_ENTRY_INFOA {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: super::super::Foundation::PSTR,
    pub lpszLocalFileName: super::super::Foundation::PSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: super::super::Foundation::PSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: super::super::Foundation::PSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_ENTRY_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_ENTRY_INFOA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_ENTRY_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_ENTRY_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_ENTRY_INFOA_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_ENTRY_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_ENTRY_INFOA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_ENTRY_INFOA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_ENTRY_INFOA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_ENTRY_INFOW {
    pub dwStructSize: u32,
    pub lpszSourceUrlName: super::super::Foundation::PWSTR,
    pub lpszLocalFileName: super::super::Foundation::PWSTR,
    pub CacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub LastModifiedTime: super::super::Foundation::FILETIME,
    pub ExpireTime: super::super::Foundation::FILETIME,
    pub LastAccessTime: super::super::Foundation::FILETIME,
    pub LastSyncTime: super::super::Foundation::FILETIME,
    pub lpHeaderInfo: super::super::Foundation::PWSTR,
    pub dwHeaderInfoSize: u32,
    pub lpszFileExtension: super::super::Foundation::PWSTR,
    pub Anonymous: INTERNET_CACHE_ENTRY_INFOW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_ENTRY_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_ENTRY_INFOW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_ENTRY_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_ENTRY_INFOW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CACHE_ENTRY_INFOW_0 {
    pub dwReserved: u32,
    pub dwExemptDelta: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_ENTRY_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_ENTRY_INFOW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_ENTRY_INFOW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_ENTRY_INFOW_0 {
    type Abi = Self;
}
pub const INTERNET_CACHE_FLAG_ADD_FILENAME_ONLY: u32 = 2048u32;
pub const INTERNET_CACHE_FLAG_ALLOW_COLLISIONS: u32 = 256u32;
pub const INTERNET_CACHE_FLAG_ENTRY_OR_MAPPING: u32 = 1024u32;
pub const INTERNET_CACHE_FLAG_GET_STRUCT_ONLY: u32 = 4096u32;
pub const INTERNET_CACHE_FLAG_INSTALLED_ENTRY: u32 = 512u32;
pub const INTERNET_CACHE_GROUP_ADD: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_GROUP_INFOA {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [super::super::Foundation::CHAR; 120],
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_GROUP_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_GROUP_INFOA")
            .field("dwGroupSize", &self.dwGroupSize)
            .field("dwGroupFlags", &self.dwGroupFlags)
            .field("dwGroupType", &self.dwGroupType)
            .field("dwDiskUsage", &self.dwDiskUsage)
            .field("dwDiskQuota", &self.dwDiskQuota)
            .field("dwOwnerStorage", &self.dwOwnerStorage)
            .field("szGroupName", &self.szGroupName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_GROUP_INFOA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_CACHE_GROUP_INFOW {
    pub dwGroupSize: u32,
    pub dwGroupFlags: u32,
    pub dwGroupType: u32,
    pub dwDiskUsage: u32,
    pub dwDiskQuota: u32,
    pub dwOwnerStorage: [u32; 4],
    pub szGroupName: [u16; 120],
}
impl INTERNET_CACHE_GROUP_INFOW {}
impl ::core::default::Default for INTERNET_CACHE_GROUP_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_CACHE_GROUP_INFOW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_GROUP_INFOW")
            .field("dwGroupSize", &self.dwGroupSize)
            .field("dwGroupFlags", &self.dwGroupFlags)
            .field("dwGroupType", &self.dwGroupType)
            .field("dwDiskUsage", &self.dwDiskUsage)
            .field("dwDiskQuota", &self.dwDiskQuota)
            .field("dwOwnerStorage", &self.dwOwnerStorage)
            .field("szGroupName", &self.szGroupName)
            .finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_CACHE_GROUP_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwGroupSize == other.dwGroupSize && self.dwGroupFlags == other.dwGroupFlags && self.dwGroupType == other.dwGroupType && self.dwDiskUsage == other.dwDiskUsage && self.dwDiskQuota == other.dwDiskQuota && self.dwOwnerStorage == other.dwOwnerStorage && self.szGroupName == other.szGroupName
    }
}
impl ::core::cmp::Eq for INTERNET_CACHE_GROUP_INFOW {}
unsafe impl ::windows::core::Abi for INTERNET_CACHE_GROUP_INFOW {
    type Abi = Self;
}
pub const INTERNET_CACHE_GROUP_REMOVE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CACHE_TIMESTAMPS {
    pub ftExpires: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CACHE_TIMESTAMPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CACHE_TIMESTAMPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CACHE_TIMESTAMPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CACHE_TIMESTAMPS").field("ftExpires", &self.ftExpires).field("ftLastModified", &self.ftLastModified).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CACHE_TIMESTAMPS {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpires == other.ftExpires && self.ftLastModified == other.ftLastModified
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CACHE_TIMESTAMPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CACHE_TIMESTAMPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CALLBACK_COOKIE {
    pub pcwszName: super::super::Foundation::PWSTR,
    pub pcwszValue: super::super::Foundation::PWSTR,
    pub pcwszDomain: super::super::Foundation::PWSTR,
    pub pcwszPath: super::super::Foundation::PWSTR,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CALLBACK_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CALLBACK_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CALLBACK_COOKIE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CALLBACK_COOKIE").field("pcwszName", &self.pcwszName).field("pcwszValue", &self.pcwszValue).field("pcwszDomain", &self.pcwszDomain).field("pcwszPath", &self.pcwszPath).field("ftExpires", &self.ftExpires).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CALLBACK_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszName == other.pcwszName && self.pcwszValue == other.pcwszValue && self.pcwszDomain == other.pcwszDomain && self.pcwszPath == other.pcwszPath && self.ftExpires == other.ftExpires && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CALLBACK_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CALLBACK_COOKIE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CERTIFICATE_INFO {
    pub ftExpiry: super::super::Foundation::FILETIME,
    pub ftStart: super::super::Foundation::FILETIME,
    pub lpszSubjectInfo: *mut i8,
    pub lpszIssuerInfo: *mut i8,
    pub lpszProtocolName: *mut i8,
    pub lpszSignatureAlgName: *mut i8,
    pub lpszEncryptionAlgName: *mut i8,
    pub dwKeySize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CERTIFICATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CERTIFICATE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CERTIFICATE_INFO")
            .field("ftExpiry", &self.ftExpiry)
            .field("ftStart", &self.ftStart)
            .field("lpszSubjectInfo", &self.lpszSubjectInfo)
            .field("lpszIssuerInfo", &self.lpszIssuerInfo)
            .field("lpszProtocolName", &self.lpszProtocolName)
            .field("lpszSignatureAlgName", &self.lpszSignatureAlgName)
            .field("lpszEncryptionAlgName", &self.lpszEncryptionAlgName)
            .field("dwKeySize", &self.dwKeySize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CERTIFICATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ftExpiry == other.ftExpiry && self.ftStart == other.ftStart && self.lpszSubjectInfo == other.lpszSubjectInfo && self.lpszIssuerInfo == other.lpszIssuerInfo && self.lpszProtocolName == other.lpszProtocolName && self.lpszSignatureAlgName == other.lpszSignatureAlgName && self.lpszEncryptionAlgName == other.lpszEncryptionAlgName && self.dwKeySize == other.dwKeySize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CERTIFICATE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CERTIFICATE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_CONNECTED_INFO {
    pub dwConnectedState: INTERNET_STATE,
    pub dwFlags: u32,
}
impl INTERNET_CONNECTED_INFO {}
impl ::core::default::Default for INTERNET_CONNECTED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_CONNECTED_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_CONNECTED_INFO").field("dwConnectedState", &self.dwConnectedState).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_CONNECTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwConnectedState == other.dwConnectedState && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for INTERNET_CONNECTED_INFO {}
unsafe impl ::windows::core::Abi for INTERNET_CONNECTED_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_CONNECTION(pub u32);
pub const INTERNET_CONNECTION_CONFIGURED: INTERNET_CONNECTION = INTERNET_CONNECTION(64u32);
pub const INTERNET_CONNECTION_LAN_: INTERNET_CONNECTION = INTERNET_CONNECTION(2u32);
pub const INTERNET_CONNECTION_MODEM: INTERNET_CONNECTION = INTERNET_CONNECTION(1u32);
pub const INTERNET_CONNECTION_MODEM_BUSY: INTERNET_CONNECTION = INTERNET_CONNECTION(8u32);
pub const INTERNET_CONNECTION_OFFLINE_: INTERNET_CONNECTION = INTERNET_CONNECTION(32u32);
pub const INTERNET_CONNECTION_PROXY: INTERNET_CONNECTION = INTERNET_CONNECTION(4u32);
pub const INTERNET_RAS_INSTALLED: INTERNET_CONNECTION = INTERNET_CONNECTION(16u32);
impl ::core::convert::From<u32> for INTERNET_CONNECTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_CONNECTION {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_CONNECTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_CONNECTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_CONNECTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_CONNECTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_CONNECTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_CONNECTION_LAN: u32 = 2u32;
pub const INTERNET_CONNECTION_OFFLINE: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_COOKIE {
    pub cbSize: u32,
    pub pszName: super::super::Foundation::PSTR,
    pub pszData: super::super::Foundation::PSTR,
    pub pszDomain: super::super::Foundation::PSTR,
    pub pszPath: super::super::Foundation::PSTR,
    pub pftExpires: *mut super::super::Foundation::FILETIME,
    pub dwFlags: u32,
    pub pszUrl: super::super::Foundation::PSTR,
    pub pszP3PPolicy: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_COOKIE")
            .field("cbSize", &self.cbSize)
            .field("pszName", &self.pszName)
            .field("pszData", &self.pszData)
            .field("pszDomain", &self.pszDomain)
            .field("pszPath", &self.pszPath)
            .field("pftExpires", &self.pftExpires)
            .field("dwFlags", &self.dwFlags)
            .field("pszUrl", &self.pszUrl)
            .field("pszP3PPolicy", &self.pszP3PPolicy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pszName == other.pszName && self.pszData == other.pszData && self.pszDomain == other.pszDomain && self.pszPath == other.pszPath && self.pftExpires == other.pftExpires && self.dwFlags == other.dwFlags && self.pszUrl == other.pszUrl && self.pszP3PPolicy == other.pszP3PPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_COOKIE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_COOKIE2 {
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszValue: super::super::Foundation::PWSTR,
    pub pwszDomain: super::super::Foundation::PWSTR,
    pub pwszPath: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub fExpiresSet: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_COOKIE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_COOKIE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_COOKIE2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_COOKIE2").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).field("pwszDomain", &self.pwszDomain).field("pwszPath", &self.pwszPath).field("dwFlags", &self.dwFlags).field("ftExpires", &self.ftExpires).field("fExpiresSet", &self.fExpiresSet).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_COOKIE2 {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue && self.pwszDomain == other.pwszDomain && self.pwszPath == other.pwszPath && self.dwFlags == other.dwFlags && self.ftExpires == other.ftExpires && self.fExpiresSet == other.fExpiresSet
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_COOKIE2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_COOKIE2 {
    type Abi = Self;
}
pub const INTERNET_COOKIE_ALL_COOKIES: u32 = 536870912u32;
pub const INTERNET_COOKIE_APPLY_HOST_ONLY: u32 = 32768u32;
pub const INTERNET_COOKIE_APPLY_P3P: u32 = 128u32;
pub const INTERNET_COOKIE_ECTX_3RDPARTY: u32 = 2147483648u32;
pub const INTERNET_COOKIE_EDGE_COOKIES: u32 = 262144u32;
pub const INTERNET_COOKIE_EVALUATE_P3P: u32 = 64u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_COOKIE_FLAGS(pub u32);
pub const INTERNET_COOKIE_HTTPONLY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(8192u32);
pub const INTERNET_COOKIE_THIRD_PARTY: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(16u32);
pub const INTERNET_FLAG_RESTRICTED_ZONE: INTERNET_COOKIE_FLAGS = INTERNET_COOKIE_FLAGS(131072u32);
impl ::core::convert::From<u32> for INTERNET_COOKIE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_COOKIE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_COOKIE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_COOKIE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_COOKIE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_COOKIE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_COOKIE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_COOKIE_HOST_ONLY: u32 = 16384u32;
pub const INTERNET_COOKIE_HOST_ONLY_APPLIED: u32 = 524288u32;
pub const INTERNET_COOKIE_IE6: u32 = 1024u32;
pub const INTERNET_COOKIE_IS_LEGACY: u32 = 2048u32;
pub const INTERNET_COOKIE_IS_RESTRICTED: u32 = 512u32;
pub const INTERNET_COOKIE_IS_SECURE: u32 = 1u32;
pub const INTERNET_COOKIE_IS_SESSION: u32 = 2u32;
pub const INTERNET_COOKIE_NON_SCRIPT: u32 = 4096u32;
pub const INTERNET_COOKIE_NO_CALLBACK: u32 = 1073741824u32;
pub const INTERNET_COOKIE_P3P_ENABLED: u32 = 256u32;
pub const INTERNET_COOKIE_PERSISTENT_HOST_ONLY: u32 = 65536u32;
pub const INTERNET_COOKIE_PROMPT_REQUIRED: u32 = 32u32;
pub const INTERNET_COOKIE_RESTRICTED_ZONE: u32 = 131072u32;
pub const INTERNET_COOKIE_SAME_SITE_LAX: u32 = 2097152u32;
pub const INTERNET_COOKIE_SAME_SITE_LEVEL_CROSS_SITE: u32 = 4194304u32;
pub const INTERNET_COOKIE_SAME_SITE_STRICT: u32 = 1048576u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CREDENTIALS {
    pub lpcwszHostName: super::super::Foundation::PWSTR,
    pub dwPort: u32,
    pub dwScheme: u32,
    pub lpcwszUrl: super::super::Foundation::PWSTR,
    pub lpcwszRealm: super::super::Foundation::PWSTR,
    pub fAuthIdentity: super::super::Foundation::BOOL,
    pub Anonymous: INTERNET_CREDENTIALS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CREDENTIALS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CREDENTIALS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CREDENTIALS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_CREDENTIALS_0 {
    pub Anonymous: INTERNET_CREDENTIALS_0_0,
    pub pAuthIdentityOpaque: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CREDENTIALS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CREDENTIALS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CREDENTIALS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CREDENTIALS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_CREDENTIALS_0_0 {
    pub lpcwszUserName: super::super::Foundation::PWSTR,
    pub lpcwszPassword: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_CREDENTIALS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_CREDENTIALS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_CREDENTIALS_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("lpcwszUserName", &self.lpcwszUserName).field("lpcwszPassword", &self.lpcwszPassword).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_CREDENTIALS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.lpcwszUserName == other.lpcwszUserName && self.lpcwszPassword == other.lpcwszPassword
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_CREDENTIALS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_CREDENTIALS_0_0 {
    type Abi = Self;
}
pub const INTERNET_CUSTOMDIAL_CAN_HANGUP: u32 = 4u32;
pub const INTERNET_CUSTOMDIAL_CONNECT: u32 = 0u32;
pub const INTERNET_CUSTOMDIAL_DISCONNECT: u32 = 2u32;
pub const INTERNET_CUSTOMDIAL_SAFE_FOR_UNATTENDED: u32 = 1u32;
pub const INTERNET_CUSTOMDIAL_SHOWOFFLINE: u32 = 4u32;
pub const INTERNET_CUSTOMDIAL_UNATTENDED: u32 = 1u32;
pub const INTERNET_CUSTOMDIAL_WILL_SUPPLY_STATE: u32 = 2u32;
pub const INTERNET_DEFAULT_FTP_PORT: u32 = 21u32;
pub const INTERNET_DEFAULT_GOPHER_PORT: u32 = 70u32;
pub const INTERNET_DEFAULT_SOCKS_PORT: u32 = 1080u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_DIAGNOSTIC_SOCKET_INFO {
    pub Socket: usize,
    pub SourcePort: u32,
    pub DestPort: u32,
    pub Flags: u32,
}
impl INTERNET_DIAGNOSTIC_SOCKET_INFO {}
impl ::core::default::Default for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_DIAGNOSTIC_SOCKET_INFO").field("Socket", &self.Socket).field("SourcePort", &self.SourcePort).field("DestPort", &self.DestPort).field("Flags", &self.Flags).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Socket == other.Socket && self.SourcePort == other.SourcePort && self.DestPort == other.DestPort && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for INTERNET_DIAGNOSTIC_SOCKET_INFO {}
unsafe impl ::windows::core::Abi for INTERNET_DIAGNOSTIC_SOCKET_INFO {
    type Abi = Self;
}
pub const INTERNET_DIALSTATE_DISCONNECTED: u32 = 1u32;
pub const INTERNET_DIAL_FORCE_PROMPT: u32 = 8192u32;
pub const INTERNET_DIAL_SHOW_OFFLINE: u32 = 16384u32;
pub const INTERNET_DIAL_UNATTENDED: u32 = 32768u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_DOWNLOAD_MODE_HANDLE {
    pub pcwszFileName: super::super::Foundation::PWSTR,
    pub phFile: *mut super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_DOWNLOAD_MODE_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_DOWNLOAD_MODE_HANDLE").field("pcwszFileName", &self.pcwszFileName).field("phFile", &self.phFile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_DOWNLOAD_MODE_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.pcwszFileName == other.pcwszFileName && self.phFile == other.phFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_DOWNLOAD_MODE_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_DOWNLOAD_MODE_HANDLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_END_BROWSER_SESSION_DATA {
    pub lpBuffer: *mut ::core::ffi::c_void,
    pub dwBufferLength: u32,
}
impl INTERNET_END_BROWSER_SESSION_DATA {}
impl ::core::default::Default for INTERNET_END_BROWSER_SESSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_END_BROWSER_SESSION_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_END_BROWSER_SESSION_DATA").field("lpBuffer", &self.lpBuffer).field("dwBufferLength", &self.dwBufferLength).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_END_BROWSER_SESSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpBuffer == other.lpBuffer && self.dwBufferLength == other.dwBufferLength
    }
}
impl ::core::cmp::Eq for INTERNET_END_BROWSER_SESSION_DATA {}
unsafe impl ::windows::core::Abi for INTERNET_END_BROWSER_SESSION_DATA {
    type Abi = Self;
}
pub const INTERNET_ERROR_BASE: u32 = 12000u32;
pub const INTERNET_ERROR_LAST: u32 = 12192u32;
pub const INTERNET_ERROR_MASK_COMBINED_SEC_CERT: u32 = 2u32;
pub const INTERNET_ERROR_MASK_INSERT_CDROM: u32 = 1u32;
pub const INTERNET_ERROR_MASK_LOGIN_FAILURE_DISPLAY_ENTITY_BODY: u32 = 8u32;
pub const INTERNET_ERROR_MASK_NEED_MSN_SSPI_PKG: u32 = 4u32;
pub const INTERNET_FIRST_OPTION: u32 = 1u32;
pub const INTERNET_FLAG_ASYNC: u32 = 268435456u32;
pub const INTERNET_FLAG_BGUPDATE: u32 = 8u32;
pub const INTERNET_FLAG_CACHE_ASYNC: u32 = 128u32;
pub const INTERNET_FLAG_CACHE_IF_NET_FAIL: u32 = 65536u32;
pub const INTERNET_FLAG_DONT_CACHE: u32 = 67108864u32;
pub const INTERNET_FLAG_EXISTING_CONNECT: u32 = 536870912u32;
pub const INTERNET_FLAG_FORMS_SUBMIT: u32 = 64u32;
pub const INTERNET_FLAG_FROM_CACHE: u32 = 16777216u32;
pub const INTERNET_FLAG_FTP_FOLDER_VIEW: u32 = 4u32;
pub const INTERNET_FLAG_FWD_BACK: u32 = 32u32;
pub const INTERNET_FLAG_HYPERLINK: u32 = 1024u32;
pub const INTERNET_FLAG_IDN_DIRECT: u32 = 1u32;
pub const INTERNET_FLAG_IDN_PROXY: u32 = 2u32;
pub const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 4096u32;
pub const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 8192u32;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
pub const INTERNET_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
pub const INTERNET_FLAG_KEEP_CONNECTION: u32 = 4194304u32;
pub const INTERNET_FLAG_MAKE_PERSISTENT: u32 = 33554432u32;
pub const INTERNET_FLAG_MUST_CACHE_REQUEST: u32 = 16u32;
pub const INTERNET_FLAG_NEED_FILE: u32 = 16u32;
pub const INTERNET_FLAG_NO_AUTH: u32 = 262144u32;
pub const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 2097152u32;
pub const INTERNET_FLAG_NO_CACHE_WRITE: u32 = 67108864u32;
pub const INTERNET_FLAG_NO_COOKIES: u32 = 524288u32;
pub const INTERNET_FLAG_NO_UI: u32 = 512u32;
pub const INTERNET_FLAG_OFFLINE: u32 = 16777216u32;
pub const INTERNET_FLAG_PASSIVE: u32 = 134217728u32;
pub const INTERNET_FLAG_PRAGMA_NOCACHE: u32 = 256u32;
pub const INTERNET_FLAG_RAW_DATA: u32 = 1073741824u32;
pub const INTERNET_FLAG_READ_PREFETCH: u32 = 1048576u32;
pub const INTERNET_FLAG_RELOAD: u32 = 2147483648u32;
pub const INTERNET_FLAG_RESYNCHRONIZE: u32 = 2048u32;
pub const INTERNET_FLAG_SECURE: u32 = 8388608u32;
pub const INTERNET_GLOBAL_CALLBACK_SENDING_HTTP_HEADERS: u32 = 1u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_FTP: u32 = 2u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_GOPHER: u32 = 3u32;
pub const INTERNET_HANDLE_TYPE_CONNECT_HTTP: u32 = 4u32;
pub const INTERNET_HANDLE_TYPE_FILE_REQUEST: u32 = 14u32;
pub const INTERNET_HANDLE_TYPE_FTP_FILE: u32 = 7u32;
pub const INTERNET_HANDLE_TYPE_FTP_FILE_HTML: u32 = 8u32;
pub const INTERNET_HANDLE_TYPE_FTP_FIND: u32 = 5u32;
pub const INTERNET_HANDLE_TYPE_FTP_FIND_HTML: u32 = 6u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE: u32 = 11u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FILE_HTML: u32 = 12u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND: u32 = 9u32;
pub const INTERNET_HANDLE_TYPE_GOPHER_FIND_HTML: u32 = 10u32;
pub const INTERNET_HANDLE_TYPE_HTTP_REQUEST: u32 = 13u32;
pub const INTERNET_HANDLE_TYPE_INTERNET: u32 = 1u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_CONTENT: u32 = 32u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_COOKIES: u32 = 8u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_DATA: u32 = 4u32;
pub const INTERNET_IDENTITY_FLAG_CLEAR_HISTORY: u32 = 16u32;
pub const INTERNET_IDENTITY_FLAG_PRIVATE_CACHE: u32 = 1u32;
pub const INTERNET_IDENTITY_FLAG_SHARED_CACHE: u32 = 2u32;
pub const INTERNET_INTERNAL_ERROR_BASE: u32 = 12900u32;
pub const INTERNET_INVALID_PORT_NUMBER: u32 = 0u32;
pub const INTERNET_KEEP_ALIVE_DISABLED: u32 = 0u32;
pub const INTERNET_KEEP_ALIVE_ENABLED: u32 = 1u32;
pub const INTERNET_KEEP_ALIVE_UNKNOWN: u32 = 4294967295u32;
pub const INTERNET_LAST_OPTION: u32 = 187u32;
pub const INTERNET_LAST_OPTION_INTERNAL: u32 = 191u32;
pub const INTERNET_MAX_HOST_NAME_LENGTH: u32 = 256u32;
pub const INTERNET_MAX_PASSWORD_LENGTH: u32 = 128u32;
pub const INTERNET_MAX_PORT_NUMBER_LENGTH: u32 = 5u32;
pub const INTERNET_MAX_PORT_NUMBER_VALUE: u32 = 65535u32;
pub const INTERNET_MAX_USER_NAME_LENGTH: u32 = 128u32;
pub const INTERNET_NO_CALLBACK: u32 = 0u32;
pub const INTERNET_OPEN_TYPE_PRECONFIG_WITH_NO_AUTOPROXY: u32 = 4u32;
pub const INTERNET_OPTION_ACTIVATE_WORKER_THREADS: u32 = 92u32;
pub const INTERNET_OPTION_ACTIVITY_ID: u32 = 185u32;
pub const INTERNET_OPTION_ALLOW_FAILED_CONNECT_CONTENT: u32 = 110u32;
pub const INTERNET_OPTION_ALLOW_INSECURE_FALLBACK: u32 = 161u32;
pub const INTERNET_OPTION_ALTER_IDENTITY: u32 = 80u32;
pub const INTERNET_OPTION_APP_CACHE: u32 = 130u32;
pub const INTERNET_OPTION_ASYNC: u32 = 30u32;
pub const INTERNET_OPTION_ASYNC_ID: u32 = 15u32;
pub const INTERNET_OPTION_ASYNC_PRIORITY: u32 = 16u32;
pub const INTERNET_OPTION_AUTH_FLAGS: u32 = 85u32;
pub const INTERNET_OPTION_AUTH_SCHEME_SELECTED: u32 = 183u32;
pub const INTERNET_OPTION_AUTODIAL_CONNECTION: u32 = 83u32;
pub const INTERNET_OPTION_AUTODIAL_HWND: u32 = 112u32;
pub const INTERNET_OPTION_AUTODIAL_MODE: u32 = 82u32;
pub const INTERNET_OPTION_BACKGROUND_CONNECTIONS: u32 = 121u32;
pub const INTERNET_OPTION_BYPASS_EDITED_ENTRY: u32 = 64u32;
pub const INTERNET_OPTION_CACHE_ENTRY_EXTRA_DATA: u32 = 139u32;
pub const INTERNET_OPTION_CACHE_PARTITION: u32 = 111u32;
pub const INTERNET_OPTION_CACHE_STREAM_HANDLE: u32 = 27u32;
pub const INTERNET_OPTION_CACHE_TIMESTAMPS: u32 = 69u32;
pub const INTERNET_OPTION_CALLBACK: u32 = 1u32;
pub const INTERNET_OPTION_CALLBACK_FILTER: u32 = 54u32;
pub const INTERNET_OPTION_CANCEL_CACHE_WRITE: u32 = 182u32;
pub const INTERNET_OPTION_CERT_ERROR_FLAGS: u32 = 98u32;
pub const INTERNET_OPTION_CHUNK_ENCODE_REQUEST: u32 = 150u32;
pub const INTERNET_OPTION_CLIENT_CERT_CONTEXT: u32 = 84u32;
pub const INTERNET_OPTION_CLIENT_CERT_ISSUER_LIST: u32 = 153u32;
pub const INTERNET_OPTION_CM_HANDLE_COPY_REF: u32 = 118u32;
pub const INTERNET_OPTION_CODEPAGE: u32 = 68u32;
pub const INTERNET_OPTION_CODEPAGE_EXTRA: u32 = 101u32;
pub const INTERNET_OPTION_CODEPAGE_PATH: u32 = 100u32;
pub const INTERNET_OPTION_COMPRESSED_CONTENT_LENGTH: u32 = 147u32;
pub const INTERNET_OPTION_CONNECTED_STATE: u32 = 50u32;
pub const INTERNET_OPTION_CONNECTION_FILTER: u32 = 162u32;
pub const INTERNET_OPTION_CONNECTION_INFO: u32 = 120u32;
pub const INTERNET_OPTION_CONNECT_BACKOFF: u32 = 4u32;
pub const INTERNET_OPTION_CONNECT_LIMIT: u32 = 46u32;
pub const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3u32;
pub const INTERNET_OPTION_CONNECT_TIME: u32 = 55u32;
pub const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2u32;
pub const INTERNET_OPTION_CONTEXT_VALUE: u32 = 45u32;
pub const INTERNET_OPTION_CONTEXT_VALUE_OLD: u32 = 10u32;
pub const INTERNET_OPTION_CONTROL_RECEIVE_TIMEOUT: u32 = 6u32;
pub const INTERNET_OPTION_CONTROL_SEND_TIMEOUT: u32 = 5u32;
pub const INTERNET_OPTION_COOKIES_3RD_PARTY: u32 = 86u32;
pub const INTERNET_OPTION_COOKIES_APPLY_HOST_ONLY: u32 = 179u32;
pub const INTERNET_OPTION_COOKIES_SAME_SITE_LEVEL: u32 = 187u32;
pub const INTERNET_OPTION_DATAFILE_EXT: u32 = 96u32;
pub const INTERNET_OPTION_DATAFILE_NAME: u32 = 33u32;
pub const INTERNET_OPTION_DATA_RECEIVE_TIMEOUT: u32 = 8u32;
pub const INTERNET_OPTION_DATA_SEND_TIMEOUT: u32 = 7u32;
pub const INTERNET_OPTION_DEPENDENCY_HANDLE: u32 = 131u32;
pub const INTERNET_OPTION_DETECT_POST_SEND: u32 = 71u32;
pub const INTERNET_OPTION_DIAGNOSTIC_SOCKET_INFO: u32 = 67u32;
pub const INTERNET_OPTION_DIGEST_AUTH_UNLOAD: u32 = 76u32;
pub const INTERNET_OPTION_DISABLE_AUTODIAL: u32 = 70u32;
pub const INTERNET_OPTION_DISABLE_INSECURE_FALLBACK: u32 = 160u32;
pub const INTERNET_OPTION_DISABLE_NTLM_PREAUTH: u32 = 72u32;
pub const INTERNET_OPTION_DISABLE_PASSPORT_AUTH: u32 = 87u32;
pub const INTERNET_OPTION_DISABLE_PROXY_LINK_LOCAL_NAME_RESOLUTION: u32 = 190u32;
pub const INTERNET_OPTION_DISALLOW_PREMATURE_EOF: u32 = 137u32;
pub const INTERNET_OPTION_DISCONNECTED_TIMEOUT: u32 = 49u32;
pub const INTERNET_OPTION_DOWNLOAD_MODE: u32 = 116u32;
pub const INTERNET_OPTION_DOWNLOAD_MODE_HANDLE: u32 = 165u32;
pub const INTERNET_OPTION_DO_NOT_TRACK: u32 = 123u32;
pub const INTERNET_OPTION_DUO_USED: u32 = 149u32;
pub const INTERNET_OPTION_EDGE_COOKIES: u32 = 166u32;
pub const INTERNET_OPTION_EDGE_COOKIES_TEMP: u32 = 175u32;
pub const INTERNET_OPTION_EDGE_MODE: u32 = 180u32;
pub const INTERNET_OPTION_ENABLE_DUO: u32 = 148u32;
pub const INTERNET_OPTION_ENABLE_HEADER_CALLBACKS: u32 = 168u32;
pub const INTERNET_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 148u32;
pub const INTERNET_OPTION_ENABLE_PASSPORT_AUTH: u32 = 90u32;
pub const INTERNET_OPTION_ENABLE_REDIRECT_CACHE_READ: u32 = 122u32;
pub const INTERNET_OPTION_ENABLE_TEST_SIGNING: u32 = 189u32;
pub const INTERNET_OPTION_ENABLE_WBOEXT: u32 = 158u32;
pub const INTERNET_OPTION_ENABLE_ZLIB_DEFLATE: u32 = 173u32;
pub const INTERNET_OPTION_ENCODE_EXTRA: u32 = 155u32;
pub const INTERNET_OPTION_ENCODE_FALLBACK_FOR_REDIRECT_URI: u32 = 174u32;
pub const INTERNET_OPTION_END_BROWSER_SESSION: u32 = 42u32;
pub const INTERNET_OPTION_ENTERPRISE_CONTEXT: u32 = 159u32;
pub const INTERNET_OPTION_ERROR_MASK: u32 = 62u32;
pub const INTERNET_OPTION_EXEMPT_CONNECTION_LIMIT: u32 = 89u32;
pub const INTERNET_OPTION_EXTENDED_CALLBACKS: u32 = 108u32;
pub const INTERNET_OPTION_EXTENDED_ERROR: u32 = 24u32;
pub const INTERNET_OPTION_FAIL_ON_CACHE_WRITE_ERROR: u32 = 115u32;
pub const INTERNET_OPTION_FALSE_START: u32 = 141u32;
pub const INTERNET_OPTION_FLUSH_STATE: u32 = 135u32;
pub const INTERNET_OPTION_FORCE_DECODE: u32 = 178u32;
pub const INTERNET_OPTION_FROM_CACHE_TIMEOUT: u32 = 63u32;
pub const INTERNET_OPTION_GLOBAL_CALLBACK: u32 = 188u32;
pub const INTERNET_OPTION_HANDLE_TYPE: u32 = 9u32;
pub const INTERNET_OPTION_HIBERNATE_INACTIVE_WORKER_THREADS: u32 = 91u32;
pub const INTERNET_OPTION_HSTS: u32 = 157u32;
pub const INTERNET_OPTION_HTTP_09: u32 = 191u32;
pub const INTERNET_OPTION_HTTP_DECODING: u32 = 65u32;
pub const INTERNET_OPTION_HTTP_PROTOCOL_USED: u32 = 149u32;
pub const INTERNET_OPTION_HTTP_VERSION: u32 = 59u32;
pub const INTERNET_OPTION_IDENTITY: u32 = 78u32;
pub const INTERNET_OPTION_IDLE_STATE: u32 = 51u32;
pub const INTERNET_OPTION_IDN: u32 = 102u32;
pub const INTERNET_OPTION_IGNORE_CERT_ERROR_FLAGS: u32 = 99u32;
pub const INTERNET_OPTION_IGNORE_OFFLINE: u32 = 77u32;
pub const INTERNET_OPTION_KEEP_CONNECTION: u32 = 22u32;
pub const INTERNET_OPTION_LINE_STATE: u32 = 50u32;
pub const INTERNET_OPTION_LISTEN_TIMEOUT: u32 = 11u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_1_0_SERVER: u32 = 74u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_PROXY: u32 = 103u32;
pub const INTERNET_OPTION_MAX_CONNS_PER_SERVER: u32 = 73u32;
pub const INTERNET_OPTION_MAX_QUERY_BUFFER_SIZE: u32 = 140u32;
pub const INTERNET_OPTION_NET_SPEED: u32 = 61u32;
pub const INTERNET_OPTION_NOCACHE_WRITE_IN_PRIVATE: u32 = 184u32;
pub const INTERNET_OPTION_NOTIFY_SENDING_COOKIE: u32 = 152u32;
pub const INTERNET_OPTION_NO_HTTP_SERVER_AUTH: u32 = 167u32;
pub const INTERNET_OPTION_OFFLINE_MODE: u32 = 26u32;
pub const INTERNET_OPTION_OFFLINE_SEMANTICS: u32 = 52u32;
pub const INTERNET_OPTION_OFFLINE_TIMEOUT: u32 = 49u32;
pub const INTERNET_OPTION_OPT_IN_WEAK_SIGNATURE: u32 = 176u32;
pub const INTERNET_OPTION_ORIGINAL_CONNECT_FLAGS: u32 = 97u32;
pub const INTERNET_OPTION_PARENT_HANDLE: u32 = 21u32;
pub const INTERNET_OPTION_PARSE_LINE_FOLDING: u32 = 177u32;
pub const INTERNET_OPTION_PASSWORD: u32 = 29u32;
pub const INTERNET_OPTION_PER_CONNECTION_OPTION: u32 = 75u32;
pub const INTERNET_OPTION_POLICY: u32 = 48u32;
pub const INTERNET_OPTION_PRESERVE_REFERER_ON_HTTPS_TO_HTTP_REDIRECT: u32 = 170u32;
pub const INTERNET_OPTION_PRESERVE_REQUEST_SERVER_CREDENTIALS_ON_REDIRECT: u32 = 169u32;
pub const INTERNET_OPTION_PROXY: u32 = 38u32;
pub const INTERNET_OPTION_PROXY_AUTH_SCHEME: u32 = 144u32;
pub const INTERNET_OPTION_PROXY_CREDENTIALS: u32 = 107u32;
pub const INTERNET_OPTION_PROXY_FROM_REQUEST: u32 = 109u32;
pub const INTERNET_OPTION_PROXY_PASSWORD: u32 = 44u32;
pub const INTERNET_OPTION_PROXY_SETTINGS_CHANGED: u32 = 95u32;
pub const INTERNET_OPTION_PROXY_USERNAME: u32 = 43u32;
pub const INTERNET_OPTION_READ_BUFFER_SIZE: u32 = 12u32;
pub const INTERNET_OPTION_RECEIVE_THROUGHPUT: u32 = 57u32;
pub const INTERNET_OPTION_RECEIVE_TIMEOUT: u32 = 6u32;
pub const INTERNET_OPTION_REFERER_TOKEN_BINDING_HOSTNAME: u32 = 163u32;
pub const INTERNET_OPTION_REFRESH: u32 = 37u32;
pub const INTERNET_OPTION_REMOVE_IDENTITY: u32 = 79u32;
pub const INTERNET_OPTION_REQUEST_FLAGS: u32 = 23u32;
pub const INTERNET_OPTION_REQUEST_PRIORITY: u32 = 58u32;
pub const INTERNET_OPTION_REQUEST_TIMES: u32 = 186u32;
pub const INTERNET_OPTION_RESET: u32 = 154u32;
pub const INTERNET_OPTION_RESET_URLCACHE_SESSION: u32 = 60u32;
pub const INTERNET_OPTION_RESPONSE_RESUMABLE: u32 = 117u32;
pub const INTERNET_OPTION_RESTORE_WORKER_THREAD_DEFAULTS: u32 = 93u32;
pub const INTERNET_OPTION_SECONDARY_CACHE_KEY: u32 = 53u32;
pub const INTERNET_OPTION_SECURE_FAILURE: u32 = 151u32;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE: u32 = 35u32;
pub const INTERNET_OPTION_SECURITY_CERTIFICATE_STRUCT: u32 = 32u32;
pub const INTERNET_OPTION_SECURITY_CONNECTION_INFO: u32 = 66u32;
pub const INTERNET_OPTION_SECURITY_FLAGS: u32 = 31u32;
pub const INTERNET_OPTION_SECURITY_KEY_BITNESS: u32 = 36u32;
pub const INTERNET_OPTION_SECURITY_SELECT_CLIENT_CERT: u32 = 47u32;
pub const INTERNET_OPTION_SEND_THROUGHPUT: u32 = 56u32;
pub const INTERNET_OPTION_SEND_TIMEOUT: u32 = 5u32;
pub const INTERNET_OPTION_SEND_UTF8_SERVERNAME_TO_PROXY: u32 = 88u32;
pub const INTERNET_OPTION_SERVER_ADDRESS_INFO: u32 = 156u32;
pub const INTERNET_OPTION_SERVER_AUTH_SCHEME: u32 = 143u32;
pub const INTERNET_OPTION_SERVER_CERT_CHAIN_CONTEXT: u32 = 105u32;
pub const INTERNET_OPTION_SERVER_CREDENTIALS: u32 = 113u32;
pub const INTERNET_OPTION_SESSION_START_TIME: u32 = 106u32;
pub const INTERNET_OPTION_SETTINGS_CHANGED: u32 = 39u32;
pub const INTERNET_OPTION_SET_IN_PRIVATE: u32 = 164u32;
pub const INTERNET_OPTION_SOCKET_NODELAY: u32 = 129u32;
pub const INTERNET_OPTION_SOCKET_NOTIFICATION_IOCTL: u32 = 138u32;
pub const INTERNET_OPTION_SOCKET_SEND_BUFFER_LENGTH: u32 = 94u32;
pub const INTERNET_OPTION_SOURCE_PORT: u32 = 146u32;
pub const INTERNET_OPTION_SUPPRESS_BEHAVIOR: u32 = 81u32;
pub const INTERNET_OPTION_SUPPRESS_SERVER_AUTH: u32 = 104u32;
pub const INTERNET_OPTION_SYNC_MODE_AUTOMATIC_SESSION_DISABLED: u32 = 172u32;
pub const INTERNET_OPTION_TCP_FAST_OPEN: u32 = 171u32;
pub const INTERNET_OPTION_TIMED_CONNECTION_LIMIT_BYPASS: u32 = 133u32;
pub const INTERNET_OPTION_TOKEN_BINDING_PUBLIC_KEY: u32 = 181u32;
pub const INTERNET_OPTION_TUNNEL_ONLY: u32 = 145u32;
pub const INTERNET_OPTION_UNLOAD_NOTIFY_EVENT: u32 = 128u32;
pub const INTERNET_OPTION_UPGRADE_TO_WEB_SOCKET: u32 = 126u32;
pub const INTERNET_OPTION_URL: u32 = 34u32;
pub const INTERNET_OPTION_USERNAME: u32 = 28u32;
pub const INTERNET_OPTION_USER_AGENT: u32 = 41u32;
pub const INTERNET_OPTION_USER_PASS_SERVER_ONLY: u32 = 142u32;
pub const INTERNET_OPTION_USE_FIRST_AVAILABLE_CONNECTION: u32 = 132u32;
pub const INTERNET_OPTION_USE_MODIFIED_HEADER_FILTER: u32 = 124u32;
pub const INTERNET_OPTION_VERSION: u32 = 40u32;
pub const INTERNET_OPTION_WEB_SOCKET_CLOSE_TIMEOUT: u32 = 134u32;
pub const INTERNET_OPTION_WEB_SOCKET_KEEPALIVE_INTERVAL: u32 = 127u32;
pub const INTERNET_OPTION_WPAD_SLEEP: u32 = 114u32;
pub const INTERNET_OPTION_WRITE_BUFFER_SIZE: u32 = 13u32;
pub const INTERNET_OPTION_WWA_MODE: u32 = 125u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_PER_CONN(pub u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(4u32);
pub const INTERNET_PER_CONN_AUTODISCOVERY_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(5u32);
pub const INTERNET_PER_CONN_FLAGS: INTERNET_PER_CONN = INTERNET_PER_CONN(1u32);
pub const INTERNET_PER_CONN_PROXY_BYPASS: INTERNET_PER_CONN = INTERNET_PER_CONN(3u32);
pub const INTERNET_PER_CONN_PROXY_SERVER: INTERNET_PER_CONN = INTERNET_PER_CONN(2u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_SECONDARY_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(6u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_RELOAD_DELAY_MINS: INTERNET_PER_CONN = INTERNET_PER_CONN(7u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_TIME: INTERNET_PER_CONN = INTERNET_PER_CONN(8u32);
pub const INTERNET_PER_CONN_AUTOCONFIG_LAST_DETECT_URL: INTERNET_PER_CONN = INTERNET_PER_CONN(9u32);
impl ::core::convert::From<u32> for INTERNET_PER_CONN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_PER_CONN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_PER_CONN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_PER_CONN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_PER_CONN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_PER_CONN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_PER_CONN_FLAGS_UI: u32 = 10u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTIONA {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTIONA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTIONA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_PER_CONN_OPTIONA_0 {
    pub dwValue: u32,
    pub pszValue: super::super::Foundation::PSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTIONA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTIONA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTIONA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTIONW {
    pub dwOption: INTERNET_PER_CONN,
    pub Value: INTERNET_PER_CONN_OPTIONW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTIONW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTIONW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INTERNET_PER_CONN_OPTIONW_0 {
    pub dwValue: u32,
    pub pszValue: super::super::Foundation::PWSTR,
    pub ftValue: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTIONW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTIONW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTIONW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTIONW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTION_LISTA {
    pub dwSize: u32,
    pub pszConnection: super::super::Foundation::PSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTION_LISTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_PER_CONN_OPTION_LISTA").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTION_LISTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_PER_CONN_OPTION_LISTW {
    pub dwSize: u32,
    pub pszConnection: super::super::Foundation::PWSTR,
    pub dwOptionCount: u32,
    pub dwOptionError: u32,
    pub pOptions: *mut INTERNET_PER_CONN_OPTIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_PER_CONN_OPTION_LISTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_PER_CONN_OPTION_LISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_PER_CONN_OPTION_LISTW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_PER_CONN_OPTION_LISTW").field("dwSize", &self.dwSize).field("pszConnection", &self.pszConnection).field("dwOptionCount", &self.dwOptionCount).field("dwOptionError", &self.dwOptionError).field("pOptions", &self.pOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_PER_CONN_OPTION_LISTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pszConnection == other.pszConnection && self.dwOptionCount == other.dwOptionCount && self.dwOptionError == other.dwOptionError && self.pOptions == other.pOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_PER_CONN_OPTION_LISTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_PER_CONN_OPTION_LISTW {
    type Abi = Self;
}
pub const INTERNET_PREFETCH_ABORTED: u32 = 2u32;
pub const INTERNET_PREFETCH_COMPLETE: u32 = 1u32;
pub const INTERNET_PREFETCH_PROGRESS: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_PREFETCH_STATUS {
    pub dwStatus: u32,
    pub dwSize: u32,
}
impl INTERNET_PREFETCH_STATUS {}
impl ::core::default::Default for INTERNET_PREFETCH_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_PREFETCH_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_PREFETCH_STATUS").field("dwStatus", &self.dwStatus).field("dwSize", &self.dwSize).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_PREFETCH_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwStatus == other.dwStatus && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for INTERNET_PREFETCH_STATUS {}
unsafe impl ::windows::core::Abi for INTERNET_PREFETCH_STATUS {
    type Abi = Self;
}
pub const INTERNET_PRIORITY_FOREGROUND: u32 = 1000u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_PROXY_INFO {
    pub dwAccessType: INTERNET_ACCESS_TYPE,
    pub lpszProxy: *mut i8,
    pub lpszProxyBypass: *mut i8,
}
impl INTERNET_PROXY_INFO {}
impl ::core::default::Default for INTERNET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_PROXY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_PROXY_INFO").field("dwAccessType", &self.dwAccessType).field("lpszProxy", &self.lpszProxy).field("lpszProxyBypass", &self.lpszProxyBypass).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwAccessType == other.dwAccessType && self.lpszProxy == other.lpszProxy && self.lpszProxyBypass == other.lpszProxyBypass
    }
}
impl ::core::cmp::Eq for INTERNET_PROXY_INFO {}
unsafe impl ::windows::core::Abi for INTERNET_PROXY_INFO {
    type Abi = Self;
}
pub const INTERNET_REQFLAG_ASYNC: u32 = 2u32;
pub const INTERNET_REQFLAG_CACHE_WRITE_DISABLED: u32 = 64u32;
pub const INTERNET_REQFLAG_FROM_APP_CACHE: u32 = 256u32;
pub const INTERNET_REQFLAG_FROM_CACHE: u32 = 1u32;
pub const INTERNET_REQFLAG_NET_TIMEOUT: u32 = 128u32;
pub const INTERNET_REQFLAG_NO_HEADERS: u32 = 8u32;
pub const INTERNET_REQFLAG_PASSIVE: u32 = 16u32;
pub const INTERNET_REQFLAG_VIA_PROXY: u32 = 4u32;
pub const INTERNET_RFC1123_BUFSIZE: u32 = 30u32;
pub const INTERNET_RFC1123_FORMAT: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_SCHEME(pub i32);
pub const INTERNET_SCHEME_PARTIAL: INTERNET_SCHEME = INTERNET_SCHEME(-2i32);
pub const INTERNET_SCHEME_UNKNOWN: INTERNET_SCHEME = INTERNET_SCHEME(-1i32);
pub const INTERNET_SCHEME_DEFAULT: INTERNET_SCHEME = INTERNET_SCHEME(0i32);
pub const INTERNET_SCHEME_FTP: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
pub const INTERNET_SCHEME_GOPHER: INTERNET_SCHEME = INTERNET_SCHEME(2i32);
pub const INTERNET_SCHEME_HTTP: INTERNET_SCHEME = INTERNET_SCHEME(3i32);
pub const INTERNET_SCHEME_HTTPS: INTERNET_SCHEME = INTERNET_SCHEME(4i32);
pub const INTERNET_SCHEME_FILE: INTERNET_SCHEME = INTERNET_SCHEME(5i32);
pub const INTERNET_SCHEME_NEWS: INTERNET_SCHEME = INTERNET_SCHEME(6i32);
pub const INTERNET_SCHEME_MAILTO: INTERNET_SCHEME = INTERNET_SCHEME(7i32);
pub const INTERNET_SCHEME_SOCKS: INTERNET_SCHEME = INTERNET_SCHEME(8i32);
pub const INTERNET_SCHEME_JAVASCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(9i32);
pub const INTERNET_SCHEME_VBSCRIPT: INTERNET_SCHEME = INTERNET_SCHEME(10i32);
pub const INTERNET_SCHEME_RES: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
pub const INTERNET_SCHEME_FIRST: INTERNET_SCHEME = INTERNET_SCHEME(1i32);
pub const INTERNET_SCHEME_LAST: INTERNET_SCHEME = INTERNET_SCHEME(11i32);
impl ::core::convert::From<i32> for INTERNET_SCHEME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_SCHEME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
pub struct INTERNET_SECURITY_CONNECTION_INFO {
    pub dwSize: u32,
    pub fSecure: super::super::Foundation::BOOL,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl INTERNET_SECURITY_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::default::Default for INTERNET_SECURITY_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_CONNECTION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_SECURITY_CONNECTION_INFO").field("dwSize", &self.dwSize).field("fSecure", &self.fSecure).field("connectionInfo", &self.connectionInfo).field("cipherInfo", &self.cipherInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_CONNECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fSecure == other.fSecure && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_CONNECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity"))]
unsafe impl ::windows::core::Abi for INTERNET_SECURITY_CONNECTION_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub struct INTERNET_SECURITY_INFO {
    pub dwSize: u32,
    pub pCertificate: *mut super::super::Security::Cryptography::CERT_CONTEXT,
    pub pcCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub connectionInfo: super::super::Security::Authentication::Identity::SecPkgContext_ConnectionInfo,
    pub cipherInfo: super::super::Security::Authentication::Identity::SecPkgContext_CipherInfo,
    pub pcUnverifiedCertChain: *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT,
    pub channelBindingToken: super::super::Security::Authentication::Identity::SecPkgContext_Bindings,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl INTERNET_SECURITY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for INTERNET_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for INTERNET_SECURITY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_SECURITY_INFO")
            .field("dwSize", &self.dwSize)
            .field("pCertificate", &self.pCertificate)
            .field("pcCertChain", &self.pcCertChain)
            .field("connectionInfo", &self.connectionInfo)
            .field("cipherInfo", &self.cipherInfo)
            .field("pcUnverifiedCertChain", &self.pcUnverifiedCertChain)
            .field("channelBindingToken", &self.channelBindingToken)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for INTERNET_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pCertificate == other.pCertificate && self.pcCertChain == other.pcCertChain && self.connectionInfo == other.connectionInfo && self.cipherInfo == other.cipherInfo && self.pcUnverifiedCertChain == other.pcUnverifiedCertChain && self.channelBindingToken == other.channelBindingToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for INTERNET_SECURITY_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for INTERNET_SECURITY_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INTERNET_SERVER_CONNECTION_STATE {
    pub lpcwszHostName: super::super::Foundation::PWSTR,
    pub fProxy: super::super::Foundation::BOOL,
    pub dwCounter: u32,
    pub dwConnectionLimit: u32,
    pub dwAvailableCreates: u32,
    pub dwAvailableKeepAlives: u32,
    pub dwActiveConnections: u32,
    pub dwWaiters: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INTERNET_SERVER_CONNECTION_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INTERNET_SERVER_CONNECTION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INTERNET_SERVER_CONNECTION_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_SERVER_CONNECTION_STATE")
            .field("lpcwszHostName", &self.lpcwszHostName)
            .field("fProxy", &self.fProxy)
            .field("dwCounter", &self.dwCounter)
            .field("dwConnectionLimit", &self.dwConnectionLimit)
            .field("dwAvailableCreates", &self.dwAvailableCreates)
            .field("dwAvailableKeepAlives", &self.dwAvailableKeepAlives)
            .field("dwActiveConnections", &self.dwActiveConnections)
            .field("dwWaiters", &self.dwWaiters)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INTERNET_SERVER_CONNECTION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.lpcwszHostName == other.lpcwszHostName && self.fProxy == other.fProxy && self.dwCounter == other.dwCounter && self.dwConnectionLimit == other.dwConnectionLimit && self.dwAvailableCreates == other.dwAvailableCreates && self.dwAvailableKeepAlives == other.dwAvailableKeepAlives && self.dwActiveConnections == other.dwActiveConnections && self.dwWaiters == other.dwWaiters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INTERNET_SERVER_CONNECTION_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for INTERNET_SERVER_CONNECTION_STATE {
    type Abi = Self;
}
pub const INTERNET_SERVICE_FTP: u32 = 1u32;
pub const INTERNET_SERVICE_GOPHER: u32 = 2u32;
pub const INTERNET_SERVICE_HTTP: u32 = 3u32;
pub const INTERNET_SERVICE_URL: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct INTERNET_STATE(pub u32);
pub const INTERNET_STATE_CONNECTED: INTERNET_STATE = INTERNET_STATE(1u32);
pub const INTERNET_STATE_DISCONNECTED: INTERNET_STATE = INTERNET_STATE(2u32);
pub const INTERNET_STATE_DISCONNECTED_BY_USER: INTERNET_STATE = INTERNET_STATE(16u32);
pub const INTERNET_STATE_IDLE: INTERNET_STATE = INTERNET_STATE(256u32);
pub const INTERNET_STATE_BUSY: INTERNET_STATE = INTERNET_STATE(512u32);
impl ::core::convert::From<u32> for INTERNET_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for INTERNET_STATE {
    type Abi = Self;
}
impl ::core::ops::BitOr for INTERNET_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for INTERNET_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for INTERNET_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for INTERNET_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for INTERNET_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const INTERNET_STATUS_CLOSING_CONNECTION: u32 = 50u32;
pub const INTERNET_STATUS_CONNECTED_TO_SERVER: u32 = 21u32;
pub const INTERNET_STATUS_CONNECTING_TO_SERVER: u32 = 20u32;
pub const INTERNET_STATUS_CONNECTION_CLOSED: u32 = 51u32;
pub const INTERNET_STATUS_COOKIE: u32 = 430u32;
pub const INTERNET_STATUS_COOKIE_HISTORY: u32 = 327u32;
pub const INTERNET_STATUS_COOKIE_RECEIVED: u32 = 321u32;
pub const INTERNET_STATUS_COOKIE_SENT: u32 = 320u32;
pub const INTERNET_STATUS_CTL_RESPONSE_RECEIVED: u32 = 42u32;
pub const INTERNET_STATUS_DETECTING_PROXY: u32 = 80u32;
pub const INTERNET_STATUS_END_BROWSER_SESSION: u32 = 420u32;
pub const INTERNET_STATUS_FILTER_CLOSED: u32 = 512u32;
pub const INTERNET_STATUS_FILTER_CLOSING: u32 = 256u32;
pub const INTERNET_STATUS_FILTER_CONNECTED: u32 = 8u32;
pub const INTERNET_STATUS_FILTER_CONNECTING: u32 = 4u32;
pub const INTERNET_STATUS_FILTER_HANDLE_CLOSING: u32 = 2048u32;
pub const INTERNET_STATUS_FILTER_HANDLE_CREATED: u32 = 1024u32;
pub const INTERNET_STATUS_FILTER_PREFETCH: u32 = 4096u32;
pub const INTERNET_STATUS_FILTER_RECEIVED: u32 = 128u32;
pub const INTERNET_STATUS_FILTER_RECEIVING: u32 = 64u32;
pub const INTERNET_STATUS_FILTER_REDIRECT: u32 = 8192u32;
pub const INTERNET_STATUS_FILTER_RESOLVED: u32 = 2u32;
pub const INTERNET_STATUS_FILTER_RESOLVING: u32 = 1u32;
pub const INTERNET_STATUS_FILTER_SENDING: u32 = 16u32;
pub const INTERNET_STATUS_FILTER_SENT: u32 = 32u32;
pub const INTERNET_STATUS_FILTER_STATE_CHANGE: u32 = 16384u32;
pub const INTERNET_STATUS_HANDLE_CLOSING: u32 = 70u32;
pub const INTERNET_STATUS_HANDLE_CREATED: u32 = 60u32;
pub const INTERNET_STATUS_INTERMEDIATE_RESPONSE: u32 = 120u32;
pub const INTERNET_STATUS_NAME_RESOLVED: u32 = 11u32;
pub const INTERNET_STATUS_P3P_HEADER: u32 = 325u32;
pub const INTERNET_STATUS_P3P_POLICYREF: u32 = 326u32;
pub const INTERNET_STATUS_PREFETCH: u32 = 43u32;
pub const INTERNET_STATUS_PRIVACY_IMPACTED: u32 = 324u32;
pub const INTERNET_STATUS_PROXY_CREDENTIALS: u32 = 400u32;
pub const INTERNET_STATUS_RECEIVING_RESPONSE: u32 = 40u32;
pub const INTERNET_STATUS_REDIRECT: u32 = 110u32;
pub const INTERNET_STATUS_REQUEST_COMPLETE: u32 = 100u32;
pub const INTERNET_STATUS_REQUEST_HEADERS_SET: u32 = 329u32;
pub const INTERNET_STATUS_REQUEST_SENT: u32 = 31u32;
pub const INTERNET_STATUS_RESOLVING_NAME: u32 = 10u32;
pub const INTERNET_STATUS_RESPONSE_HEADERS_SET: u32 = 330u32;
pub const INTERNET_STATUS_RESPONSE_RECEIVED: u32 = 41u32;
pub const INTERNET_STATUS_SENDING_COOKIE: u32 = 328u32;
pub const INTERNET_STATUS_SENDING_REQUEST: u32 = 30u32;
pub const INTERNET_STATUS_SERVER_CONNECTION_STATE: u32 = 410u32;
pub const INTERNET_STATUS_SERVER_CREDENTIALS: u32 = 401u32;
pub const INTERNET_STATUS_STATE_CHANGE: u32 = 200u32;
pub const INTERNET_STATUS_USER_INPUT_REQUIRED: u32 = 140u32;
pub const INTERNET_SUPPRESS_COOKIE_PERSIST: u32 = 3u32;
pub const INTERNET_SUPPRESS_COOKIE_PERSIST_RESET: u32 = 4u32;
pub const INTERNET_SUPPRESS_COOKIE_POLICY: u32 = 1u32;
pub const INTERNET_SUPPRESS_COOKIE_POLICY_RESET: u32 = 2u32;
pub const INTERNET_SUPPRESS_RESET_ALL: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct INTERNET_VERSION_INFO {
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
}
impl INTERNET_VERSION_INFO {}
impl ::core::default::Default for INTERNET_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for INTERNET_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INTERNET_VERSION_INFO").field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).finish()
    }
}
impl ::core::cmp::PartialEq for INTERNET_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion
    }
}
impl ::core::cmp::Eq for INTERNET_VERSION_INFO {}
unsafe impl ::windows::core::Abi for INTERNET_VERSION_INFO {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProofOfPossessionCookieInfoManager(pub ::windows::core::IUnknown);
impl IProofOfPossessionCookieInfoManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCookieInfoForUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, uri: Param0, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), uri.into_param().abi(), ::core::mem::transmute(cookieinfocount), ::core::mem::transmute(cookieinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IProofOfPossessionCookieInfoManager {
    type Vtable = IProofOfPossessionCookieInfoManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdaece56_4edf_43df_b113_88e4556fa1bb);
}
impl ::core::convert::From<IProofOfPossessionCookieInfoManager> for ::windows::core::IUnknown {
    fn from(value: IProofOfPossessionCookieInfoManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProofOfPossessionCookieInfoManager> for ::windows::core::IUnknown {
    fn from(value: &IProofOfPossessionCookieInfoManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProofOfPossessionCookieInfoManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProofOfPossessionCookieInfoManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProofOfPossessionCookieInfoManager2(pub ::windows::core::IUnknown);
impl IProofOfPossessionCookieInfoManager2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCookieInfoWithUriForAccount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, webaccount: Param0, uri: Param1, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), webaccount.into_param().abi(), uri.into_param().abi(), ::core::mem::transmute(cookieinfocount), ::core::mem::transmute(cookieinfo)).ok()
    }
}
unsafe impl ::windows::core::Interface for IProofOfPossessionCookieInfoManager2 {
    type Vtable = IProofOfPossessionCookieInfoManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15e41407_b42f_4ae7_9966_34a087b2d713);
}
impl ::core::convert::From<IProofOfPossessionCookieInfoManager2> for ::windows::core::IUnknown {
    fn from(value: IProofOfPossessionCookieInfoManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProofOfPossessionCookieInfoManager2> for ::windows::core::IUnknown {
    fn from(value: &IProofOfPossessionCookieInfoManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProofOfPossessionCookieInfoManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProofOfPossessionCookieInfoManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProofOfPossessionCookieInfoManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const IRF_ASYNC: u32 = 1u32;
pub const IRF_NO_WAIT: u32 = 8u32;
pub const IRF_SYNC: u32 = 4u32;
pub const IRF_USE_CONTEXT: u32 = 8u32;
pub const ISO_FORCE_DISCONNECTED: u32 = 1u32;
pub const ISO_FORCE_OFFLINE: u32 = 1u32;
pub const ISO_GLOBAL: u32 = 1u32;
pub const ISO_REGISTRY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImportCookieFileA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(szfilename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImportCookieFileA(szfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImportCookieFileA(szfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImportCookieFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(szfilename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImportCookieFileW(szfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ImportCookieFileW(szfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IncomingCookieState {
    pub cSession: i32,
    pub cPersistent: i32,
    pub cAccepted: i32,
    pub cLeashed: i32,
    pub cDowngraded: i32,
    pub cBlocked: i32,
    pub pszLocation: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl IncomingCookieState {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IncomingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IncomingCookieState {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IncomingCookieState")
            .field("cSession", &self.cSession)
            .field("cPersistent", &self.cPersistent)
            .field("cAccepted", &self.cAccepted)
            .field("cLeashed", &self.cLeashed)
            .field("cDowngraded", &self.cDowngraded)
            .field("cBlocked", &self.cBlocked)
            .field("pszLocation", &self.pszLocation)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IncomingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSession == other.cSession && self.cPersistent == other.cPersistent && self.cAccepted == other.cAccepted && self.cLeashed == other.cLeashed && self.cDowngraded == other.cDowngraded && self.cBlocked == other.cBlocked && self.pszLocation == other.pszLocation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IncomingCookieState {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IncomingCookieState {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IncrementUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IncrementUrlCacheHeaderData(nidx: u32, lpdwdata: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IncrementUrlCacheHeaderData(::core::mem::transmute(nidx), ::core::mem::transmute(lpdwdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternalInternetGetCookie<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszcookiedata: super::super::Foundation::PSTR, lpdwdatasize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternalInternetGetCookie(lpszurl: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwdatasize: *mut u32) -> u32;
        }
        ::core::mem::transmute(InternalInternetGetCookie(lpszurl.into_param().abi(), ::core::mem::transmute(lpszcookiedata), ::core::mem::transmute(lpdwdatasize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAlgIdToStringA(ai: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetAlgIdToStringA(ai: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetAlgIdToStringA(::core::mem::transmute(ai), ::core::mem::transmute(lpstr), ::core::mem::transmute(lpdwstrlength), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAlgIdToStringW(ai: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetAlgIdToStringW(ai: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetAlgIdToStringW(::core::mem::transmute(ai), ::core::mem::transmute(lpstr), ::core::mem::transmute(lpdwstrlength), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetAttemptConnect(dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetAttemptConnect(dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(InternetAttemptConnect(::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAutodial<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(dwflags: INTERNET_AUTODIAL, hwndparent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetAutodial(dwflags: INTERNET_AUTODIAL, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetAutodial(::core::mem::transmute(dwflags), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetAutodialHangup(dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetAutodialHangup(dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetAutodialHangup(::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCanonicalizeUrlA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCanonicalizeUrlA(lpszurl: super::super::Foundation::PSTR, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCanonicalizeUrlA(lpszurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCanonicalizeUrlW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCanonicalizeUrlW(lpszurl: super::super::Foundation::PWSTR, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCanonicalizeUrlW(lpszurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCheckConnectionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCheckConnectionA(lpszurl: super::super::Foundation::PSTR, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCheckConnectionA(lpszurl.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCheckConnectionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCheckConnectionW(lpszurl: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCheckConnectionW(lpszurl.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetClearAllPerSiteCookieDecisions() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetClearAllPerSiteCookieDecisions() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetClearAllPerSiteCookieDecisions())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCloseHandle(hinternet: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCloseHandle(hinternet: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCloseHandle(::core::mem::transmute(hinternet)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCombineUrlA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszbaseurl: Param0, lpszrelativeurl: Param1, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCombineUrlA(lpszbaseurl: super::super::Foundation::PSTR, lpszrelativeurl: super::super::Foundation::PSTR, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCombineUrlA(lpszbaseurl.into_param().abi(), lpszrelativeurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCombineUrlW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszbaseurl: Param0, lpszrelativeurl: Param1, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCombineUrlW(lpszbaseurl: super::super::Foundation::PWSTR, lpszrelativeurl: super::super::Foundation::PWSTR, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCombineUrlW(lpszbaseurl.into_param().abi(), lpszrelativeurl.into_param().abi(), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, szurlprev: Param1, szurlnew: Param2, bpost: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConfirmZoneCrossing(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PSTR, szurlnew: super::super::Foundation::PSTR, bpost: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(InternetConfirmZoneCrossing(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, szurlprev: Param1, szurlnew: Param2, bpost: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConfirmZoneCrossingA(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PSTR, szurlnew: super::super::Foundation::PSTR, bpost: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(InternetConfirmZoneCrossingA(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConfirmZoneCrossingW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, szurlprev: Param1, szurlnew: Param2, bpost: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConfirmZoneCrossingW(hwnd: super::super::Foundation::HWND, szurlprev: super::super::Foundation::PWSTR, szurlnew: super::super::Foundation::PWSTR, bpost: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(InternetConfirmZoneCrossingW(hwnd.into_param().abi(), szurlprev.into_param().abi(), szurlnew.into_param().abi(), bpost.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConnectA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hinternet: *const ::core::ffi::c_void, lpszservername: Param1, nserverport: u16, lpszusername: Param3, lpszpassword: Param4, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConnectA(hinternet: *const ::core::ffi::c_void, lpszservername: super::super::Foundation::PSTR, nserverport: u16, lpszusername: super::super::Foundation::PSTR, lpszpassword: super::super::Foundation::PSTR, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetConnectA(::core::mem::transmute(hinternet), lpszservername.into_param().abi(), ::core::mem::transmute(nserverport), lpszusername.into_param().abi(), lpszpassword.into_param().abi(), ::core::mem::transmute(dwservice), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConnectW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hinternet: *const ::core::ffi::c_void, lpszservername: Param1, nserverport: u16, lpszusername: Param3, lpszpassword: Param4, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConnectW(hinternet: *const ::core::ffi::c_void, lpszservername: super::super::Foundation::PWSTR, nserverport: u16, lpszusername: super::super::Foundation::PWSTR, lpszpassword: super::super::Foundation::PWSTR, dwservice: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetConnectW(::core::mem::transmute(hinternet), lpszservername.into_param().abi(), ::core::mem::transmute(nserverport), lpszusername.into_param().abi(), lpszpassword.into_param().abi(), ::core::mem::transmute(dwservice), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetConvertUrlFromWireToWideChar<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pcszurl: Param0, cchurl: u32, pcwszbaseurl: Param2, dwcodepagehost: u32, dwcodepagepath: u32, fencodepathextra: Param5, dwcodepageextra: u32, ppwszconvertedurl: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetConvertUrlFromWireToWideChar(pcszurl: super::super::Foundation::PSTR, cchurl: u32, pcwszbaseurl: super::super::Foundation::PWSTR, dwcodepagehost: u32, dwcodepagepath: u32, fencodepathextra: super::super::Foundation::BOOL, dwcodepageextra: u32, ppwszconvertedurl: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(InternetConvertUrlFromWireToWideChar(
            pcszurl.into_param().abi(),
            ::core::mem::transmute(cchurl),
            pcwszbaseurl.into_param().abi(),
            ::core::mem::transmute(dwcodepagehost),
            ::core::mem::transmute(dwcodepagepath),
            fencodepathextra.into_param().abi(),
            ::core::mem::transmute(dwcodepageextra),
            ::core::mem::transmute(ppwszconvertedurl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct InternetCookieHistory {
    pub fAccepted: super::super::Foundation::BOOL,
    pub fLeashed: super::super::Foundation::BOOL,
    pub fDowngraded: super::super::Foundation::BOOL,
    pub fRejected: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl InternetCookieHistory {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for InternetCookieHistory {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for InternetCookieHistory {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("InternetCookieHistory").field("fAccepted", &self.fAccepted).field("fLeashed", &self.fLeashed).field("fDowngraded", &self.fDowngraded).field("fRejected", &self.fRejected).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for InternetCookieHistory {
    fn eq(&self, other: &Self) -> bool {
        self.fAccepted == other.fAccepted && self.fLeashed == other.fLeashed && self.fDowngraded == other.fDowngraded && self.fRejected == other.fRejected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for InternetCookieHistory {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for InternetCookieHistory {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InternetCookieState(pub i32);
pub const COOKIE_STATE_UNKNOWN: InternetCookieState = InternetCookieState(0i32);
pub const COOKIE_STATE_ACCEPT: InternetCookieState = InternetCookieState(1i32);
pub const COOKIE_STATE_PROMPT: InternetCookieState = InternetCookieState(2i32);
pub const COOKIE_STATE_LEASH: InternetCookieState = InternetCookieState(3i32);
pub const COOKIE_STATE_DOWNGRADE: InternetCookieState = InternetCookieState(4i32);
pub const COOKIE_STATE_REJECT: InternetCookieState = InternetCookieState(5i32);
pub const COOKIE_STATE_MAX: InternetCookieState = InternetCookieState(5i32);
impl ::core::convert::From<i32> for InternetCookieState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InternetCookieState {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
#[inline]
pub unsafe fn InternetCrackUrlA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCrackUrlA(lpszurl: super::super::Foundation::PSTR, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCrackUrlA(lpszurl.into_param().abi(), ::core::mem::transmute(dwurllength), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpurlcomponents)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinHttp"))]
#[inline]
pub unsafe fn InternetCrackUrlW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCrackUrlW(lpszurl: super::super::Foundation::PWSTR, dwurllength: u32, dwflags: super::WinHttp::WIN_HTTP_CREATE_URL_FLAGS, lpurlcomponents: *mut URL_COMPONENTSW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCrackUrlW(lpszurl.into_param().abi(), ::core::mem::transmute(dwurllength), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpurlcomponents)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: super::super::Foundation::PSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCreateUrlA(lpurlcomponents: *const URL_COMPONENTSA, dwflags: u32, lpszurl: super::super::Foundation::PSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCreateUrlA(::core::mem::transmute(lpurlcomponents), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpszurl), ::core::mem::transmute(lpdwurllength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetCreateUrlW(lpurlcomponents: *const URL_COMPONENTSW, dwflags: u32, lpszurl: super::super::Foundation::PWSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetCreateUrlW(lpurlcomponents: *const URL_COMPONENTSW, dwflags: u32, lpszurl: super::super::Foundation::PWSTR, lpdwurllength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetCreateUrlW(::core::mem::transmute(lpurlcomponents), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpszurl), ::core::mem::transmute(lpdwurllength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDial<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwndparent: Param0, lpszconnectoid: Param1, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetDial(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PSTR, dwflags: u32, lpdwconnection: *mut u32, dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(InternetDial(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpdwconnection), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDialA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwndparent: Param0, lpszconnectoid: Param1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetDialA(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PSTR, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(InternetDialA(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpdwconnection), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetDialW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndparent: Param0, lpszconnectoid: Param1, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetDialW(hwndparent: super::super::Foundation::HWND, lpszconnectoid: super::super::Foundation::PWSTR, dwflags: u32, lpdwconnection: *mut usize, dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(InternetDialW(hwndparent.into_param().abi(), lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpdwconnection), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionA(pszsitename: super::super::Foundation::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetEnumPerSiteCookieDecisionA(pszsitename: super::super::Foundation::PSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetEnumPerSiteCookieDecisionA(::core::mem::transmute(pszsitename), ::core::mem::transmute(pcsitenamesize), ::core::mem::transmute(pdwdecision), ::core::mem::transmute(dwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetEnumPerSiteCookieDecisionW(pszsitename: super::super::Foundation::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetEnumPerSiteCookieDecisionW(pszsitename: super::super::Foundation::PWSTR, pcsitenamesize: *mut u32, pdwdecision: *mut u32, dwindex: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetEnumPerSiteCookieDecisionW(::core::mem::transmute(pszsitename), ::core::mem::transmute(pcsitenamesize), ::core::mem::transmute(pdwdecision), ::core::mem::transmute(dwindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetErrorDlg<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, hrequest: *mut ::core::ffi::c_void, dwerror: u32, dwflags: u32, lppvdata: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetErrorDlg(hwnd: super::super::Foundation::HWND, hrequest: *mut ::core::ffi::c_void, dwerror: u32, dwflags: u32, lppvdata: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(InternetErrorDlg(hwnd.into_param().abi(), ::core::mem::transmute(hrequest), ::core::mem::transmute(dwerror), ::core::mem::transmute(dwflags), ::core::mem::transmute(lppvdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFindNextFileA(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetFindNextFileA(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetFindNextFileA(::core::mem::transmute(hfind), ::core::mem::transmute(lpvfinddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFindNextFileW(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetFindNextFileW(hfind: *const ::core::ffi::c_void, lpvfinddata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetFindNextFileW(::core::mem::transmute(hfind), ::core::mem::transmute(lpvfinddata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFortezzaCommand<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(dwcommand: u32, hwnd: Param1, dwreserved: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetFortezzaCommand(dwcommand: u32, hwnd: super::super::Foundation::HWND, dwreserved: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetFortezzaCommand(::core::mem::transmute(dwcommand), hwnd.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFreeCookies(pcookies: *mut INTERNET_COOKIE2, dwcookiecount: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetFreeCookies(pcookies: *mut INTERNET_COOKIE2, dwcookiecount: u32);
        }
        ::core::mem::transmute(InternetFreeCookies(::core::mem::transmute(pcookies), ::core::mem::transmute(dwcookiecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetFreeProxyInfoList(pproxyinfolist: *mut WININET_PROXY_INFO_LIST) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetFreeProxyInfoList(pproxyinfolist: *mut WININET_PROXY_INFO_LIST);
        }
        ::core::mem::transmute(InternetFreeProxyInfoList(::core::mem::transmute(pproxyinfolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedState(lpdwflags: *mut INTERNET_CONNECTION, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetConnectedState(lpdwflags: *mut INTERNET_CONNECTION, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetConnectedState(::core::mem::transmute(lpdwflags), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateEx(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, dwnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetConnectedStateEx(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, dwnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetConnectedStateEx(::core::mem::transmute(lpdwflags), ::core::mem::transmute(lpszconnectionname), ::core::mem::transmute(dwnamelen), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateExA(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetConnectedStateExA(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetConnectedStateExA(::core::mem::transmute(lpdwflags), ::core::mem::transmute(lpszconnectionname), ::core::mem::transmute(cchnamelen), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetConnectedStateExW(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PWSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetConnectedStateExW(lpdwflags: *mut INTERNET_CONNECTION, lpszconnectionname: super::super::Foundation::PWSTR, cchnamelen: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetConnectedStateExW(::core::mem::transmute(lpdwflags), ::core::mem::transmute(lpszconnectionname), ::core::mem::transmute(cchnamelen), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetCookieA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetCookieA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), ::core::mem::transmute(lpszcookiedata), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcwszurl: Param0, pcwszcookiename: Param1, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetCookieEx2(pcwszurl: super::super::Foundation::PWSTR, pcwszcookiename: super::super::Foundation::PWSTR, dwflags: u32, ppcookies: *mut *mut INTERNET_COOKIE2, pdwcookiecount: *mut u32) -> u32;
        }
        ::core::mem::transmute(InternetGetCookieEx2(pcwszurl.into_param().abi(), pcwszcookiename.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppcookies), ::core::mem::transmute(pdwcookiecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetCookieExA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetCookieExA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), ::core::mem::transmute(lpdwsize), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetCookieExW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, lpdwsize: *mut u32, dwflags: INTERNET_COOKIE_FLAGS, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetCookieExW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), ::core::mem::transmute(lpdwsize), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetCookieW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetCookieW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetCookieW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), ::core::mem::transmute(lpszcookiedata), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetLastResponseInfoA(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetLastResponseInfoA(::core::mem::transmute(lpdwerror), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetLastResponseInfoW(lpdwerror: *mut u32, lpszbuffer: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetLastResponseInfoW(::core::mem::transmute(lpdwerror), ::core::mem::transmute(lpszbuffer), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pchhostname: Param0, presult: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetPerSiteCookieDecisionA(pchhostname: super::super::Foundation::PSTR, presult: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetPerSiteCookieDecisionA(pchhostname.into_param().abi(), ::core::mem::transmute(presult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetPerSiteCookieDecisionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pchhostname: Param0, presult: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetPerSiteCookieDecisionW(pchhostname: super::super::Foundation::PWSTR, presult: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetPerSiteCookieDecisionW(pchhostname.into_param().abi(), ::core::mem::transmute(presult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGetProxyForUrl<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hinternet: *const ::core::ffi::c_void, pcwszurl: Param1, pproxyinfolist: *mut WININET_PROXY_INFO_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetProxyForUrl(hinternet: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pproxyinfolist: *mut WININET_PROXY_INFO_LIST) -> u32;
        }
        ::core::mem::transmute(InternetGetProxyForUrl(::core::mem::transmute(hinternet), pcwszurl.into_param().abi(), ::core::mem::transmute(pproxyinfolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetSecurityInfoByURL(lpszurl: super::super::Foundation::PSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetSecurityInfoByURL(lpszurl.into_param().abi(), ::core::mem::transmute(ppcertchain), ::core::mem::transmute(pdwsecureflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetSecurityInfoByURLA(lpszurl: super::super::Foundation::PSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetSecurityInfoByURLA(lpszurl.into_param().abi(), ::core::mem::transmute(ppcertchain), ::core::mem::transmute(pdwsecureflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn InternetGetSecurityInfoByURLW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGetSecurityInfoByURLW(lpszurl: super::super::Foundation::PWSTR, ppcertchain: *mut *mut super::super::Security::Cryptography::CERT_CHAIN_CONTEXT, pdwsecureflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGetSecurityInfoByURLW(lpszurl.into_param().abi(), ::core::mem::transmute(ppcertchain), ::core::mem::transmute(pdwsecureflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnline<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGoOnline(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGoOnline(lpszurl.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnlineA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGoOnlineA(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGoOnlineA(lpszurl.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetGoOnlineW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetGoOnlineW(lpszurl: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetGoOnlineW(lpszurl.into_param().abi(), hwndparent.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetHangUp(dwconnection: usize, dwreserved: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetHangUp(dwconnection: usize, dwreserved: u32) -> u32;
        }
        ::core::mem::transmute(InternetHangUp(::core::mem::transmute(dwconnection), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetInitializeAutoProxyDll(dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetInitializeAutoProxyDll(dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetInitializeAutoProxyDll(::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetLockRequestFile(hinternet: *const ::core::ffi::c_void, lphlockrequestinfo: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetLockRequestFile(hinternet: *const ::core::ffi::c_void, lphlockrequestinfo: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetLockRequestFile(::core::mem::transmute(hinternet), ::core::mem::transmute(lphlockrequestinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetOpenA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszagent: Param0, dwaccesstype: u32, lpszproxy: Param2, lpszproxybypass: Param3, dwflags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetOpenA(lpszagent: super::super::Foundation::PSTR, dwaccesstype: u32, lpszproxy: super::super::Foundation::PSTR, lpszproxybypass: super::super::Foundation::PSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetOpenA(lpszagent.into_param().abi(), ::core::mem::transmute(dwaccesstype), lpszproxy.into_param().abi(), lpszproxybypass.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetOpenUrlA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hinternet: *const ::core::ffi::c_void, lpszurl: Param1, lpszheaders: Param2, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetOpenUrlA(hinternet: *const ::core::ffi::c_void, lpszurl: super::super::Foundation::PSTR, lpszheaders: super::super::Foundation::PSTR, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetOpenUrlA(::core::mem::transmute(hinternet), lpszurl.into_param().abi(), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetOpenUrlW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hinternet: *const ::core::ffi::c_void, lpszurl: Param1, lpszheaders: Param2, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetOpenUrlW(hinternet: *const ::core::ffi::c_void, lpszurl: super::super::Foundation::PWSTR, lpszheaders: super::super::Foundation::PWSTR, dwheaderslength: u32, dwflags: u32, dwcontext: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetOpenUrlW(::core::mem::transmute(hinternet), lpszurl.into_param().abi(), lpszheaders.into_param().abi(), ::core::mem::transmute(dwheaderslength), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetOpenW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszagent: Param0, dwaccesstype: u32, lpszproxy: Param2, lpszproxybypass: Param3, dwflags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetOpenW(lpszagent: super::super::Foundation::PWSTR, dwaccesstype: u32, lpszproxy: super::super::Foundation::PWSTR, lpszproxybypass: super::super::Foundation::PWSTR, dwflags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(InternetOpenW(lpszagent.into_param().abi(), ::core::mem::transmute(dwaccesstype), lpszproxy.into_param().abi(), lpszproxybypass.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryDataAvailable(hfile: *const ::core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetQueryDataAvailable(hfile: *const ::core::ffi::c_void, lpdwnumberofbytesavailable: *mut u32, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetQueryDataAvailable(::core::mem::transmute(hfile), ::core::mem::transmute(lpdwnumberofbytesavailable), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryFortezzaStatus(pdwstatus: *mut u32, dwreserved: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetQueryFortezzaStatus(pdwstatus: *mut u32, dwreserved: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetQueryFortezzaStatus(::core::mem::transmute(pdwstatus), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetQueryOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetQueryOptionA(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetQueryOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetQueryOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetQueryOptionW(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFile(hfile: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetReadFile(hfile: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, dwnumberofbytestoread: u32, lpdwnumberofbytesread: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetReadFile(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwnumberofbytestoread), ::core::mem::transmute(lpdwnumberofbytesread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFileExA(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetReadFileExA(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetReadFileExA(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetReadFileExW(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetReadFileExW(hfile: *const ::core::ffi::c_void, lpbuffersout: *mut INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetReadFileExW(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffersout), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSecurityProtocolToStringA(dwprotocol: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSecurityProtocolToStringA(dwprotocol: u32, lpstr: super::super::Foundation::PSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSecurityProtocolToStringA(::core::mem::transmute(dwprotocol), ::core::mem::transmute(lpstr), ::core::mem::transmute(lpdwstrlength), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSecurityProtocolToStringW(dwprotocol: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSecurityProtocolToStringW(dwprotocol: u32, lpstr: super::super::Foundation::PWSTR, lpdwstrlength: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSecurityProtocolToStringW(::core::mem::transmute(dwprotocol), ::core::mem::transmute(lpstr), ::core::mem::transmute(lpdwstrlength), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetCookieA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetCookieA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieEx2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pcwszurl: Param0, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: Param2, dwflags: u32, pdwcookiestate: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetCookieEx2(pcwszurl: super::super::Foundation::PWSTR, pcookie: *const INTERNET_COOKIE2, pcwszp3ppolicy: super::super::Foundation::PWSTR, dwflags: u32, pdwcookiestate: *mut u32) -> u32;
        }
        ::core::mem::transmute(InternetSetCookieEx2(pcwszurl.into_param().abi(), ::core::mem::transmute(pcookie), pcwszp3ppolicy.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pdwcookiestate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2, dwflags: u32, dwreserved: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetCookieExA(lpszurl: super::super::Foundation::PSTR, lpszcookiename: super::super::Foundation::PSTR, lpszcookiedata: super::super::Foundation::PSTR, dwflags: u32, dwreserved: usize) -> u32;
        }
        ::core::mem::transmute(InternetSetCookieExA(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2, dwflags: u32, dwreserved: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetCookieExW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: usize) -> u32;
        }
        ::core::mem::transmute(InternetSetCookieExW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetCookieW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurl: Param0, lpszcookiename: Param1, lpszcookiedata: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetCookieW(lpszurl: super::super::Foundation::PWSTR, lpszcookiename: super::super::Foundation::PWSTR, lpszcookiedata: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetCookieW(lpszurl.into_param().abi(), lpszcookiename.into_param().abi(), lpszcookiedata.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszconnectoid: Param0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetDialState(lpszconnectoid: super::super::Foundation::PSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetDialState(lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwstate), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialStateA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszconnectoid: Param0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetDialStateA(lpszconnectoid: super::super::Foundation::PSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetDialStateA(lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwstate), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetDialStateW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszconnectoid: Param0, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetDialStateW(lpszconnectoid: super::super::Foundation::PWSTR, dwstate: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetDialStateW(lpszconnectoid.into_param().abi(), ::core::mem::transmute(dwstate), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetSetFilePointer(hfile: *const ::core::ffi::c_void, ldistancetomove: i32, lpdistancetomovehigh: *mut i32, dwmovemethod: u32, dwcontext: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetFilePointer(hfile: *const ::core::ffi::c_void, ldistancetomove: i32, lpdistancetomovehigh: *mut i32, dwmovemethod: u32, dwcontext: usize) -> u32;
        }
        ::core::mem::transmute(InternetSetFilePointer(::core::mem::transmute(hfile), ::core::mem::transmute(ldistancetomove), ::core::mem::transmute(lpdistancetomovehigh), ::core::mem::transmute(dwmovemethod), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetOptionA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetOptionA(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionExA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetOptionExA(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetOptionExA(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionExW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetOptionExW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetOptionExW(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbufferlength), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetOptionW(hinternet: *const ::core::ffi::c_void, dwoption: u32, lpbuffer: *const ::core::ffi::c_void, dwbufferlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetOptionW(::core::mem::transmute(hinternet), ::core::mem::transmute(dwoption), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pchhostname: Param0, dwdecision: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetPerSiteCookieDecisionA(pchhostname: super::super::Foundation::PSTR, dwdecision: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetPerSiteCookieDecisionA(pchhostname.into_param().abi(), ::core::mem::transmute(dwdecision)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetSetPerSiteCookieDecisionW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pchhostname: Param0, dwdecision: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetPerSiteCookieDecisionW(pchhostname: super::super::Foundation::PWSTR, dwdecision: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetSetPerSiteCookieDecisionW(pchhostname.into_param().abi(), ::core::mem::transmute(dwdecision)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetSetStatusCallback(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::core::option::Option<LPINTERNET_STATUS_CALLBACK>) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetStatusCallback(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::core::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
        }
        ::core::mem::transmute(InternetSetStatusCallback(::core::mem::transmute(hinternet), ::core::mem::transmute(lpfninternetcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetSetStatusCallbackA(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::core::option::Option<LPINTERNET_STATUS_CALLBACK>) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetStatusCallbackA(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::core::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
        }
        ::core::mem::transmute(InternetSetStatusCallbackA(::core::mem::transmute(hinternet), ::core::mem::transmute(lpfninternetcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InternetSetStatusCallbackW(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::core::option::Option<LPINTERNET_STATUS_CALLBACK>) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetSetStatusCallbackW(hinternet: *const ::core::ffi::c_void, lpfninternetcallback: ::windows::core::RawPtr) -> ::core::option::Option<LPINTERNET_STATUS_CALLBACK>;
        }
        ::core::mem::transmute(InternetSetStatusCallbackW(::core::mem::transmute(hinternet), ::core::mem::transmute(lpfninternetcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURL<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetShowSecurityInfoByURL(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetShowSecurityInfoByURL(lpszurl.into_param().abi(), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetShowSecurityInfoByURLA(lpszurl: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetShowSecurityInfoByURLA(lpszurl.into_param().abi(), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetShowSecurityInfoByURLW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszurl: Param0, hwndparent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetShowSecurityInfoByURLW(lpszurl: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetShowSecurityInfoByURLW(lpszurl.into_param().abi(), hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeFromSystemTime(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeFromSystemTime(::core::mem::transmute(pst), ::core::mem::transmute(dwrfc), ::core::mem::transmute(lpsztime), ::core::mem::transmute(cbtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeA(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeFromSystemTimeA(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PSTR, cbtime: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeFromSystemTimeA(::core::mem::transmute(pst), ::core::mem::transmute(dwrfc), ::core::mem::transmute(lpsztime), ::core::mem::transmute(cbtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeFromSystemTimeW(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PWSTR, cbtime: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeFromSystemTimeW(pst: *const super::super::Foundation::SYSTEMTIME, dwrfc: u32, lpsztime: super::super::Foundation::PWSTR, cbtime: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeFromSystemTimeW(::core::mem::transmute(pst), ::core::mem::transmute(dwrfc), ::core::mem::transmute(lpsztime), ::core::mem::transmute(cbtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpsztime: Param0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeToSystemTime(lpsztime: super::super::Foundation::PSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeToSystemTime(lpsztime.into_param().abi(), ::core::mem::transmute(pst), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpsztime: Param0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeToSystemTimeA(lpsztime: super::super::Foundation::PSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeToSystemTimeA(lpsztime.into_param().abi(), ::core::mem::transmute(pst), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetTimeToSystemTimeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpsztime: Param0, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetTimeToSystemTimeW(lpsztime: super::super::Foundation::PWSTR, pst: *mut super::super::Foundation::SYSTEMTIME, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetTimeToSystemTimeW(lpsztime.into_param().abi(), ::core::mem::transmute(pst), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetUnlockRequestFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hlockrequestinfo: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetUnlockRequestFile(hlockrequestinfo: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetUnlockRequestFile(hlockrequestinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFile(hfile: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetWriteFile(hfile: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, dwnumberofbytestowrite: u32, lpdwnumberofbyteswritten: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetWriteFile(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwnumberofbytestowrite), ::core::mem::transmute(lpdwnumberofbyteswritten)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFileExA(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetWriteFileExA(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSA, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetWriteFileExA(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffersin), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InternetWriteFileExW(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InternetWriteFileExW(hfile: *const ::core::ffi::c_void, lpbuffersin: *const INTERNET_BUFFERSW, dwflags: u32, dwcontext: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InternetWriteFileExW(::core::mem::transmute(hfile), ::core::mem::transmute(lpbuffersin), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDomainLegalCookieDomainA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pchdomain: Param0, pchfulldomain: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDomainLegalCookieDomainA(pchdomain: super::super::Foundation::PSTR, pchfulldomain: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsDomainLegalCookieDomainA(pchdomain.into_param().abi(), pchfulldomain.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDomainLegalCookieDomainW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pchdomain: Param0, pchfulldomain: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDomainLegalCookieDomainW(pchdomain: super::super::Foundation::PWSTR, pchfulldomain: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsDomainLegalCookieDomainW(pchdomain.into_param().abi(), pchfulldomain.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsHostInProxyBypassList<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(tscheme: INTERNET_SCHEME, lpszhost: Param1, cchhost: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsHostInProxyBypassList(tscheme: INTERNET_SCHEME, lpszhost: super::super::Foundation::PSTR, cchhost: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsHostInProxyBypassList(::core::mem::transmute(tscheme), lpszhost.into_param().abi(), ::core::mem::transmute(cchhost)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProfilesEnabled() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProfilesEnabled() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProfilesEnabled())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsUrlCacheEntryExpiredA(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsUrlCacheEntryExpiredA(lpszurlname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pftlastmodified)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUrlCacheEntryExpiredW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsUrlCacheEntryExpiredW(lpszurlname: super::super::Foundation::PWSTR, dwflags: u32, pftlastmodified: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsUrlCacheEntryExpiredW(lpszurlname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pftlastmodified)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type LPINTERNET_STATUS_CALLBACK = unsafe extern "system" fn(hinternet: *const ::core::ffi::c_void, dwcontext: usize, dwinternetstatus: u32, lpvstatusinformation: *const ::core::ffi::c_void, dwstatusinformationlength: u32);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadUrlCacheContent() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadUrlCacheContent() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LoadUrlCacheContent())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_CACHE_ENTRY_INFO_SIZE: u32 = 4096u32;
pub const MAX_GOPHER_ATTRIBUTE_NAME: u32 = 128u32;
pub const MAX_GOPHER_CATEGORY_NAME: u32 = 128u32;
pub const MAX_GOPHER_DISPLAY_TEXT: u32 = 128u32;
pub const MAX_GOPHER_HOST_NAME: u32 = 256u32;
pub const MAX_GOPHER_SELECTOR_TEXT: u32 = 256u32;
pub const MIN_GOPHER_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const MUST_REVALIDATE_CACHE_ENTRY: u32 = 256u32;
pub const MaxPrivacySettings: u32 = 16384u32;
pub const NORMAL_CACHE_ENTRY: u32 = 1u32;
pub const OTHER_USER_CACHE_ENTRY: u32 = 8388608u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OutgoingCookieState {
    pub cSent: i32,
    pub cSuppressed: i32,
    pub pszLocation: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl OutgoingCookieState {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OutgoingCookieState {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OutgoingCookieState {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OutgoingCookieState").field("cSent", &self.cSent).field("cSuppressed", &self.cSuppressed).field("pszLocation", &self.pszLocation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OutgoingCookieState {
    fn eq(&self, other: &Self) -> bool {
        self.cSent == other.cSent && self.cSuppressed == other.cSuppressed && self.pszLocation == other.pszLocation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OutgoingCookieState {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OutgoingCookieState {
    type Abi = Self;
}
pub const PENDING_DELETE_CACHE_ENTRY: u32 = 4194304u32;
pub type PFN_AUTH_NOTIFY = unsafe extern "system" fn(param0: usize, param1: u32, param2: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DIAL_HANDLER = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR, param2: u32, param3: *mut u32) -> u32;
pub const POST_CHECK_CACHE_ENTRY: u32 = 536870912u32;
pub const POST_RESPONSE_CACHE_ENTRY: u32 = 67108864u32;
pub const PRIVACY_IMPACTED_CACHE_ENTRY: u32 = 33554432u32;
pub const PRIVACY_MODE_CACHE_ENTRY: u32 = 131072u32;
pub const PRIVACY_TEMPLATE_ADVANCED: u32 = 101u32;
pub const PRIVACY_TEMPLATE_CUSTOM: u32 = 100u32;
pub const PRIVACY_TEMPLATE_HIGH: u32 = 1u32;
pub const PRIVACY_TEMPLATE_LOW: u32 = 5u32;
pub const PRIVACY_TEMPLATE_MAX: u32 = 5u32;
pub const PRIVACY_TEMPLATE_MEDIUM: u32 = 3u32;
pub const PRIVACY_TEMPLATE_MEDIUM_HIGH: u32 = 2u32;
pub const PRIVACY_TEMPLATE_MEDIUM_LOW: u32 = 4u32;
pub const PRIVACY_TEMPLATE_NO_COOKIES: u32 = 0u32;
pub const PRIVACY_TYPE_FIRST_PARTY: u32 = 0u32;
pub const PRIVACY_TYPE_THIRD_PARTY: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROXY_AUTO_DETECT_TYPE(pub u32);
pub const PROXY_AUTO_DETECT_TYPE_DHCP: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(1u32);
pub const PROXY_AUTO_DETECT_TYPE_DNS_A: PROXY_AUTO_DETECT_TYPE = PROXY_AUTO_DETECT_TYPE(2u32);
impl ::core::convert::From<u32> for PROXY_AUTO_DETECT_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROXY_AUTO_DETECT_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PROXY_AUTO_DETECT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PROXY_AUTO_DETECT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROXY_TYPE_AUTO_DETECT: u32 = 8u32;
pub const PROXY_TYPE_AUTO_PROXY_URL: u32 = 4u32;
pub const PROXY_TYPE_DIRECT: u32 = 1u32;
pub const PROXY_TYPE_PROXY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ParseX509EncodedCertificateForListBoxEntry(lpcert: *const u8, cbcert: u32, lpszlistboxentry: super::super::Foundation::PSTR, lpdwlistboxentry: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ParseX509EncodedCertificateForListBoxEntry(lpcert: *const u8, cbcert: u32, lpszlistboxentry: super::super::Foundation::PSTR, lpdwlistboxentry: *mut u32) -> u32;
        }
        ::core::mem::transmute(ParseX509EncodedCertificateForListBoxEntry(::core::mem::transmute(lpcert), ::core::mem::transmute(cbcert), ::core::mem::transmute(lpszlistboxentry), ::core::mem::transmute(lpdwlistboxentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PerformOperationOverUrlCacheA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszurlsearchpattern: Param0, dwflags: u32, dwfilter: u32, groupid: i64, preserved1: *mut ::core::ffi::c_void, pdwreserved2: *mut u32, preserved3: *mut ::core::ffi::c_void, op: ::core::option::Option<CACHE_OPERATOR>, poperatordata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PerformOperationOverUrlCacheA(pszurlsearchpattern: super::super::Foundation::PSTR, dwflags: u32, dwfilter: u32, groupid: i64, preserved1: *mut ::core::ffi::c_void, pdwreserved2: *mut u32, preserved3: *mut ::core::ffi::c_void, op: ::windows::core::RawPtr, poperatordata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PerformOperationOverUrlCacheA(
            pszurlsearchpattern.into_param().abi(),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(dwfilter),
            ::core::mem::transmute(groupid),
            ::core::mem::transmute(preserved1),
            ::core::mem::transmute(pdwreserved2),
            ::core::mem::transmute(preserved3),
            ::core::mem::transmute(op),
            ::core::mem::transmute(poperatordata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: *mut u32, pszbuffer: super::super::Foundation::PWSTR, pdwbufferlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivacyGetZonePreferenceW(dwzone: u32, dwtype: u32, pdwtemplate: *mut u32, pszbuffer: super::super::Foundation::PWSTR, pdwbufferlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(PrivacyGetZonePreferenceW(::core::mem::transmute(dwzone), ::core::mem::transmute(dwtype), ::core::mem::transmute(pdwtemplate), ::core::mem::transmute(pszbuffer), ::core::mem::transmute(pdwbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrivacySetZonePreferenceW<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrivacySetZonePreferenceW(dwzone: u32, dwtype: u32, dwtemplate: u32, pszpreference: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(PrivacySetZonePreferenceW(::core::mem::transmute(dwzone), ::core::mem::transmute(dwtype), ::core::mem::transmute(dwtemplate), pszpreference.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ProofOfPossessionCookieInfo {
    pub name: super::super::Foundation::PWSTR,
    pub data: super::super::Foundation::PWSTR,
    pub flags: u32,
    pub p3pHeader: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ProofOfPossessionCookieInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ProofOfPossessionCookieInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ProofOfPossessionCookieInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ProofOfPossessionCookieInfo").field("name", &self.name).field("data", &self.data).field("flags", &self.flags).field("p3pHeader", &self.p3pHeader).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ProofOfPossessionCookieInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data && self.flags == other.flags && self.p3pHeader == other.p3pHeader
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ProofOfPossessionCookieInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ProofOfPossessionCookieInfo {
    type Abi = Self;
}
pub const ProofOfPossessionCookieInfoManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9927f85_a304_4390_8b23_a75f1c668600);
pub const REDIRECT_CACHE_ENTRY: u32 = 2048u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REQUEST_TIMES(pub i32);
pub const NameResolutionStart: REQUEST_TIMES = REQUEST_TIMES(0i32);
pub const NameResolutionEnd: REQUEST_TIMES = REQUEST_TIMES(1i32);
pub const ConnectionEstablishmentStart: REQUEST_TIMES = REQUEST_TIMES(2i32);
pub const ConnectionEstablishmentEnd: REQUEST_TIMES = REQUEST_TIMES(3i32);
pub const TLSHandshakeStart: REQUEST_TIMES = REQUEST_TIMES(4i32);
pub const TLSHandshakeEnd: REQUEST_TIMES = REQUEST_TIMES(5i32);
pub const HttpRequestTimeMax: REQUEST_TIMES = REQUEST_TIMES(32i32);
impl ::core::convert::From<i32> for REQUEST_TIMES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REQUEST_TIMES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadGuidsForConnectedNetworks(pcnetworks: *mut u32, pppwsznetworkguids: *mut *mut super::super::Foundation::PWSTR, pppbstrnetworknames: *mut *mut super::super::Foundation::BSTR, pppwszgwmacs: *mut *mut super::super::Foundation::PWSTR, pcgatewaymacs: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadGuidsForConnectedNetworks(pcnetworks: *mut u32, pppwsznetworkguids: *mut *mut super::super::Foundation::PWSTR, pppbstrnetworknames: *mut *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppwszgwmacs: *mut *mut super::super::Foundation::PWSTR, pcgatewaymacs: *mut u32, pdwflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadGuidsForConnectedNetworks(::core::mem::transmute(pcnetworks), ::core::mem::transmute(pppwsznetworkguids), ::core::mem::transmute(pppbstrnetworknames), ::core::mem::transmute(pppwszgwmacs), ::core::mem::transmute(pcgatewaymacs), ::core::mem::transmute(pdwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hurlcachestream: Param0, dwlocation: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32, reserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, dwlocation: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32, reserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadUrlCacheEntryStream(hurlcachestream.into_param().abi(), ::core::mem::transmute(dwlocation), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwlen), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReadUrlCacheEntryStreamEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hurlcachestream: Param0, qwlocation: u64, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReadUrlCacheEntryStreamEx(hurlcachestream: super::super::Foundation::HANDLE, qwlocation: u64, lpbuffer: *mut ::core::ffi::c_void, lpdwlen: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReadUrlCacheEntryStreamEx(hurlcachestream.into_param().abi(), ::core::mem::transmute(qwlocation), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpdwlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterUrlCacheNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, umsg: u32, gid: i64, dwopsfilter: u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterUrlCacheNotification(hwnd: super::super::Foundation::HWND, umsg: u32, gid: i64, dwopsfilter: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterUrlCacheNotification(hwnd.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(gid), ::core::mem::transmute(dwopsfilter), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResumeSuspendedDownload(hrequest: *const ::core::ffi::c_void, dwresultcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResumeSuspendedDownload(hrequest: *const ::core::ffi::c_void, dwresultcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ResumeSuspendedDownload(::core::mem::transmute(hrequest), ::core::mem::transmute(dwresultcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RetrieveUrlCacheEntryFileA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RetrieveUrlCacheEntryFileA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RetrieveUrlCacheEntryFileW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RetrieveUrlCacheEntryFileW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, frandomread: Param3, dwreserved: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RetrieveUrlCacheEntryStreamA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOA, lpcbcacheentryinfo: *mut u32, frandomread: super::super::Foundation::BOOL, dwreserved: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(RetrieveUrlCacheEntryStreamA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), frandomread.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RetrieveUrlCacheEntryStreamW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpszurlname: Param0, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, frandomread: Param3, dwreserved: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RetrieveUrlCacheEntryStreamW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *mut INTERNET_CACHE_ENTRY_INFOW, lpcbcacheentryinfo: *mut u32, frandomread: super::super::Foundation::BOOL, dwreserved: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(RetrieveUrlCacheEntryStreamW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(lpcbcacheentryinfo), frandomread.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunOnceUrlCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, hinst: Param1, lpszcmd: Param2, ncmdshow: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RunOnceUrlCache(hwnd: super::super::Foundation::HWND, hinst: super::super::Foundation::HINSTANCE, lpszcmd: super::super::Foundation::PSTR, ncmdshow: i32) -> u32;
        }
        ::core::mem::transmute(RunOnceUrlCache(hwnd.into_param().abi(), hinst.into_param().abi(), lpszcmd.into_param().abi(), ::core::mem::transmute(ncmdshow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SECURITY_FLAG_128BIT: u32 = 536870912u32;
pub const SECURITY_FLAG_40BIT: u32 = 268435456u32;
pub const SECURITY_FLAG_56BIT: u32 = 1073741824u32;
pub const SECURITY_FLAG_FORTEZZA: u32 = 134217728u32;
pub const SECURITY_FLAG_IETFSSL4: u32 = 32u32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTP: u32 = 32768u32;
pub const SECURITY_FLAG_IGNORE_REDIRECT_TO_HTTPS: u32 = 16384u32;
pub const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 128u32;
pub const SECURITY_FLAG_IGNORE_WEAK_SIGNATURE: u32 = 65536u32;
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: u32 = 512u32;
pub const SECURITY_FLAG_NORMALBITNESS: u32 = 268435456u32;
pub const SECURITY_FLAG_OPT_IN_WEAK_SIGNATURE: u32 = 131072u32;
pub const SECURITY_FLAG_PCT: u32 = 8u32;
pub const SECURITY_FLAG_PCT4: u32 = 16u32;
pub const SECURITY_FLAG_SSL: u32 = 2u32;
pub const SECURITY_FLAG_SSL3: u32 = 4u32;
pub const SECURITY_FLAG_UNKNOWNBIT: u32 = 2147483648u32;
pub const SHORTPATH_CACHE_ENTRY: u32 = 512u32;
pub const SPARSE_CACHE_ENTRY: u32 = 65536u32;
pub const STATIC_CACHE_ENTRY: u32 = 128u32;
pub const STICKY_CACHE_ENTRY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheConfigInfoA(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheConfigInfoA(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheConfigInfoA(::core::mem::transmute(lpcacheconfiginfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheConfigInfoW(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheConfigInfoW(lpcacheconfiginfo: *const INTERNET_CACHE_CONFIG_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheConfigInfoW(::core::mem::transmute(lpcacheconfiginfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheEntryGroup(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheEntryGroup(lpszurlname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(groupid), ::core::mem::transmute(pbgroupattributes), ::core::mem::transmute(cbgroupattributes), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroupA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheEntryGroupA(lpszurlname: super::super::Foundation::PSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheEntryGroupA(lpszurlname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(groupid), ::core::mem::transmute(pbgroupattributes), ::core::mem::transmute(cbgroupattributes), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryGroupW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheEntryGroupW(lpszurlname: super::super::Foundation::PWSTR, dwflags: u32, groupid: i64, pbgroupattributes: *mut u8, cbgroupattributes: u32, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheEntryGroupW(lpszurlname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(groupid), ::core::mem::transmute(pbgroupattributes), ::core::mem::transmute(cbgroupattributes), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheEntryInfoA(lpszurlname: super::super::Foundation::PSTR, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOA, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheEntryInfoA(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheEntryInfoW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheEntryInfoW(lpszurlname: super::super::Foundation::PWSTR, lpcacheentryinfo: *const INTERNET_CACHE_ENTRY_INFOW, dwfieldcontrol: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheEntryInfoW(lpszurlname.into_param().abi(), ::core::mem::transmute(lpcacheentryinfo), ::core::mem::transmute(dwfieldcontrol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheGroupAttributeA(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOA, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheGroupAttributeA(::core::mem::transmute(gid), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwattributes), ::core::mem::transmute(lpgroupinfo), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheGroupAttributeW(gid: i64, dwflags: u32, dwattributes: u32, lpgroupinfo: *const INTERNET_CACHE_GROUP_INFOW, lpreserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheGroupAttributeW(::core::mem::transmute(gid), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwattributes), ::core::mem::transmute(lpgroupinfo), ::core::mem::transmute(lpreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUrlCacheHeaderData(nidx: u32, dwdata: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUrlCacheHeaderData(nidx: u32, dwdata: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUrlCacheHeaderData(::core::mem::transmute(nidx), ::core::mem::transmute(dwdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowClientAuthCerts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowClientAuthCerts(hwndparent: super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(ShowClientAuthCerts(hwndparent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn ShowSecurityInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, psecurityinfo: *const INTERNET_SECURITY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowSecurityInfo(hwndparent: super::super::Foundation::HWND, psecurityinfo: *const INTERNET_SECURITY_INFO) -> u32;
        }
        ::core::mem::transmute(ShowSecurityInfo(hwndparent.into_param().abi(), ::core::mem::transmute(psecurityinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowX509EncodedCertificate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwndparent: Param0, lpcert: *const u8, cbcert: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowX509EncodedCertificate(hwndparent: super::super::Foundation::HWND, lpcert: *const u8, cbcert: u32) -> u32;
        }
        ::core::mem::transmute(ShowX509EncodedCertificate(hwndparent.into_param().abi(), ::core::mem::transmute(lpcert), ::core::mem::transmute(cbcert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TRACK_OFFLINE_CACHE_ENTRY: u32 = 16u32;
pub const TRACK_ONLINE_CACHE_ENTRY: u32 = 32u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct URLCACHE_ENTRY_INFO {
    pub pwszSourceUrlName: super::super::Foundation::PWSTR,
    pub pwszLocalFileName: super::super::Foundation::PWSTR,
    pub dwCacheEntryType: u32,
    pub dwUseCount: u32,
    pub dwHitRate: u32,
    pub dwSizeLow: u32,
    pub dwSizeHigh: u32,
    pub ftLastModifiedTime: super::super::Foundation::FILETIME,
    pub ftExpireTime: super::super::Foundation::FILETIME,
    pub ftLastAccessTime: super::super::Foundation::FILETIME,
    pub ftLastSyncTime: super::super::Foundation::FILETIME,
    pub pbHeaderInfo: *mut u8,
    pub cbHeaderInfoSize: u32,
    pub pbExtraData: *mut u8,
    pub cbExtraDataSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl URLCACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URLCACHE_ENTRY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URLCACHE_ENTRY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("URLCACHE_ENTRY_INFO")
            .field("pwszSourceUrlName", &self.pwszSourceUrlName)
            .field("pwszLocalFileName", &self.pwszLocalFileName)
            .field("dwCacheEntryType", &self.dwCacheEntryType)
            .field("dwUseCount", &self.dwUseCount)
            .field("dwHitRate", &self.dwHitRate)
            .field("dwSizeLow", &self.dwSizeLow)
            .field("dwSizeHigh", &self.dwSizeHigh)
            .field("ftLastModifiedTime", &self.ftLastModifiedTime)
            .field("ftExpireTime", &self.ftExpireTime)
            .field("ftLastAccessTime", &self.ftLastAccessTime)
            .field("ftLastSyncTime", &self.ftLastSyncTime)
            .field("pbHeaderInfo", &self.pbHeaderInfo)
            .field("cbHeaderInfoSize", &self.cbHeaderInfoSize)
            .field("pbExtraData", &self.pbExtraData)
            .field("cbExtraDataSize", &self.cbExtraDataSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URLCACHE_ENTRY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszSourceUrlName == other.pwszSourceUrlName
            && self.pwszLocalFileName == other.pwszLocalFileName
            && self.dwCacheEntryType == other.dwCacheEntryType
            && self.dwUseCount == other.dwUseCount
            && self.dwHitRate == other.dwHitRate
            && self.dwSizeLow == other.dwSizeLow
            && self.dwSizeHigh == other.dwSizeHigh
            && self.ftLastModifiedTime == other.ftLastModifiedTime
            && self.ftExpireTime == other.ftExpireTime
            && self.ftLastAccessTime == other.ftLastAccessTime
            && self.ftLastSyncTime == other.ftLastSyncTime
            && self.pbHeaderInfo == other.pbHeaderInfo
            && self.cbHeaderInfoSize == other.cbHeaderInfoSize
            && self.pbExtraData == other.pbExtraData
            && self.cbExtraDataSize == other.cbExtraDataSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URLCACHE_ENTRY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for URLCACHE_ENTRY_INFO {
    type Abi = Self;
}
pub const URLHISTORY_CACHE_ENTRY: u32 = 2097152u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct URL_CACHE_LIMIT_TYPE(pub i32);
pub const UrlCacheLimitTypeIE: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(0i32);
pub const UrlCacheLimitTypeIETotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(1i32);
pub const UrlCacheLimitTypeAppContainer: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(2i32);
pub const UrlCacheLimitTypeAppContainerTotal: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(3i32);
pub const UrlCacheLimitTypeNum: URL_CACHE_LIMIT_TYPE = URL_CACHE_LIMIT_TYPE(4i32);
impl ::core::convert::From<i32> for URL_CACHE_LIMIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for URL_CACHE_LIMIT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct URL_COMPONENTSA {
    pub dwStructSize: u32,
    pub lpszScheme: super::super::Foundation::PSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: super::super::Foundation::PSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: super::super::Foundation::PSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: super::super::Foundation::PSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: super::super::Foundation::PSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: super::super::Foundation::PSTR,
    pub dwExtraInfoLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl URL_COMPONENTSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URL_COMPONENTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URL_COMPONENTSA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("URL_COMPONENTSA")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URL_COMPONENTSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.lpszScheme == other.lpszScheme
            && self.dwSchemeLength == other.dwSchemeLength
            && self.nScheme == other.nScheme
            && self.lpszHostName == other.lpszHostName
            && self.dwHostNameLength == other.dwHostNameLength
            && self.nPort == other.nPort
            && self.lpszUserName == other.lpszUserName
            && self.dwUserNameLength == other.dwUserNameLength
            && self.lpszPassword == other.lpszPassword
            && self.dwPasswordLength == other.dwPasswordLength
            && self.lpszUrlPath == other.lpszUrlPath
            && self.dwUrlPathLength == other.dwUrlPathLength
            && self.lpszExtraInfo == other.lpszExtraInfo
            && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URL_COMPONENTSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for URL_COMPONENTSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct URL_COMPONENTSW {
    pub dwStructSize: u32,
    pub lpszScheme: super::super::Foundation::PWSTR,
    pub dwSchemeLength: u32,
    pub nScheme: INTERNET_SCHEME,
    pub lpszHostName: super::super::Foundation::PWSTR,
    pub dwHostNameLength: u32,
    pub nPort: u16,
    pub lpszUserName: super::super::Foundation::PWSTR,
    pub dwUserNameLength: u32,
    pub lpszPassword: super::super::Foundation::PWSTR,
    pub dwPasswordLength: u32,
    pub lpszUrlPath: super::super::Foundation::PWSTR,
    pub dwUrlPathLength: u32,
    pub lpszExtraInfo: super::super::Foundation::PWSTR,
    pub dwExtraInfoLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl URL_COMPONENTSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for URL_COMPONENTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for URL_COMPONENTSW {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("URL_COMPONENTSW")
            .field("dwStructSize", &self.dwStructSize)
            .field("lpszScheme", &self.lpszScheme)
            .field("dwSchemeLength", &self.dwSchemeLength)
            .field("nScheme", &self.nScheme)
            .field("lpszHostName", &self.lpszHostName)
            .field("dwHostNameLength", &self.dwHostNameLength)
            .field("nPort", &self.nPort)
            .field("lpszUserName", &self.lpszUserName)
            .field("dwUserNameLength", &self.dwUserNameLength)
            .field("lpszPassword", &self.lpszPassword)
            .field("dwPasswordLength", &self.dwPasswordLength)
            .field("lpszUrlPath", &self.lpszUrlPath)
            .field("dwUrlPathLength", &self.dwUrlPathLength)
            .field("lpszExtraInfo", &self.lpszExtraInfo)
            .field("dwExtraInfoLength", &self.dwExtraInfoLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for URL_COMPONENTSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize
            && self.lpszScheme == other.lpszScheme
            && self.dwSchemeLength == other.dwSchemeLength
            && self.nScheme == other.nScheme
            && self.lpszHostName == other.lpszHostName
            && self.dwHostNameLength == other.dwHostNameLength
            && self.nPort == other.nPort
            && self.lpszUserName == other.lpszUserName
            && self.dwUserNameLength == other.dwUserNameLength
            && self.lpszPassword == other.lpszPassword
            && self.dwPasswordLength == other.dwPasswordLength
            && self.lpszUrlPath == other.lpszUrlPath
            && self.dwUrlPathLength == other.dwUrlPathLength
            && self.lpszExtraInfo == other.lpszExtraInfo
            && self.dwExtraInfoLength == other.dwExtraInfoLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for URL_COMPONENTSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for URL_COMPONENTSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockUrlCacheEntryFile(lpszurlname: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnlockUrlCacheEntryFile(lpszurlname.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpszurlname: Param0, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockUrlCacheEntryFileA(lpszurlname: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnlockUrlCacheEntryFileA(lpszurlname.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryFileW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszurlname: Param0, dwreserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockUrlCacheEntryFileW(lpszurlname: super::super::Foundation::PWSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnlockUrlCacheEntryFileW(lpszurlname.into_param().abi(), ::core::mem::transmute(dwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockUrlCacheEntryStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hurlcachestream: Param0, reserved: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockUrlCacheEntryStream(hurlcachestream: super::super::Foundation::HANDLE, reserved: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnlockUrlCacheEntryStream(hurlcachestream.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateUrlCacheContentPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(sznewpath: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateUrlCacheContentPath(sznewpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateUrlCacheContentPath(sznewpath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheCheckEntriesExist(rgpwszurls: *const super::super::Foundation::PWSTR, centries: u32, rgfexist: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheCheckEntriesExist(rgpwszurls: *const super::super::Foundation::PWSTR, centries: u32, rgfexist: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(UrlCacheCheckEntriesExist(::core::mem::transmute(rgpwszurls), ::core::mem::transmute(centries), ::core::mem::transmute(rgfexist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheCloseEntryHandle(hentryfile: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheCloseEntryHandle(hentryfile: *const ::core::ffi::c_void);
        }
        ::core::mem::transmute(UrlCacheCloseEntryHandle(::core::mem::transmute(hentryfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheContainerSetEntryMaximumAge<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszprefix: Param0, dwentrymaxage: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheContainerSetEntryMaximumAge(pwszprefix: super::super::Foundation::PWSTR, dwentrymaxage: u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheContainerSetEntryMaximumAge(pwszprefix.into_param().abi(), ::core::mem::transmute(dwentrymaxage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheCreateContainer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszname: Param0, pwszprefix: Param1, pwszdirectory: Param2, ulllimit: u64, dwoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheCreateContainer(pwszname: super::super::Foundation::PWSTR, pwszprefix: super::super::Foundation::PWSTR, pwszdirectory: super::super::Foundation::PWSTR, ulllimit: u64, dwoptions: u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheCreateContainer(pwszname.into_param().abi(), pwszprefix.into_param().abi(), pwszdirectory.into_param().abi(), ::core::mem::transmute(ulllimit), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFindFirstEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszprefix: Param0, dwflags: u32, dwfilter: u32, groupid: i64, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phfind: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheFindFirstEntry(pwszprefix: super::super::Foundation::PWSTR, dwflags: u32, dwfilter: u32, groupid: i64, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phfind: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(UrlCacheFindFirstEntry(pwszprefix.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwfilter), ::core::mem::transmute(groupid), ::core::mem::transmute(pcacheentryinfo), ::core::mem::transmute(phfind)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFindNextEntry<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfind: Param0, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheFindNextEntry(hfind: super::super::Foundation::HANDLE, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32;
        }
        ::core::mem::transmute(UrlCacheFindNextEntry(hfind.into_param().abi(), ::core::mem::transmute(pcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheFreeEntryInfo(pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheFreeEntryInfo(pcacheentryinfo: *mut URLCACHE_ENTRY_INFO);
        }
        ::core::mem::transmute(UrlCacheFreeEntryInfo(::core::mem::transmute(pcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheFreeGlobalSpace(ulltargetsize: u64, dwfilter: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheFreeGlobalSpace(ulltargetsize: u64, dwfilter: u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheFreeGlobalSpace(::core::mem::transmute(ulltargetsize), ::core::mem::transmute(dwfilter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheGetContentPaths(pppwszdirectories: *mut *mut super::super::Foundation::PWSTR, pcdirectories: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheGetContentPaths(pppwszdirectories: *mut *mut super::super::Foundation::PWSTR, pcdirectories: *mut u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheGetContentPaths(::core::mem::transmute(pppwszdirectories), ::core::mem::transmute(pcdirectories)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheGetEntryInfo<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(happcache: *const ::core::ffi::c_void, pcwszurl: Param1, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheGetEntryInfo(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO) -> u32;
        }
        ::core::mem::transmute(UrlCacheGetEntryInfo(::core::mem::transmute(happcache), pcwszurl.into_param().abi(), ::core::mem::transmute(pcacheentryinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheGetGlobalCacheSize(dwfilter: u32, pullsize: *mut u64, pulllimit: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheGetGlobalCacheSize(dwfilter: u32, pullsize: *mut u64, pulllimit: *mut u64) -> u32;
        }
        ::core::mem::transmute(UrlCacheGetGlobalCacheSize(::core::mem::transmute(dwfilter), ::core::mem::transmute(pullsize), ::core::mem::transmute(pulllimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheGetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, pulllimit: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheGetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, pulllimit: *mut u64) -> u32;
        }
        ::core::mem::transmute(UrlCacheGetGlobalLimit(::core::mem::transmute(limittype), ::core::mem::transmute(pulllimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheReadEntryStream(hurlcachestream: *const ::core::ffi::c_void, ulllocation: u64, pbuffer: *mut ::core::ffi::c_void, dwbufferlen: u32, pdwbufferlen: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheReadEntryStream(hurlcachestream: *const ::core::ffi::c_void, ulllocation: u64, pbuffer: *mut ::core::ffi::c_void, dwbufferlen: u32, pdwbufferlen: *mut u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheReadEntryStream(::core::mem::transmute(hurlcachestream), ::core::mem::transmute(ulllocation), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwbufferlen), ::core::mem::transmute(pdwbufferlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheReloadSettings() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheReloadSettings() -> u32;
        }
        ::core::mem::transmute(UrlCacheReloadSettings())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheRetrieveEntryFile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(happcache: *const ::core::ffi::c_void, pcwszurl: Param1, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentryfile: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheRetrieveEntryFile(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentryfile: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(UrlCacheRetrieveEntryFile(::core::mem::transmute(happcache), pcwszurl.into_param().abi(), ::core::mem::transmute(pcacheentryinfo), ::core::mem::transmute(phentryfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheRetrieveEntryStream<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(happcache: *const ::core::ffi::c_void, pcwszurl: Param1, frandomread: Param2, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentrystream: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheRetrieveEntryStream(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, frandomread: super::super::Foundation::BOOL, pcacheentryinfo: *mut URLCACHE_ENTRY_INFO, phentrystream: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(UrlCacheRetrieveEntryStream(::core::mem::transmute(happcache), pcwszurl.into_param().abi(), frandomread.into_param().abi(), ::core::mem::transmute(pcacheentryinfo), ::core::mem::transmute(phentrystream)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheServer() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheServer() -> u32;
        }
        ::core::mem::transmute(UrlCacheServer())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UrlCacheSetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, ulllimit: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheSetGlobalLimit(limittype: URL_CACHE_LIMIT_TYPE, ulllimit: u64) -> u32;
        }
        ::core::mem::transmute(UrlCacheSetGlobalLimit(::core::mem::transmute(limittype), ::core::mem::transmute(ulllimit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UrlCacheUpdateEntryExtraData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(happcache: *const ::core::ffi::c_void, pcwszurl: Param1, pbextradata: *const u8, cbextradata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UrlCacheUpdateEntryExtraData(happcache: *const ::core::ffi::c_void, pcwszurl: super::super::Foundation::PWSTR, pbextradata: *const u8, cbextradata: u32) -> u32;
        }
        ::core::mem::transmute(UrlCacheUpdateEntryExtraData(::core::mem::transmute(happcache), pcwszurl.into_param().abi(), ::core::mem::transmute(pbextradata), ::core::mem::transmute(cbextradata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WININET_API_FLAG_ASYNC: u32 = 1u32;
pub const WININET_API_FLAG_SYNC: u32 = 4u32;
pub const WININET_API_FLAG_USE_CONTEXT: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WININET_PROXY_INFO {
    pub fProxy: super::super::Foundation::BOOL,
    pub fBypass: super::super::Foundation::BOOL,
    pub ProxyScheme: INTERNET_SCHEME,
    pub pwszProxy: super::super::Foundation::PWSTR,
    pub ProxyPort: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl WININET_PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WININET_PROXY_INFO").field("fProxy", &self.fProxy).field("fBypass", &self.fBypass).field("ProxyScheme", &self.ProxyScheme).field("pwszProxy", &self.pwszProxy).field("ProxyPort", &self.ProxyPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fProxy == other.fProxy && self.fBypass == other.fBypass && self.ProxyScheme == other.ProxyScheme && self.pwszProxy == other.pwszProxy && self.ProxyPort == other.ProxyPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WININET_PROXY_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WININET_PROXY_INFO_LIST {
    pub dwProxyInfoCount: u32,
    pub pProxyInfo: *mut WININET_PROXY_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl WININET_PROXY_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WININET_PROXY_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WININET_PROXY_INFO_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WININET_PROXY_INFO_LIST").field("dwProxyInfoCount", &self.dwProxyInfoCount).field("pProxyInfo", &self.pProxyInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WININET_PROXY_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwProxyInfoCount == other.dwProxyInfoCount && self.pProxyInfo == other.pProxyInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WININET_PROXY_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WININET_PROXY_INFO_LIST {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WININET_SYNC_MODE(pub i32);
pub const WININET_SYNC_MODE_NEVER: WININET_SYNC_MODE = WININET_SYNC_MODE(0i32);
pub const WININET_SYNC_MODE_ON_EXPIRY: WININET_SYNC_MODE = WININET_SYNC_MODE(1i32);
pub const WININET_SYNC_MODE_ONCE_PER_SESSION: WININET_SYNC_MODE = WININET_SYNC_MODE(2i32);
pub const WININET_SYNC_MODE_ALWAYS: WININET_SYNC_MODE = WININET_SYNC_MODE(3i32);
pub const WININET_SYNC_MODE_AUTOMATIC: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
pub const WININET_SYNC_MODE_DEFAULT: WININET_SYNC_MODE = WININET_SYNC_MODE(4i32);
impl ::core::convert::From<i32> for WININET_SYNC_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WININET_SYNC_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPAD_CACHE_DELETE(pub i32);
pub const WPAD_CACHE_DELETE_CURRENT: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(0i32);
pub const WPAD_CACHE_DELETE_ALL: WPAD_CACHE_DELETE = WPAD_CACHE_DELETE(1i32);
impl ::core::convert::From<i32> for WPAD_CACHE_DELETE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WPAD_CACHE_DELETE {
    type Abi = Self;
}
pub const XDR_CACHE_ENTRY: u32 = 262144u32;
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetDeInitializeAutoProxyDll = unsafe extern "system" fn(lpszmime: super::super::Foundation::PSTR, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetGetProxyInfo = unsafe extern "system" fn(lpszurl: super::super::Foundation::PSTR, dwurllength: u32, lpszurlhostname: super::super::Foundation::PSTR, dwurlhostnamelength: u32, lplpszproxyhostname: *mut super::super::Foundation::PSTR, lpdwproxyhostnamelength: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type pfnInternetInitializeAutoProxyDll = unsafe extern "system" fn(dwversion: u32, lpszdownloadedtempfile: super::super::Foundation::PSTR, lpszmime: super::super::Foundation::PSTR, lpautoproxycallbacks: *mut AutoProxyHelperFunctions, lpautoproxyscriptbuffer: *mut AUTO_PROXY_SCRIPT_BUFFER) -> super::super::Foundation::BOOL;
