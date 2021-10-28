#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_APP_STATUS(pub i32);
pub const RmStatusUnknown: RM_APP_STATUS = RM_APP_STATUS(0i32);
pub const RmStatusRunning: RM_APP_STATUS = RM_APP_STATUS(1i32);
pub const RmStatusStopped: RM_APP_STATUS = RM_APP_STATUS(2i32);
pub const RmStatusStoppedOther: RM_APP_STATUS = RM_APP_STATUS(4i32);
pub const RmStatusRestarted: RM_APP_STATUS = RM_APP_STATUS(8i32);
pub const RmStatusErrorOnStop: RM_APP_STATUS = RM_APP_STATUS(16i32);
pub const RmStatusErrorOnRestart: RM_APP_STATUS = RM_APP_STATUS(32i32);
pub const RmStatusShutdownMasked: RM_APP_STATUS = RM_APP_STATUS(64i32);
pub const RmStatusRestartMasked: RM_APP_STATUS = RM_APP_STATUS(128i32);
impl ::std::convert::From<i32> for RM_APP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_APP_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_APP_TYPE(pub i32);
pub const RmUnknownApp: RM_APP_TYPE = RM_APP_TYPE(0i32);
pub const RmMainWindow: RM_APP_TYPE = RM_APP_TYPE(1i32);
pub const RmOtherWindow: RM_APP_TYPE = RM_APP_TYPE(2i32);
pub const RmService: RM_APP_TYPE = RM_APP_TYPE(3i32);
pub const RmExplorer: RM_APP_TYPE = RM_APP_TYPE(4i32);
pub const RmConsole: RM_APP_TYPE = RM_APP_TYPE(5i32);
pub const RmCritical: RM_APP_TYPE = RM_APP_TYPE(1000i32);
impl ::std::convert::From<i32> for RM_APP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_APP_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_FILTER_ACTION(pub i32);
pub const RmInvalidFilterAction: RM_FILTER_ACTION = RM_FILTER_ACTION(0i32);
pub const RmNoRestart: RM_FILTER_ACTION = RM_FILTER_ACTION(1i32);
pub const RmNoShutdown: RM_FILTER_ACTION = RM_FILTER_ACTION(2i32);
impl ::std::convert::From<i32> for RM_FILTER_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_FILTER_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RM_FILTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RM_FILTER_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RM_FILTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RM_FILTER_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RM_FILTER_INFO_0 {
    pub strFilename: super::super::Foundation::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl RM_FILTER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RM_FILTER_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RM_FILTER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RM_FILTER_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_FILTER_TRIGGER(pub i32);
pub const RmFilterTriggerInvalid: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(0i32);
pub const RmFilterTriggerFile: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(1i32);
pub const RmFilterTriggerProcess: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(2i32);
pub const RmFilterTriggerService: RM_FILTER_TRIGGER = RM_FILTER_TRIGGER(3i32);
impl ::std::convert::From<i32> for RM_FILTER_TRIGGER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_FILTER_TRIGGER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RM_INVALID_PROCESS: i32 = -1i32;
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RM_PROCESS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_PROCESS_INFO")
            .field("Process", &self.Process)
            .field("strAppName", &self.strAppName)
            .field("strServiceShortName", &self.strServiceShortName)
            .field("ApplicationType", &self.ApplicationType)
            .field("AppStatus", &self.AppStatus)
            .field("TSSessionId", &self.TSSessionId)
            .field("bRestartable", &self.bRestartable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RM_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Process == other.Process
            && self.strAppName == other.strAppName
            && self.strServiceShortName == other.strServiceShortName
            && self.ApplicationType == other.ApplicationType
            && self.AppStatus == other.AppStatus
            && self.TSSessionId == other.TSSessionId
            && self.bRestartable == other.bRestartable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RM_PROCESS_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_REBOOT_REASON(pub i32);
pub const RmRebootReasonNone: RM_REBOOT_REASON = RM_REBOOT_REASON(0i32);
pub const RmRebootReasonPermissionDenied: RM_REBOOT_REASON = RM_REBOOT_REASON(1i32);
pub const RmRebootReasonSessionMismatch: RM_REBOOT_REASON = RM_REBOOT_REASON(2i32);
pub const RmRebootReasonCriticalProcess: RM_REBOOT_REASON = RM_REBOOT_REASON(4i32);
pub const RmRebootReasonCriticalService: RM_REBOOT_REASON = RM_REBOOT_REASON(8i32);
pub const RmRebootReasonDetectedSelf: RM_REBOOT_REASON = RM_REBOOT_REASON(16i32);
impl ::std::convert::From<i32> for RM_REBOOT_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_REBOOT_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RM_SHUTDOWN_TYPE(pub i32);
pub const RmForceShutdown: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(1i32);
pub const RmShutdownOnlyRegistered: RM_SHUTDOWN_TYPE = RM_SHUTDOWN_TYPE(16i32);
impl ::std::convert::From<i32> for RM_SHUTDOWN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RM_SHUTDOWN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RM_UNIQUE_PROCESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RM_UNIQUE_PROCESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RM_UNIQUE_PROCESS")
            .field("dwProcessId", &self.dwProcessId)
            .field("ProcessStartTime", &self.ProcessStartTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RM_UNIQUE_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwProcessId == other.dwProcessId && self.ProcessStartTime == other.ProcessStartTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RM_UNIQUE_PROCESS {
    type Abi = Self;
    type DefaultType = Self;
}
pub type RM_WRITE_STATUS_CALLBACK = unsafe extern "system" fn(npercentcomplete: u32);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmAddFilter<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwsessionhandle: u32,
    strmodulename: Param1,
    pprocess: *const RM_UNIQUE_PROCESS,
    strserviceshortname: Param3,
    filteraction: RM_FILTER_ACTION,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmAddFilter(
                dwsessionhandle: u32,
                strmodulename: super::super::Foundation::PWSTR,
                pprocess: *const RM_UNIQUE_PROCESS,
                strserviceshortname: super::super::Foundation::PWSTR,
                filteraction: RM_FILTER_ACTION,
            ) -> u32;
        }
        ::std::mem::transmute(RmAddFilter(
            ::std::mem::transmute(dwsessionhandle),
            strmodulename.into_param().abi(),
            ::std::mem::transmute(pprocess),
            strserviceshortname.into_param().abi(),
            ::std::mem::transmute(filteraction),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32;
        }
        ::std::mem::transmute(RmCancelCurrentTask(::std::mem::transmute(dwsessionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmEndSession(dwsessionhandle: u32) -> u32;
        }
        ::std::mem::transmute(RmEndSession(::std::mem::transmute(dwsessionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RmGetFilterList(
    dwsessionhandle: u32,
    pbfilterbuf: *mut u8,
    cbfilterbuf: u32,
    cbfilterbufneeded: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmGetFilterList(
                dwsessionhandle: u32,
                pbfilterbuf: *mut u8,
                cbfilterbuf: u32,
                cbfilterbufneeded: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(RmGetFilterList(
            ::std::mem::transmute(dwsessionhandle),
            ::std::mem::transmute(pbfilterbuf),
            ::std::mem::transmute(cbfilterbuf),
            ::std::mem::transmute(cbfilterbufneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmGetList(
    dwsessionhandle: u32,
    pnprocinfoneeded: *mut u32,
    pnprocinfo: *mut u32,
    rgaffectedapps: *mut RM_PROCESS_INFO,
    lpdwrebootreasons: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmGetList(
                dwsessionhandle: u32,
                pnprocinfoneeded: *mut u32,
                pnprocinfo: *mut u32,
                rgaffectedapps: *mut RM_PROCESS_INFO,
                lpdwrebootreasons: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(RmGetList(
            ::std::mem::transmute(dwsessionhandle),
            ::std::mem::transmute(pnprocinfoneeded),
            ::std::mem::transmute(pnprocinfo),
            ::std::mem::transmute(rgaffectedapps),
            ::std::mem::transmute(lpdwrebootreasons),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmJoinSession<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    psessionhandle: *mut u32,
    strsessionkey: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmJoinSession(
                psessionhandle: *mut u32,
                strsessionkey: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(RmJoinSession(
            ::std::mem::transmute(psessionhandle),
            strsessionkey.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmRegisterResources(
    dwsessionhandle: u32,
    nfiles: u32,
    rgsfilenames: *const super::super::Foundation::PWSTR,
    napplications: u32,
    rgapplications: *const RM_UNIQUE_PROCESS,
    nservices: u32,
    rgsservicenames: *const super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmRegisterResources(
                dwsessionhandle: u32,
                nfiles: u32,
                rgsfilenames: *const super::super::Foundation::PWSTR,
                napplications: u32,
                rgapplications: *const RM_UNIQUE_PROCESS,
                nservices: u32,
                rgsservicenames: *const super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(RmRegisterResources(
            ::std::mem::transmute(dwsessionhandle),
            ::std::mem::transmute(nfiles),
            ::std::mem::transmute(rgsfilenames),
            ::std::mem::transmute(napplications),
            ::std::mem::transmute(rgapplications),
            ::std::mem::transmute(nservices),
            ::std::mem::transmute(rgsservicenames),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmRemoveFilter<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwsessionhandle: u32,
    strmodulename: Param1,
    pprocess: *const RM_UNIQUE_PROCESS,
    strserviceshortname: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmRemoveFilter(
                dwsessionhandle: u32,
                strmodulename: super::super::Foundation::PWSTR,
                pprocess: *const RM_UNIQUE_PROCESS,
                strserviceshortname: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(RmRemoveFilter(
            ::std::mem::transmute(dwsessionhandle),
            strmodulename.into_param().abi(),
            ::std::mem::transmute(pprocess),
            strserviceshortname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RmRestart(
    dwsessionhandle: u32,
    dwrestartflags: u32,
    fnstatus: ::std::option::Option<RM_WRITE_STATUS_CALLBACK>,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmRestart(
                dwsessionhandle: u32,
                dwrestartflags: u32,
                fnstatus: ::windows::runtime::RawPtr,
            ) -> u32;
        }
        ::std::mem::transmute(RmRestart(
            ::std::mem::transmute(dwsessionhandle),
            ::std::mem::transmute(dwrestartflags),
            ::std::mem::transmute(fnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn RmShutdown(
    dwsessionhandle: u32,
    lactionflags: u32,
    fnstatus: ::std::option::Option<RM_WRITE_STATUS_CALLBACK>,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmShutdown(
                dwsessionhandle: u32,
                lactionflags: u32,
                fnstatus: ::windows::runtime::RawPtr,
            ) -> u32;
        }
        ::std::mem::transmute(RmShutdown(
            ::std::mem::transmute(dwsessionhandle),
            ::std::mem::transmute(lactionflags),
            ::std::mem::transmute(fnstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn RmStartSession(
    psessionhandle: *mut u32,
    dwsessionflags: u32,
    strsessionkey: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RmStartSession(
                psessionhandle: *mut u32,
                dwsessionflags: u32,
                strsessionkey: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(RmStartSession(
            ::std::mem::transmute(psessionhandle),
            ::std::mem::transmute(dwsessionflags),
            ::std::mem::transmute(strsessionkey),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
