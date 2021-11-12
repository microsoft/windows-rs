#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CMD_ENTRY {
    pub pwszCmdToken: super::super::Foundation::PWSTR,
    pub pfnCmdHandler: ::core::option::Option<PFN_HANDLE_CMD>,
    pub dwShortCmdHelpToken: u32,
    pub dwCmdHlpToken: u32,
    pub dwFlags: u32,
    pub pOsVersionCheck: ::core::option::Option<PNS_OSVERSIONCHECK>,
}
#[cfg(feature = "Win32_Foundation")]
impl CMD_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMD_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CMD_ENTRY").field("pwszCmdToken", &self.pwszCmdToken).field("dwShortCmdHelpToken", &self.dwShortCmdHelpToken).field("dwCmdHlpToken", &self.dwCmdHlpToken).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszCmdToken == other.pwszCmdToken && self.pfnCmdHandler.map(|f| f as usize) == other.pfnCmdHandler.map(|f| f as usize) && self.dwShortCmdHelpToken == other.dwShortCmdHelpToken && self.dwCmdHlpToken == other.dwCmdHlpToken && self.dwFlags == other.dwFlags && self.pOsVersionCheck.map(|f| f as usize) == other.pOsVersionCheck.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMD_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CMD_ENTRY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CMD_GROUP_ENTRY {
    pub pwszCmdGroupToken: super::super::Foundation::PWSTR,
    pub dwShortCmdHelpToken: u32,
    pub ulCmdGroupSize: u32,
    pub dwFlags: u32,
    pub pCmdGroup: *mut CMD_ENTRY,
    pub pOsVersionCheck: ::core::option::Option<PNS_OSVERSIONCHECK>,
}
#[cfg(feature = "Win32_Foundation")]
impl CMD_GROUP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CMD_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CMD_GROUP_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CMD_GROUP_ENTRY").field("pwszCmdGroupToken", &self.pwszCmdGroupToken).field("dwShortCmdHelpToken", &self.dwShortCmdHelpToken).field("ulCmdGroupSize", &self.ulCmdGroupSize).field("dwFlags", &self.dwFlags).field("pCmdGroup", &self.pCmdGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CMD_GROUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.pwszCmdGroupToken == other.pwszCmdGroupToken && self.dwShortCmdHelpToken == other.dwShortCmdHelpToken && self.ulCmdGroupSize == other.ulCmdGroupSize && self.dwFlags == other.dwFlags && self.pCmdGroup == other.pCmdGroup && self.pOsVersionCheck.map(|f| f as usize) == other.pOsVersionCheck.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CMD_GROUP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CMD_GROUP_ENTRY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const DEFAULT_CONTEXT_PRIORITY: u32 = 100u32;
pub const ERROR_CMD_NOT_FOUND: u32 = 15004u32;
pub const ERROR_CONTEXT_ALREADY_REGISTERED: u32 = 15019u32;
pub const ERROR_CONTINUE_IN_PARENT_CONTEXT: u32 = 15016u32;
pub const ERROR_DLL_LOAD_FAILED: u32 = 15006u32;
pub const ERROR_ENTRY_PT_NOT_FOUND: u32 = 15005u32;
pub const ERROR_HELPER_ALREADY_REGISTERED: u32 = 15018u32;
pub const ERROR_INIT_DISPLAY: u32 = 15007u32;
pub const ERROR_INVALID_OPTION_TAG: u32 = 15009u32;
pub const ERROR_INVALID_OPTION_VALUE: u32 = 15014u32;
pub const ERROR_INVALID_SYNTAX: u32 = 15001u32;
pub const ERROR_MISSING_OPTION: u32 = 15011u32;
pub const ERROR_NO_CHANGE: u32 = 15003u32;
pub const ERROR_NO_ENTRIES: u32 = 15000u32;
pub const ERROR_NO_TAG: u32 = 15010u32;
pub const ERROR_OKAY: u32 = 15015u32;
pub const ERROR_PARSING_FAILURE: u32 = 15020u32;
pub const ERROR_PROTOCOL_NOT_IN_TRANSPORT: u32 = 15002u32;
pub const ERROR_SHOW_USAGE: u32 = 15013u32;
pub const ERROR_SUPPRESS_OUTPUT: u32 = 15017u32;
pub const ERROR_TAG_ALREADY_PRESENT: u32 = 15008u32;
pub const ERROR_TRANSPORT_NOT_PRESENT: u32 = 15012u32;
pub const MAX_NAME_LEN: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MatchEnumTag<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, pwcarg: Param1, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MatchEnumTag(hmodule: super::super::Foundation::HANDLE, pwcarg: super::super::Foundation::PWSTR, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32;
        }
        ::core::mem::transmute(MatchEnumTag(hmodule.into_param().abi(), pwcarg.into_param().abi(), ::core::mem::transmute(dwnumarg), ::core::mem::transmute(penumtable), ::core::mem::transmute(pdwvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MatchToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszusertoken: Param0, pwszcmdtoken: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MatchToken(pwszusertoken: super::super::Foundation::PWSTR, pwszcmdtoken: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MatchToken(pwszusertoken.into_param().abi(), pwszcmdtoken.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const NETSH_ERROR_BASE: u32 = 15000u32;
pub const NETSH_ERROR_END: u32 = 15019u32;
pub const NETSH_MAX_CMD_TOKEN_LENGTH: u32 = 128u32;
pub const NETSH_MAX_TOKEN_LENGTH: u32 = 64u32;
pub const NETSH_VERSION_50: u32 = 20480u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NS_CMD_FLAGS(pub i32);
pub const CMD_FLAG_PRIVATE: NS_CMD_FLAGS = NS_CMD_FLAGS(1i32);
pub const CMD_FLAG_INTERACTIVE: NS_CMD_FLAGS = NS_CMD_FLAGS(2i32);
pub const CMD_FLAG_LOCAL: NS_CMD_FLAGS = NS_CMD_FLAGS(8i32);
pub const CMD_FLAG_ONLINE: NS_CMD_FLAGS = NS_CMD_FLAGS(16i32);
pub const CMD_FLAG_HIDDEN: NS_CMD_FLAGS = NS_CMD_FLAGS(32i32);
pub const CMD_FLAG_LIMIT_MASK: NS_CMD_FLAGS = NS_CMD_FLAGS(65535i32);
pub const CMD_FLAG_PRIORITY: NS_CMD_FLAGS = NS_CMD_FLAGS(-2147483648i32);
impl ::core::convert::From<i32> for NS_CMD_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NS_CMD_FLAGS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NS_CONTEXT_ATTRIBUTES {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NS_CONTEXT_ATTRIBUTES {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0,
    pub pwszContext: super::super::Foundation::PWSTR,
    pub guidHelper: ::windows::core::GUID,
    pub dwFlags: u32,
    pub ulPriority: u32,
    pub ulNumTopCmds: u32,
    pub pTopCmds: *mut CMD_ENTRY,
    pub ulNumGroups: u32,
    pub pCmdGroups: *mut CMD_GROUP_ENTRY,
    pub pfnCommitFn: ::core::option::Option<PNS_CONTEXT_COMMIT_FN>,
    pub pfnDumpFn: ::core::option::Option<PNS_CONTEXT_DUMP_FN>,
    pub pfnConnectFn: ::core::option::Option<PNS_CONTEXT_CONNECT_FN>,
    pub pReserved: *mut ::core::ffi::c_void,
    pub pfnOsVersionCheck: ::core::option::Option<PNS_OSVERSIONCHECK>,
}
#[cfg(feature = "Win32_Foundation")]
impl NS_CONTEXT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NS_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NS_CONTEXT_ATTRIBUTES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NS_CONTEXT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NS_CONTEXT_ATTRIBUTES {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union NS_CONTEXT_ATTRIBUTES_0 {
    pub Anonymous: NS_CONTEXT_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl NS_CONTEXT_ATTRIBUTES_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NS_CONTEXT_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NS_CONTEXT_ATTRIBUTES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NS_CONTEXT_ATTRIBUTES_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NS_CONTEXT_ATTRIBUTES_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NS_CONTEXT_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NS_CONTEXT_ATTRIBUTES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NS_CONTEXT_ATTRIBUTES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NS_CONTEXT_ATTRIBUTES_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NS_CONTEXT_ATTRIBUTES_0_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NS_EVENTS(pub i32);
pub const NS_EVENT_LOOP: NS_EVENTS = NS_EVENTS(65536i32);
pub const NS_EVENT_LAST_N: NS_EVENTS = NS_EVENTS(1i32);
pub const NS_EVENT_LAST_SECS: NS_EVENTS = NS_EVENTS(2i32);
pub const NS_EVENT_FROM_N: NS_EVENTS = NS_EVENTS(4i32);
pub const NS_EVENT_FROM_START: NS_EVENTS = NS_EVENTS(8i32);
impl ::core::convert::From<i32> for NS_EVENTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NS_EVENTS {
    type Abi = Self;
}
impl ::core::clone::Clone for NS_HELPER_ATTRIBUTES {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0,
    pub guidHelper: ::windows::core::GUID,
    pub pfnStart: ::core::option::Option<PNS_HELPER_START_FN>,
    pub pfnStop: ::core::option::Option<PNS_HELPER_STOP_FN>,
}
impl NS_HELPER_ATTRIBUTES {}
impl ::core::default::Default for NS_HELPER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NS_HELPER_ATTRIBUTES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for NS_HELPER_ATTRIBUTES {}
unsafe impl ::windows::core::Abi for NS_HELPER_ATTRIBUTES {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union NS_HELPER_ATTRIBUTES_0 {
    pub Anonymous: NS_HELPER_ATTRIBUTES_0_0,
    pub _ullAlign: u64,
}
impl NS_HELPER_ATTRIBUTES_0 {}
impl ::core::default::Default for NS_HELPER_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NS_HELPER_ATTRIBUTES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for NS_HELPER_ATTRIBUTES_0 {}
unsafe impl ::windows::core::Abi for NS_HELPER_ATTRIBUTES_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct NS_HELPER_ATTRIBUTES_0_0 {
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl NS_HELPER_ATTRIBUTES_0_0 {}
impl ::core::default::Default for NS_HELPER_ATTRIBUTES_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for NS_HELPER_ATTRIBUTES_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::cmp::PartialEq for NS_HELPER_ATTRIBUTES_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for NS_HELPER_ATTRIBUTES_0_0 {}
unsafe impl ::windows::core::Abi for NS_HELPER_ATTRIBUTES_0_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NS_MODE_CHANGE(pub i32);
pub const NETSH_COMMIT: NS_MODE_CHANGE = NS_MODE_CHANGE(0i32);
pub const NETSH_UNCOMMIT: NS_MODE_CHANGE = NS_MODE_CHANGE(1i32);
pub const NETSH_FLUSH: NS_MODE_CHANGE = NS_MODE_CHANGE(2i32);
pub const NETSH_COMMIT_STATE: NS_MODE_CHANGE = NS_MODE_CHANGE(3i32);
pub const NETSH_SAVE: NS_MODE_CHANGE = NS_MODE_CHANGE(4i32);
impl ::core::convert::From<i32> for NS_MODE_CHANGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NS_MODE_CHANGE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct NS_REQS(pub i32);
pub const NS_REQ_ZERO: NS_REQS = NS_REQS(0i32);
pub const NS_REQ_PRESENT: NS_REQS = NS_REQS(1i32);
pub const NS_REQ_ALLOW_MULTIPLE: NS_REQS = NS_REQS(2i32);
pub const NS_REQ_ONE_OR_MORE: NS_REQS = NS_REQS(3i32);
impl ::core::convert::From<i32> for NS_REQS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for NS_REQS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HANDLE_CMD = unsafe extern "system" fn(pwszmachine: super::super::Foundation::PWSTR, ppwcarguments: *mut super::super::Foundation::PWSTR, dwcurrentindex: u32, dwargcount: u32, dwflags: u32, pvdata: *const ::core::ffi::c_void, pbdone: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_RESOURCE_STRING_FN = unsafe extern "system" fn(dwmsgid: u32, lpbuffer: super::super::Foundation::PWSTR, nbuffermax: u32) -> u32;
pub type PNS_CONTEXT_COMMIT_FN = unsafe extern "system" fn(dwaction: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PNS_CONTEXT_CONNECT_FN = unsafe extern "system" fn(pwszmachine: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PNS_CONTEXT_DUMP_FN = unsafe extern "system" fn(pwszrouter: super::super::Foundation::PWSTR, ppwcarguments: *const super::super::Foundation::PWSTR, dwargcount: u32, pvdata: *const ::core::ffi::c_void) -> u32;
pub type PNS_DLL_INIT_FN = unsafe extern "system" fn(dwnetshversion: u32, preserved: *mut ::core::ffi::c_void) -> u32;
pub type PNS_DLL_STOP_FN = unsafe extern "system" fn(dwreserved: u32) -> u32;
pub type PNS_HELPER_START_FN = unsafe extern "system" fn(pguidparent: *const ::windows::core::GUID, dwversion: u32) -> u32;
pub type PNS_HELPER_STOP_FN = unsafe extern "system" fn(dwreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PNS_OSVERSIONCHECK = unsafe extern "system" fn(cimostype: u32, cimosproductsuite: u32, cimosversion: super::super::Foundation::PWSTR, cimosbuildnumber: super::super::Foundation::PWSTR, cimservicepackmajorversion: super::super::Foundation::PWSTR, cimservicepackminorversion: super::super::Foundation::PWSTR, uireserved: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PreprocessCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmodule: Param0, ppwcarguments: *mut super::super::Foundation::PWSTR, dwcurrentindex: u32, dwargcount: u32, ptttags: *mut TAG_TYPE, dwtagcount: u32, dwminargs: u32, dwmaxargs: u32, pdwtagtype: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PreprocessCommand(hmodule: super::super::Foundation::HANDLE, ppwcarguments: *mut super::super::Foundation::PWSTR, dwcurrentindex: u32, dwargcount: u32, ptttags: *mut TAG_TYPE, dwtagcount: u32, dwminargs: u32, dwmaxargs: u32, pdwtagtype: *mut u32) -> u32;
        }
        ::core::mem::transmute(PreprocessCommand(
            hmodule.into_param().abi(),
            ::core::mem::transmute(ppwcarguments),
            ::core::mem::transmute(dwcurrentindex),
            ::core::mem::transmute(dwargcount),
            ::core::mem::transmute(ptttags),
            ::core::mem::transmute(dwtagcount),
            ::core::mem::transmute(dwminargs),
            ::core::mem::transmute(dwmaxargs),
            ::core::mem::transmute(pdwtagtype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrintError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmodule: Param0, dwerrid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintError(hmodule: super::super::Foundation::HANDLE, dwerrid: u32) -> u32;
        }
        ::core::mem::transmute(PrintError(hmodule.into_param().abi(), ::core::mem::transmute(dwerrid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrintMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwszformat: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintMessage(pwszformat: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(PrintMessage(pwszformat.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrintMessageFromModule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmodule: Param0, dwmsgid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintMessageFromModule(hmodule: super::super::Foundation::HANDLE, dwmsgid: u32) -> u32;
        }
        ::core::mem::transmute(PrintMessageFromModule(hmodule.into_param().abi(), ::core::mem::transmute(dwmsgid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterContext(pchildcontext: *const NS_CONTEXT_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterContext(pchildcontext: *const ::core::mem::ManuallyDrop<NS_CONTEXT_ATTRIBUTES>) -> u32;
        }
        ::core::mem::transmute(RegisterContext(::core::mem::transmute(pchildcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterHelper(pguidparentcontext: *const ::windows::core::GUID, pfnregistersubcontext: *const NS_HELPER_ATTRIBUTES) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterHelper(pguidparentcontext: *const ::windows::core::GUID, pfnregistersubcontext: *const ::core::mem::ManuallyDrop<NS_HELPER_ATTRIBUTES>) -> u32;
        }
        ::core::mem::transmute(RegisterHelper(::core::mem::transmute(pguidparentcontext), ::core::mem::transmute(pfnregistersubcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TAG_TYPE {
    pub pwszTag: super::super::Foundation::PWSTR,
    pub dwRequired: u32,
    pub bPresent: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl TAG_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TAG_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TAG_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TAG_TYPE").field("pwszTag", &self.pwszTag).field("dwRequired", &self.dwRequired).field("bPresent", &self.bPresent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TAG_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszTag == other.pwszTag && self.dwRequired == other.dwRequired && self.bPresent == other.bPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TAG_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TAG_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_VALUE {
    pub pwszToken: super::super::Foundation::PWSTR,
    pub dwValue: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TOKEN_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TOKEN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TOKEN_VALUE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TOKEN_VALUE").field("pwszToken", &self.pwszToken).field("dwValue", &self.dwValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TOKEN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.pwszToken == other.pwszToken && self.dwValue == other.dwValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TOKEN_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TOKEN_VALUE {
    type Abi = Self;
}
