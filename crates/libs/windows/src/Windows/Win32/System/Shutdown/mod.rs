#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AbortSystemShutdownA<'a, P0>(lpmachinename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AbortSystemShutdownA(lpmachinename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    AbortSystemShutdownA(lpmachinename.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AbortSystemShutdownW<'a, P0>(lpmachinename: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AbortSystemShutdownW(lpmachinename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    AbortSystemShutdownW(lpmachinename.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckForHiberboot<'a, P0>(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CheckForHiberboot(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: super::super::Foundation::BOOLEAN) -> u32;
    }
    CheckForHiberboot(::core::mem::transmute(phiberboot), bclearflag.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EXIT_WINDOWS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_HYBRID_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_LOGOFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_POWEROFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_REBOOT: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_RESTARTAPPS: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const EWX_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(1u32);
impl ::core::marker::Copy for EXIT_WINDOWS_FLAGS {}
impl ::core::clone::Clone for EXIT_WINDOWS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXIT_WINDOWS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EXIT_WINDOWS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXIT_WINDOWS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXIT_WINDOWS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
    }
    ExitWindowsEx(uflags, dwreason)
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
#[inline]
pub unsafe fn InitiateShutdownA<'a, P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateShutdownA(lpmachinename: ::windows::core::PCSTR, lpmessage: ::windows::core::PCSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
    }
    InitiateShutdownA(lpmachinename.into(), lpmessage.into(), dwgraceperiod, dwshutdownflags, dwreason)
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
#[inline]
pub unsafe fn InitiateShutdownW<'a, P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateShutdownW(lpmachinename: ::windows::core::PCWSTR, lpmessage: ::windows::core::PCWSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
    }
    InitiateShutdownW(lpmachinename.into(), lpmessage.into(), dwgraceperiod, dwshutdownflags, dwreason)
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitiateSystemShutdownA<'a, P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateSystemShutdownA(lpmachinename: ::windows::core::PCSTR, lpmessage: ::windows::core::PCSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    InitiateSystemShutdownA(lpmachinename.into(), lpmessage.into(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitiateSystemShutdownExA<'a, P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateSystemShutdownExA(lpmachinename: ::windows::core::PCSTR, lpmessage: ::windows::core::PCSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
    }
    InitiateSystemShutdownExA(lpmachinename.into(), lpmessage.into(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into(), dwreason)
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitiateSystemShutdownExW<'a, P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateSystemShutdownExW(lpmachinename: ::windows::core::PCWSTR, lpmessage: ::windows::core::PCWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
    }
    InitiateSystemShutdownExW(lpmachinename.into(), lpmessage.into(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into(), dwreason)
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitiateSystemShutdownW<'a, P0, P1, P2, P3>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: P2, brebootaftershutdown: P3) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitiateSystemShutdownW(lpmachinename: ::windows::core::PCWSTR, lpmessage: ::windows::core::PCWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    InitiateSystemShutdownW(lpmachinename.into(), lpmessage.into(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockWorkStation() -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LockWorkStation() -> super::super::Foundation::BOOL;
    }
    LockWorkStation()
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const MAX_NUM_REASONS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHUTDOWN_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_FORCE_OTHERS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_FORCE_SELF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_RESTART: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_POWEROFF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_NOREBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_GRACE_OVERRIDE: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_INSTALL_UPDATES: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_RESTARTAPPS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_HYBRID: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_RESTART_BOOTOPTIONS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_SOFT_REBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_MOBILE_UI: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_ARSO: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_VAIL_CONTAINER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_SYSTEM_INITIATED: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(65536u32);
impl ::core::marker::Copy for SHUTDOWN_FLAGS {}
impl ::core::clone::Clone for SHUTDOWN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHUTDOWN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHUTDOWN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHUTDOWN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHUTDOWN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHUTDOWN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SHUTDOWN_REASON(pub u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_COMMENT_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(16777216u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_DIRTY_PROBLEM_ID_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(33554432u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_CLEAN_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(67108864u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_DIRTY_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(134217728u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_MOBILE_UI_RESERVED: SHUTDOWN_REASON = SHUTDOWN_REASON(268435456u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_USER_DEFINED: SHUTDOWN_REASON = SHUTDOWN_REASON(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_FLAG_PLANNED: SHUTDOWN_REASON = SHUTDOWN_REASON(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_HARDWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(65536u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_OPERATINGSYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(131072u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_SOFTWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(196608u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_APPLICATION: SHUTDOWN_REASON = SHUTDOWN_REASON(262144u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_SYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(327680u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_POWER: SHUTDOWN_REASON = SHUTDOWN_REASON(393216u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MAJOR_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(458752u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_MAINTENANCE: SHUTDOWN_REASON = SHUTDOWN_REASON(1u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_INSTALLATION: SHUTDOWN_REASON = SHUTDOWN_REASON(2u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_UPGRADE: SHUTDOWN_REASON = SHUTDOWN_REASON(3u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_RECONFIG: SHUTDOWN_REASON = SHUTDOWN_REASON(4u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_HUNG: SHUTDOWN_REASON = SHUTDOWN_REASON(5u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_UNSTABLE: SHUTDOWN_REASON = SHUTDOWN_REASON(6u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_DISK: SHUTDOWN_REASON = SHUTDOWN_REASON(7u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_PROCESSOR: SHUTDOWN_REASON = SHUTDOWN_REASON(8u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_NETWORKCARD: SHUTDOWN_REASON = SHUTDOWN_REASON(9u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_POWER_SUPPLY: SHUTDOWN_REASON = SHUTDOWN_REASON(10u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_CORDUNPLUGGED: SHUTDOWN_REASON = SHUTDOWN_REASON(11u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_ENVIRONMENT: SHUTDOWN_REASON = SHUTDOWN_REASON(12u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_HARDWARE_DRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(13u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_OTHERDRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(14u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_BLUESCREEN: SHUTDOWN_REASON = SHUTDOWN_REASON(15u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SERVICEPACK: SHUTDOWN_REASON = SHUTDOWN_REASON(16u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_HOTFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(17u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SECURITYFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(18u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SECURITY: SHUTDOWN_REASON = SHUTDOWN_REASON(19u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_NETWORK_CONNECTIVITY: SHUTDOWN_REASON = SHUTDOWN_REASON(20u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_WMI: SHUTDOWN_REASON = SHUTDOWN_REASON(21u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SERVICEPACK_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(22u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_HOTFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(23u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SECURITYFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(24u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_MMC: SHUTDOWN_REASON = SHUTDOWN_REASON(25u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_SYSTEMRESTORE: SHUTDOWN_REASON = SHUTDOWN_REASON(26u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_TERMSRV: SHUTDOWN_REASON = SHUTDOWN_REASON(32u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_DC_PROMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(33u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_MINOR_DC_DEMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(34u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_UNKNOWN: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(2147942400u32);
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHTDN_REASON_VALID_BIT_MASK: SHUTDOWN_REASON = SHUTDOWN_REASON(3238002687u32);
impl ::core::marker::Copy for SHUTDOWN_REASON {}
impl ::core::clone::Clone for SHUTDOWN_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHUTDOWN_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHUTDOWN_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHUTDOWN_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHUTDOWN_REASON").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SHUTDOWN_REASON {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SHUTDOWN_REASON {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SHUTDOWN_REASON {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SHUTDOWN_REASON {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SHUTDOWN_REASON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`*"]
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShutdownBlockReasonCreate<'a, P0, P1>(hwnd: P0, pwszreason: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShutdownBlockReasonCreate(hwnd: super::super::Foundation::HWND, pwszreason: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ShutdownBlockReasonCreate(hwnd.into(), pwszreason.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShutdownBlockReasonDestroy<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShutdownBlockReasonDestroy(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    ShutdownBlockReasonDestroy(hwnd.into())
}
#[doc = "*Required features: `\"Win32_System_Shutdown\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShutdownBlockReasonQuery<'a, P0>(hwnd: P0, pwszbuff: ::windows::core::PWSTR, pcchbuff: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ShutdownBlockReasonQuery(hwnd: super::super::Foundation::HWND, pwszbuff: ::windows::core::PWSTR, pcchbuff: *mut u32) -> super::super::Foundation::BOOL;
    }
    ShutdownBlockReasonQuery(hwnd.into(), ::core::mem::transmute(pwszbuff), ::core::mem::transmute(pcchbuff))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
