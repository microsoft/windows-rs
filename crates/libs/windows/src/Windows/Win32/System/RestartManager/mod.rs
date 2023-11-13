#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmAddFilter<P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: ::core::option::Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P1, filteraction: RM_FILTER_ACTION) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : u32, strmodulename : ::windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : ::windows_core::PCWSTR, filteraction : RM_FILTER_ACTION) -> super::super::Foundation:: WIN32_ERROR);
    RmAddFilter(dwsessionhandle, strmodulename.into_param().abi(), ::core::mem::transmute(pprocess.unwrap_or(::std::ptr::null())), strserviceshortname.into_param().abi(), filteraction).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    RmCancelCurrentTask(dwsessionhandle).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : u32) -> super::super::Foundation:: WIN32_ERROR);
    RmEndSession(dwsessionhandle).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: ::core::option::Option<&mut [u8]>, cbfilterbufneeded: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : u32, pbfilterbuf : *mut u8, cbfilterbuf : u32, cbfilterbufneeded : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RmGetFilterList(dwsessionhandle, ::core::mem::transmute(pbfilterbuf.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pbfilterbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cbfilterbufneeded).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: ::core::option::Option<*mut RM_PROCESS_INFO>, lpdwrebootreasons: *mut u32) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : u32, pnprocinfoneeded : *mut u32, pnprocinfo : *mut u32, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : *mut u32) -> super::super::Foundation:: WIN32_ERROR);
    RmGetList(dwsessionhandle, pnprocinfoneeded, pnprocinfo, ::core::mem::transmute(rgaffectedapps.unwrap_or(::std::ptr::null_mut())), lpdwrebootreasons).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmJoinSession<P0>(psessionhandle: *mut u32, strsessionkey: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut u32, strsessionkey : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmJoinSession(psessionhandle, strsessionkey.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: u32, rgsfilenames: ::core::option::Option<&[::windows_core::PCWSTR]>, rgapplications: ::core::option::Option<&[RM_UNIQUE_PROCESS]>, rgsservicenames: ::core::option::Option<&[::windows_core::PCWSTR]>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : u32, nfiles : u32, rgsfilenames : *const ::windows_core::PCWSTR, napplications : u32, rgapplications : *const RM_UNIQUE_PROCESS, nservices : u32, rgsservicenames : *const ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmRegisterResources(
        dwsessionhandle,
        rgsfilenames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        ::core::mem::transmute(rgsfilenames.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        rgapplications.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        ::core::mem::transmute(rgapplications.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        rgsservicenames.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        ::core::mem::transmute(rgsservicenames.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
    )
    .ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmRemoveFilter<P0, P1>(dwsessionhandle: u32, strmodulename: P0, pprocess: ::core::option::Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : u32, strmodulename : ::windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : ::windows_core::PCWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmRemoveFilter(dwsessionhandle, strmodulename.into_param().abi(), ::core::mem::transmute(pprocess.unwrap_or(::std::ptr::null())), strserviceshortname.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : u32, dwrestartflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    RmRestart(dwsessionhandle, dwrestartflags, fnstatus).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : u32, lactionflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::super::Foundation:: WIN32_ERROR);
    RmShutdown(dwsessionhandle, lactionflags, fnstatus).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: ::windows_core::PWSTR) -> ::windows_core::Result<()> {
    ::windows_targets::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut u32, dwsessionflags : u32, strsessionkey : ::windows_core::PWSTR) -> super::super::Foundation:: WIN32_ERROR);
    RmStartSession(psessionhandle, dwsessionflags, ::core::mem::transmute(strsessionkey)).ok()
}
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
pub const RM_INVALID_PROCESS: i32 = -1i32;
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
pub const RmConsole: RM_APP_TYPE = RM_APP_TYPE(5i32);
pub const RmCritical: RM_APP_TYPE = RM_APP_TYPE(1000i32);
pub const RmExplorer: RM_APP_TYPE = RM_APP_TYPE(4i32);
pub const RmFilterTriggerFile: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(1i32);
pub const RmFilterTriggerInvalid: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(0i32);
pub const RmFilterTriggerProcess: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(2i32);
pub const RmFilterTriggerService: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(3i32);
pub const RmForceShutdown: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(1i32);
pub const RmInvalidFilterAction: RM_FILTER_ACTION = RM_FILTER_ACTION(0i32);
pub const RmMainWindow: RM_APP_TYPE = RM_APP_TYPE(1i32);
pub const RmNoRestart: RM_FILTER_ACTION = RM_FILTER_ACTION(1i32);
pub const RmNoShutdown: RM_FILTER_ACTION = RM_FILTER_ACTION(2i32);
pub const RmOtherWindow: RM_APP_TYPE = RM_APP_TYPE(2i32);
pub const RmRebootReasonCriticalProcess: RM_REBOOT_REASON = RM_REBOOT_REASON(4i32);
pub const RmRebootReasonCriticalService: RM_REBOOT_REASON = RM_REBOOT_REASON(8i32);
pub const RmRebootReasonDetectedSelf: RM_REBOOT_REASON = RM_REBOOT_REASON(16i32);
pub const RmRebootReasonNone: RM_REBOOT_REASON = RM_REBOOT_REASON(0i32);
pub const RmRebootReasonPermissionDenied: RM_REBOOT_REASON = RM_REBOOT_REASON(1i32);
pub const RmRebootReasonSessionMismatch: RM_REBOOT_REASON = RM_REBOOT_REASON(2i32);
pub const RmService: RM_APP_TYPE = RM_APP_TYPE(3i32);
pub const RmShutdownOnlyRegistered: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(16i32);
pub const RmStatusErrorOnRestart: RM_APP_STATUS = RM_APP_STATUS(32i32);
pub const RmStatusErrorOnStop: RM_APP_STATUS = RM_APP_STATUS(16i32);
pub const RmStatusRestartMasked: RM_APP_STATUS = RM_APP_STATUS(128i32);
pub const RmStatusRestarted: RM_APP_STATUS = RM_APP_STATUS(8i32);
pub const RmStatusRunning: RM_APP_STATUS = RM_APP_STATUS(1i32);
pub const RmStatusShutdownMasked: RM_APP_STATUS = RM_APP_STATUS(64i32);
pub const RmStatusStopped: RM_APP_STATUS = RM_APP_STATUS(2i32);
pub const RmStatusStoppedOther: RM_APP_STATUS = RM_APP_STATUS(4i32);
pub const RmStatusUnknown: RM_APP_STATUS = RM_APP_STATUS(0i32);
pub const RmUnknownApp: RM_APP_TYPE = RM_APP_TYPE(0i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_APP_STATUS(pub i32);
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
impl ::windows_core::TypeKind for RM_APP_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_APP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_APP_TYPE(pub i32);
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
impl ::windows_core::TypeKind for RM_APP_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_APP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_APP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_FILTER_ACTION(pub i32);
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
impl ::windows_core::TypeKind for RM_FILTER_ACTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_FILTER_TRIGGER(pub i32);
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
impl ::windows_core::TypeKind for RM_FILTER_TRIGGER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_FILTER_TRIGGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_FILTER_TRIGGER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_REBOOT_REASON(pub i32);
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
impl ::windows_core::TypeKind for RM_REBOOT_REASON {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_REBOOT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_REBOOT_REASON").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RM_SHUTDOWN_TYPE(pub i32);
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
impl ::windows_core::TypeKind for RM_SHUTDOWN_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RM_SHUTDOWN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RM_SHUTDOWN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
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
impl ::windows_core::TypeKind for RM_FILTER_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub union RM_FILTER_INFO_0 {
    pub strFilename: ::windows_core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: ::windows_core::PWSTR,
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
impl ::windows_core::TypeKind for RM_FILTER_INFO_0 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
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
impl ::windows_core::TypeKind for RM_PROCESS_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Process == other.Process && self.strAppName == other.strAppName && self.strServiceShortName == other.strServiceShortName && self.ApplicationType == other.ApplicationType && self.AppStatus == other.AppStatus && self.TSSessionId == other.TSSessionId && self.bRestartable == other.bRestartable
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
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
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
impl ::windows_core::TypeKind for RM_UNIQUE_PROCESS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RM_UNIQUE_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwProcessId == other.dwProcessId && self.ProcessStartTime == other.ProcessStartTime
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
pub type RM_WRITE_STATUS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(npercentcomplete: u32) -> ()>;
