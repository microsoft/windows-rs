#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSL_DISTRIBUTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(4u32);
impl ::core::marker::Copy for WSL_DISTRIBUTION_FLAGS {}
impl ::core::clone::Clone for WSL_DISTRIBUTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSL_DISTRIBUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WSL_DISTRIBUTION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WSL_DISTRIBUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSL_DISTRIBUTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslConfigureDistribution<'a, Param2: ::std::convert::Into<WSL_DISTRIBUTION_FLAGS>>(distributionname: ::windows::core::PCWSTR, defaultuid: u32, wsldistributionflags: Param2) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslConfigureDistribution(distributionname: ::windows::core::PCWSTR, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows::core::HRESULT;
    }
    WslConfigureDistribution(::core::mem::transmute(distributionname), ::core::mem::transmute(defaultuid), wsldistributionflags.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslGetDistributionConfiguration(distributionname: ::windows::core::PCWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows::core::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslGetDistributionConfiguration(distributionname: ::windows::core::PCWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows::core::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows::core::HRESULT;
    }
    WslGetDistributionConfiguration(::core::mem::transmute(distributionname), ::core::mem::transmute(distributionversion), ::core::mem::transmute(defaultuid), ::core::mem::transmute(wsldistributionflags), ::core::mem::transmute(defaultenvironmentvariables), ::core::mem::transmute(defaultenvironmentvariablecount)).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslIsDistributionRegistered(distributionname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslIsDistributionRegistered(distributionname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ::core::mem::transmute(WslIsDistributionRegistered(::core::mem::transmute(distributionname)))
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunch<'a, Param2: ::std::convert::Into<super::super::Foundation::BOOL>, Param3: ::std::convert::Into<super::super::Foundation::HANDLE>, Param4: ::std::convert::Into<super::super::Foundation::HANDLE>, Param5: ::std::convert::Into<super::super::Foundation::HANDLE>>(distributionname: ::windows::core::PCWSTR, command: ::windows::core::PCWSTR, usecurrentworkingdirectory: Param2, stdin: Param3, stdout: Param4, stderr: Param5) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslLaunch(distributionname: ::windows::core::PCWSTR, command: ::windows::core::PCWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, stdin: super::super::Foundation::HANDLE, stdout: super::super::Foundation::HANDLE, stderr: super::super::Foundation::HANDLE, process: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HANDLE>::zeroed();
    WslLaunch(::core::mem::transmute(distributionname), ::core::mem::transmute(command), usecurrentworkingdirectory.into(), stdin.into(), stdout.into(), stderr.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunchInteractive<'a, Param2: ::std::convert::Into<super::super::Foundation::BOOL>>(distributionname: ::windows::core::PCWSTR, command: ::windows::core::PCWSTR, usecurrentworkingdirectory: Param2) -> ::windows::core::Result<u32> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslLaunchInteractive(distributionname: ::windows::core::PCWSTR, command: ::windows::core::PCWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, exitcode: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
    WslLaunchInteractive(::core::mem::transmute(distributionname), ::core::mem::transmute(command), usecurrentworkingdirectory.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslRegisterDistribution(distributionname: ::windows::core::PCWSTR, targzfilename: ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslRegisterDistribution(distributionname: ::windows::core::PCWSTR, targzfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    WslRegisterDistribution(::core::mem::transmute(distributionname), ::core::mem::transmute(targzfilename)).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslUnregisterDistribution(distributionname: ::windows::core::PCWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WslUnregisterDistribution(distributionname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    WslUnregisterDistribution(::core::mem::transmute(distributionname)).ok()
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
