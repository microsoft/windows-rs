#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSL_DISTRIBUTION_FLAGS(pub u32);
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(0u32);
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(1u32);
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(2u32);
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = WSL_DISTRIBUTION_FLAGS(4u32);
impl ::core::convert::From<u32> for WSL_DISTRIBUTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WSL_DISTRIBUTION_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for WSL_DISTRIBUTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for WSL_DISTRIBUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslConfigureDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(distributionname: Param0, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslConfigureDistribution(distributionname: super::super::Foundation::PWSTR, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows::core::HRESULT;
        }
        WslConfigureDistribution(distributionname.into_param().abi(), ::core::mem::transmute(defaultuid), ::core::mem::transmute(wsldistributionflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslGetDistributionConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(distributionname: Param0, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut super::super::Foundation::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslGetDistributionConfiguration(distributionname: super::super::Foundation::PWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut super::super::Foundation::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows::core::HRESULT;
        }
        WslGetDistributionConfiguration(distributionname.into_param().abi(), ::core::mem::transmute(distributionversion), ::core::mem::transmute(defaultuid), ::core::mem::transmute(wsldistributionflags), ::core::mem::transmute(defaultenvironmentvariables), ::core::mem::transmute(defaultenvironmentvariablecount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslIsDistributionRegistered<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(distributionname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslIsDistributionRegistered(distributionname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WslIsDistributionRegistered(distributionname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunch<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(distributionname: Param0, command: Param1, usecurrentworkingdirectory: Param2, stdin: Param3, stdout: Param4, stderr: Param5) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslLaunch(distributionname: super::super::Foundation::PWSTR, command: super::super::Foundation::PWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, stdin: super::super::Foundation::HANDLE, stdout: super::super::Foundation::HANDLE, stderr: super::super::Foundation::HANDLE, process: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WslLaunch(distributionname.into_param().abi(), command.into_param().abi(), usecurrentworkingdirectory.into_param().abi(), stdin.into_param().abi(), stdout.into_param().abi(), stderr.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslLaunchInteractive<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(distributionname: Param0, command: Param1, usecurrentworkingdirectory: Param2) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslLaunchInteractive(distributionname: super::super::Foundation::PWSTR, command: super::super::Foundation::PWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, exitcode: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        WslLaunchInteractive(distributionname.into_param().abi(), command.into_param().abi(), usecurrentworkingdirectory.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslRegisterDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(distributionname: Param0, targzfilename: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslRegisterDistribution(distributionname: super::super::Foundation::PWSTR, targzfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        WslRegisterDistribution(distributionname.into_param().abi(), targzfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WslUnregisterDistribution<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(distributionname: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WslUnregisterDistribution(distributionname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        WslUnregisterDistribution(distributionname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
