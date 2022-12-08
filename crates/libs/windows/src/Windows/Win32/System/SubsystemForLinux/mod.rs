#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslConfigureDistribution<P0>(distributionname: P0, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslConfigureDistribution ( distributionname : :: windows::core::PCWSTR , defaultuid : u32 , wsldistributionflags : WSL_DISTRIBUTION_FLAGS ) -> :: windows::core::HRESULT );
    WslConfigureDistribution(distributionname.into().abi(), defaultuid, wsldistributionflags).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslGetDistributionConfiguration<P0>(distributionname: P0, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut ::windows::core::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslGetDistributionConfiguration ( distributionname : :: windows::core::PCWSTR , distributionversion : *mut u32 , defaultuid : *mut u32 , wsldistributionflags : *mut WSL_DISTRIBUTION_FLAGS , defaultenvironmentvariables : *mut *mut :: windows::core::PSTR , defaultenvironmentvariablecount : *mut u32 ) -> :: windows::core::HRESULT );
    WslGetDistributionConfiguration(distributionname.into().abi(), distributionversion, defaultuid, wsldistributionflags, defaultenvironmentvariables, defaultenvironmentvariablecount).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslIsDistributionRegistered<P0>(distributionname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslIsDistributionRegistered ( distributionname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WslIsDistributionRegistered(distributionname.into().abi())
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunch<P0, P1, P2, P3, P4, P5>(distributionname: P0, command: P1, usecurrentworkingdirectory: P2, stdin: P3, stdout: P4, stderr: P5) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P4: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P5: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslLaunch ( distributionname : :: windows::core::PCWSTR , command : :: windows::core::PCWSTR , usecurrentworkingdirectory : super::super::Foundation:: BOOL , stdin : super::super::Foundation:: HANDLE , stdout : super::super::Foundation:: HANDLE , stderr : super::super::Foundation:: HANDLE , process : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WslLaunch(distributionname.into().abi(), command.into().abi(), usecurrentworkingdirectory.into(), stdin.into(), stdout.into(), stderr.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunchInteractive<P0, P1, P2>(distributionname: P0, command: P1, usecurrentworkingdirectory: P2) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslLaunchInteractive ( distributionname : :: windows::core::PCWSTR , command : :: windows::core::PCWSTR , usecurrentworkingdirectory : super::super::Foundation:: BOOL , exitcode : *mut u32 ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    WslLaunchInteractive(distributionname.into().abi(), command.into().abi(), usecurrentworkingdirectory.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslRegisterDistribution<P0, P1>(distributionname: P0, targzfilename: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslRegisterDistribution ( distributionname : :: windows::core::PCWSTR , targzfilename : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    WslRegisterDistribution(distributionname.into().abi(), targzfilename.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_SubsystemForLinux\"`*"]
#[inline]
pub unsafe fn WslUnregisterDistribution<P0>(distributionname: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-wsl-api-l1-1-0.dll""system" fn WslUnregisterDistribution ( distributionname : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    WslUnregisterDistribution(distributionname.into().abi()).ok()
}
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
