#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn AbortSystemShutdownA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpmachinename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AbortSystemShutdownA(lpmachinename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AbortSystemShutdownA(lpmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn AbortSystemShutdownW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpmachinename: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AbortSystemShutdownW(lpmachinename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AbortSystemShutdownW(lpmachinename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CheckForHiberboot<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckForHiberboot(phiberboot: *mut super::super::Foundation::BOOLEAN, bclearflag: super::super::Foundation::BOOLEAN) -> u32;
        }
        ::std::mem::transmute(CheckForHiberboot(::std::mem::transmute(phiberboot), bclearflag.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Shutdown`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EXIT_WINDOWS_FLAGS(pub u32);
pub const EWX_HYBRID_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(4194304u32);
pub const EWX_LOGOFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(0u32);
pub const EWX_POWEROFF: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(8u32);
pub const EWX_REBOOT: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(2u32);
pub const EWX_RESTARTAPPS: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(64u32);
pub const EWX_SHUTDOWN: EXIT_WINDOWS_FLAGS = EXIT_WINDOWS_FLAGS(1u32);
impl ::std::convert::From<u32> for EXIT_WINDOWS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXIT_WINDOWS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EXIT_WINDOWS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EXIT_WINDOWS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EXIT_WINDOWS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExitWindowsEx(uflags: EXIT_WINDOWS_FLAGS, dwreason: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ExitWindowsEx(::std::mem::transmute(uflags), ::std::mem::transmute(dwreason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateShutdownA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpmachinename: Param0, lpmessage: Param1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateShutdownA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
        }
        ::std::mem::transmute(InitiateShutdownA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwgraceperiod), ::std::mem::transmute(dwshutdownflags), ::std::mem::transmute(dwreason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateShutdownW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpmachinename: Param0, lpmessage: Param1, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateShutdownW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwgraceperiod: u32, dwshutdownflags: SHUTDOWN_FLAGS, dwreason: SHUTDOWN_REASON) -> u32;
        }
        ::std::mem::transmute(InitiateShutdownW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwgraceperiod), ::std::mem::transmute(dwshutdownflags), ::std::mem::transmute(dwreason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateSystemShutdownA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpmachinename: Param0, lpmessage: Param1, dwtimeout: u32, bforceappsclosed: Param3, brebootaftershutdown: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateSystemShutdownA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitiateSystemShutdownA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwtimeout), bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateSystemShutdownExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    lpmachinename: Param0,
    lpmessage: Param1,
    dwtimeout: u32,
    bforceappsclosed: Param3,
    brebootaftershutdown: Param4,
    dwreason: SHUTDOWN_REASON,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateSystemShutdownExA(lpmachinename: super::super::Foundation::PSTR, lpmessage: super::super::Foundation::PSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitiateSystemShutdownExA(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwtimeout), bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi(), ::std::mem::transmute(dwreason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateSystemShutdownExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(
    lpmachinename: Param0,
    lpmessage: Param1,
    dwtimeout: u32,
    bforceappsclosed: Param3,
    brebootaftershutdown: Param4,
    dwreason: SHUTDOWN_REASON,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateSystemShutdownExW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL, dwreason: SHUTDOWN_REASON) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitiateSystemShutdownExW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwtimeout), bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi(), ::std::mem::transmute(dwreason)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn InitiateSystemShutdownW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpmachinename: Param0, lpmessage: Param1, dwtimeout: u32, bforceappsclosed: Param3, brebootaftershutdown: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitiateSystemShutdownW(lpmachinename: super::super::Foundation::PWSTR, lpmessage: super::super::Foundation::PWSTR, dwtimeout: u32, bforceappsclosed: super::super::Foundation::BOOL, brebootaftershutdown: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitiateSystemShutdownW(lpmachinename.into_param().abi(), lpmessage.into_param().abi(), ::std::mem::transmute(dwtimeout), bforceappsclosed.into_param().abi(), brebootaftershutdown.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn LockWorkStation() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockWorkStation() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LockWorkStation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const MAX_NUM_REASONS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const MAX_REASON_BUGID_LEN: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const MAX_REASON_COMMENT_LEN: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const MAX_REASON_DESC_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const MAX_REASON_NAME_LEN: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const POLICY_SHOWREASONUI_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const POLICY_SHOWREASONUI_NEVER: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const POLICY_SHOWREASONUI_SERVERONLY: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const POLICY_SHOWREASONUI_WORKSTATIONONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHUTDOWN_FLAGS(pub u32);
pub const SHUTDOWN_FORCE_OTHERS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1u32);
pub const SHUTDOWN_FORCE_SELF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2u32);
pub const SHUTDOWN_RESTART: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4u32);
pub const SHUTDOWN_POWEROFF: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8u32);
pub const SHUTDOWN_NOREBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16u32);
pub const SHUTDOWN_GRACE_OVERRIDE: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32u32);
pub const SHUTDOWN_INSTALL_UPDATES: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(64u32);
pub const SHUTDOWN_RESTARTAPPS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(128u32);
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(256u32);
pub const SHUTDOWN_HYBRID: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(512u32);
pub const SHUTDOWN_RESTART_BOOTOPTIONS: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(1024u32);
pub const SHUTDOWN_SOFT_REBOOT: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(2048u32);
pub const SHUTDOWN_MOBILE_UI: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(4096u32);
pub const SHUTDOWN_ARSO: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(8192u32);
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(16384u32);
pub const SHUTDOWN_VAIL_CONTAINER: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(32768u32);
pub const SHUTDOWN_SYSTEM_INITIATED: SHUTDOWN_FLAGS = SHUTDOWN_FLAGS(65536u32);
impl ::std::convert::From<u32> for SHUTDOWN_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHUTDOWN_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SHUTDOWN_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SHUTDOWN_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SHUTDOWN_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SHUTDOWN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Shutdown`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SHUTDOWN_REASON(pub u32);
pub const SHTDN_REASON_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_FLAG_COMMENT_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(16777216u32);
pub const SHTDN_REASON_FLAG_DIRTY_PROBLEM_ID_REQUIRED: SHUTDOWN_REASON = SHUTDOWN_REASON(33554432u32);
pub const SHTDN_REASON_FLAG_CLEAN_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(67108864u32);
pub const SHTDN_REASON_FLAG_DIRTY_UI: SHUTDOWN_REASON = SHUTDOWN_REASON(134217728u32);
pub const SHTDN_REASON_FLAG_MOBILE_UI_RESERVED: SHUTDOWN_REASON = SHUTDOWN_REASON(268435456u32);
pub const SHTDN_REASON_FLAG_USER_DEFINED: SHUTDOWN_REASON = SHUTDOWN_REASON(1073741824u32);
pub const SHTDN_REASON_FLAG_PLANNED: SHUTDOWN_REASON = SHUTDOWN_REASON(2147483648u32);
pub const SHTDN_REASON_MAJOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MAJOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MAJOR_HARDWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(65536u32);
pub const SHTDN_REASON_MAJOR_OPERATINGSYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(131072u32);
pub const SHTDN_REASON_MAJOR_SOFTWARE: SHUTDOWN_REASON = SHUTDOWN_REASON(196608u32);
pub const SHTDN_REASON_MAJOR_APPLICATION: SHUTDOWN_REASON = SHUTDOWN_REASON(262144u32);
pub const SHTDN_REASON_MAJOR_SYSTEM: SHUTDOWN_REASON = SHUTDOWN_REASON(327680u32);
pub const SHTDN_REASON_MAJOR_POWER: SHUTDOWN_REASON = SHUTDOWN_REASON(393216u32);
pub const SHTDN_REASON_MAJOR_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(458752u32);
pub const SHTDN_REASON_MINOR_OTHER: SHUTDOWN_REASON = SHUTDOWN_REASON(0u32);
pub const SHTDN_REASON_MINOR_NONE: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
pub const SHTDN_REASON_MINOR_MAINTENANCE: SHUTDOWN_REASON = SHUTDOWN_REASON(1u32);
pub const SHTDN_REASON_MINOR_INSTALLATION: SHUTDOWN_REASON = SHUTDOWN_REASON(2u32);
pub const SHTDN_REASON_MINOR_UPGRADE: SHUTDOWN_REASON = SHUTDOWN_REASON(3u32);
pub const SHTDN_REASON_MINOR_RECONFIG: SHUTDOWN_REASON = SHUTDOWN_REASON(4u32);
pub const SHTDN_REASON_MINOR_HUNG: SHUTDOWN_REASON = SHUTDOWN_REASON(5u32);
pub const SHTDN_REASON_MINOR_UNSTABLE: SHUTDOWN_REASON = SHUTDOWN_REASON(6u32);
pub const SHTDN_REASON_MINOR_DISK: SHUTDOWN_REASON = SHUTDOWN_REASON(7u32);
pub const SHTDN_REASON_MINOR_PROCESSOR: SHUTDOWN_REASON = SHUTDOWN_REASON(8u32);
pub const SHTDN_REASON_MINOR_NETWORKCARD: SHUTDOWN_REASON = SHUTDOWN_REASON(9u32);
pub const SHTDN_REASON_MINOR_POWER_SUPPLY: SHUTDOWN_REASON = SHUTDOWN_REASON(10u32);
pub const SHTDN_REASON_MINOR_CORDUNPLUGGED: SHUTDOWN_REASON = SHUTDOWN_REASON(11u32);
pub const SHTDN_REASON_MINOR_ENVIRONMENT: SHUTDOWN_REASON = SHUTDOWN_REASON(12u32);
pub const SHTDN_REASON_MINOR_HARDWARE_DRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(13u32);
pub const SHTDN_REASON_MINOR_OTHERDRIVER: SHUTDOWN_REASON = SHUTDOWN_REASON(14u32);
pub const SHTDN_REASON_MINOR_BLUESCREEN: SHUTDOWN_REASON = SHUTDOWN_REASON(15u32);
pub const SHTDN_REASON_MINOR_SERVICEPACK: SHUTDOWN_REASON = SHUTDOWN_REASON(16u32);
pub const SHTDN_REASON_MINOR_HOTFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(17u32);
pub const SHTDN_REASON_MINOR_SECURITYFIX: SHUTDOWN_REASON = SHUTDOWN_REASON(18u32);
pub const SHTDN_REASON_MINOR_SECURITY: SHUTDOWN_REASON = SHUTDOWN_REASON(19u32);
pub const SHTDN_REASON_MINOR_NETWORK_CONNECTIVITY: SHUTDOWN_REASON = SHUTDOWN_REASON(20u32);
pub const SHTDN_REASON_MINOR_WMI: SHUTDOWN_REASON = SHUTDOWN_REASON(21u32);
pub const SHTDN_REASON_MINOR_SERVICEPACK_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(22u32);
pub const SHTDN_REASON_MINOR_HOTFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(23u32);
pub const SHTDN_REASON_MINOR_SECURITYFIX_UNINSTALL: SHUTDOWN_REASON = SHUTDOWN_REASON(24u32);
pub const SHTDN_REASON_MINOR_MMC: SHUTDOWN_REASON = SHUTDOWN_REASON(25u32);
pub const SHTDN_REASON_MINOR_SYSTEMRESTORE: SHUTDOWN_REASON = SHUTDOWN_REASON(26u32);
pub const SHTDN_REASON_MINOR_TERMSRV: SHUTDOWN_REASON = SHUTDOWN_REASON(32u32);
pub const SHTDN_REASON_MINOR_DC_PROMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(33u32);
pub const SHTDN_REASON_MINOR_DC_DEMOTION: SHUTDOWN_REASON = SHUTDOWN_REASON(34u32);
pub const SHTDN_REASON_UNKNOWN: SHUTDOWN_REASON = SHUTDOWN_REASON(255u32);
pub const SHTDN_REASON_LEGACY_API: SHUTDOWN_REASON = SHUTDOWN_REASON(2147942400u32);
pub const SHTDN_REASON_VALID_BIT_MASK: SHUTDOWN_REASON = SHUTDOWN_REASON(3238002687u32);
impl ::std::convert::From<u32> for SHUTDOWN_REASON {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SHUTDOWN_REASON {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SHUTDOWN_REASON {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SHUTDOWN_REASON {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SHUTDOWN_REASON {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SHUTDOWN_REASON {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SHUTDOWN_REASON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const SHUTDOWN_TYPE_LEN: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const SNAPSHOT_POLICY_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const SNAPSHOT_POLICY_NEVER: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Shutdown`*"]
pub const SNAPSHOT_POLICY_UNPLANNED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ShutdownBlockReasonCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pwszreason: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShutdownBlockReasonCreate(hwnd: super::super::Foundation::HWND, pwszreason: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ShutdownBlockReasonCreate(hwnd.into_param().abi(), pwszreason.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ShutdownBlockReasonDestroy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShutdownBlockReasonDestroy(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ShutdownBlockReasonDestroy(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Shutdown`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ShutdownBlockReasonQuery<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pwszbuff: super::super::Foundation::PWSTR, pcchbuff: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShutdownBlockReasonQuery(hwnd: super::super::Foundation::HWND, pwszbuff: super::super::Foundation::PWSTR, pcchbuff: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ShutdownBlockReasonQuery(hwnd.into_param().abi(), ::std::mem::transmute(pwszbuff), ::std::mem::transmute(pcchbuff)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
