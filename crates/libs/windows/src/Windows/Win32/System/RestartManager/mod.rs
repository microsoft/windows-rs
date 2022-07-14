#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_APP_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusUnknown: RM_APP_STATUS = RM_APP_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusRunning: RM_APP_STATUS = RM_APP_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusStopped: RM_APP_STATUS = RM_APP_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusStoppedOther: RM_APP_STATUS = RM_APP_STATUS(4i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusRestarted: RM_APP_STATUS = RM_APP_STATUS(8i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusErrorOnStop: RM_APP_STATUS = RM_APP_STATUS(16i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusErrorOnRestart: RM_APP_STATUS = RM_APP_STATUS(32i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusShutdownMasked: RM_APP_STATUS = RM_APP_STATUS(64i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmStatusRestartMasked: RM_APP_STATUS = RM_APP_STATUS(128i32);
impl ::core::marker::Copy for RM_APP_STATUS {}
impl ::core::clone::Clone for RM_APP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_APP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_APP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_APP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_APP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmUnknownApp: RM_APP_TYPE = RM_APP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmMainWindow: RM_APP_TYPE = RM_APP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmOtherWindow: RM_APP_TYPE = RM_APP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmService: RM_APP_TYPE = RM_APP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmExplorer: RM_APP_TYPE = RM_APP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmConsole: RM_APP_TYPE = RM_APP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmCritical: RM_APP_TYPE = RM_APP_TYPE(1000i32);
impl ::core::marker::Copy for RM_APP_TYPE {}
impl ::core::clone::Clone for RM_APP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_APP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_APP_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_APP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_FILTER_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmInvalidFilterAction: RM_FILTER_ACTION = RM_FILTER_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmNoRestart: RM_FILTER_ACTION = RM_FILTER_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmNoShutdown: RM_FILTER_ACTION = RM_FILTER_ACTION(2i32);
impl ::core::marker::Copy for RM_FILTER_ACTION {}
impl ::core::clone::Clone for RM_FILTER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_FILTER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_FILTER_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_FILTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_FILTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RM_FILTER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_FILTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RM_FILTER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_FILTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RM_FILTER_INFO_0 {
    pub strFilename: ::windows::core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_FILTER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_FILTER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RM_FILTER_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_FILTER_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RM_FILTER_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_FILTER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_FILTER_TRIGGER(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmFilterTriggerInvalid: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(0i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmFilterTriggerFile: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmFilterTriggerProcess: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(2i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmFilterTriggerService: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(3i32);
impl ::core::marker::Copy for RM_FILTER_TRIGGER {}
impl ::core::clone::Clone for RM_FILTER_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_FILTER_TRIGGER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_FILTER_TRIGGER {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_FILTER_TRIGGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_TRIGGER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RM_INVALID_PROCESS: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_PROCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RM_PROCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_PROCESS_INFO").field("Process", &self.Process).field("strAppName", &self.strAppName).field("strServiceShortName", &self.strServiceShortName).field("ApplicationType", &self.ApplicationType).field("AppStatus", &self.AppStatus).field("TSSessionId", &self.TSSessionId).field("bRestartable", &self.bRestartable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RM_PROCESS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RM_PROCESS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_REBOOT_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonNone: RM_REBOOT_REASON = RM_REBOOT_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonPermissionDenied: RM_REBOOT_REASON = RM_REBOOT_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonSessionMismatch: RM_REBOOT_REASON = RM_REBOOT_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonCriticalProcess: RM_REBOOT_REASON = RM_REBOOT_REASON(4i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonCriticalService: RM_REBOOT_REASON = RM_REBOOT_REASON(8i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmRebootReasonDetectedSelf: RM_REBOOT_REASON = RM_REBOOT_REASON(16i32);
impl ::core::marker::Copy for RM_REBOOT_REASON {}
impl ::core::clone::Clone for RM_REBOOT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_REBOOT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_REBOOT_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_REBOOT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_REBOOT_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_SHUTDOWN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmForceShutdown: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub const RmShutdownOnlyRegistered: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(16i32);
impl ::core::marker::Copy for RM_SHUTDOWN_TYPE {}
impl ::core::clone::Clone for RM_SHUTDOWN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RM_SHUTDOWN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RM_SHUTDOWN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RM_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_UNIQUE_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RM_UNIQUE_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RM_UNIQUE_PROCESS").field("dwProcessId", &self.dwProcessId).field("ProcessStartTime", &self.ProcessStartTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RM_UNIQUE_PROCESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_UNIQUE_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RM_UNIQUE_PROCESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_UNIQUE_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
pub type RM_WRITE_STATUS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(npercentcomplete: u32)>;
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmAddFilter<'a, P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: P1, filteraction: RM_FILTER_ACTION) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmAddFilter(dwsessionhandle: u32, strmodulename: ::windows::core::PCWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: ::windows::core::PCWSTR, filteraction: RM_FILTER_ACTION) -> u32;
    }
    RmAddFilter(dwsessionhandle, strmodulename.into(), ::core::mem::transmute(pprocess), strserviceshortname.into(), filteraction)
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32;
    }
    RmCancelCurrentTask(dwsessionhandle)
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmEndSession(dwsessionhandle: u32) -> u32;
    }
    RmEndSession(dwsessionhandle)
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: *mut u8, cbfilterbuf: u32, cbfilterbufneeded: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: *mut u8, cbfilterbuf: u32, cbfilterbufneeded: *mut u32) -> u32;
    }
    RmGetFilterList(dwsessionhandle, ::core::mem::transmute(pbfilterbuf), cbfilterbuf, ::core::mem::transmute(cbfilterbufneeded))
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: *mut RM_PROCESS_INFO, lpdwrebootreasons: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: *mut RM_PROCESS_INFO, lpdwrebootreasons: *mut u32) -> u32;
    }
    RmGetList(dwsessionhandle, ::core::mem::transmute(pnprocinfoneeded), ::core::mem::transmute(pnprocinfo), ::core::mem::transmute(rgaffectedapps), ::core::mem::transmute(lpdwrebootreasons))
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmJoinSession<'a, P0>(psessionhandle: *mut u32, strsessionkey: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmJoinSession(psessionhandle: *mut u32, strsessionkey: ::windows::core::PCWSTR) -> u32;
    }
    RmJoinSession(::core::mem::transmute(psessionhandle), strsessionkey.into())
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: u32, rgsfilenames: &[::windows::core::PWSTR], rgapplications: &[RM_UNIQUE_PROCESS], rgsservicenames: &[::windows::core::PWSTR]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmRegisterResources(dwsessionhandle: u32, nfiles: u32, rgsfilenames: *const ::windows::core::PWSTR, napplications: u32, rgapplications: *const RM_UNIQUE_PROCESS, nservices: u32, rgsservicenames: *const ::windows::core::PWSTR) -> u32;
    }
    RmRegisterResources(dwsessionhandle, rgsfilenames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsfilenames)), rgapplications.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgapplications)), rgsservicenames.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsservicenames)))
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmRemoveFilter<'a, P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmRemoveFilter(dwsessionhandle: u32, strmodulename: ::windows::core::PCWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: ::windows::core::PCWSTR) -> u32;
    }
    RmRemoveFilter(dwsessionhandle, strmodulename.into(), ::core::mem::transmute(pprocess), strserviceshortname.into())
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: *mut ::core::ffi::c_void) -> u32;
    }
    RmRestart(dwsessionhandle, dwrestartflags, ::core::mem::transmute(fnstatus))
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: *mut ::core::ffi::c_void) -> u32;
    }
    RmShutdown(dwsessionhandle, lactionflags, ::core::mem::transmute(fnstatus))
}
#[doc = "*Required features: `\"Win32_System_RestartManager\"`*"]
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: ::windows::core::PWSTR) -> u32;
    }
    RmStartSession(::core::mem::transmute(psessionhandle), dwsessionflags, ::core::mem::transmute(strsessionkey))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
