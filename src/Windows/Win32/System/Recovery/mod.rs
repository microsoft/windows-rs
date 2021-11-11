#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryFinished<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(bsuccess: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryFinished(bsuccess: super::super::Foundation::BOOL);
        }
        ::core::mem::transmute(ApplicationRecoveryFinished(bsuccess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplicationRecoveryInProgress() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryInProgress(pbcancelled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        ApplicationRecoveryInProgress(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, precoverycallback: *mut ::core::option::Option<super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK>, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut ::windows::core::RawPtr, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows::core::HRESULT;
        }
        GetApplicationRecoveryCallback(hprocess.into_param().abi(), ::core::mem::transmute(precoverycallback), ::core::mem::transmute(ppvparameter), ::core::mem::transmute(pdwpinginterval), ::core::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationRestartSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pwzcommandline: super::super::Foundation::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: super::super::Foundation::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows::core::HRESULT;
        }
        GetApplicationRestartSettings(hprocess.into_param().abi(), ::core::mem::transmute(pwzcommandline), ::core::mem::transmute(pcchsize), ::core::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGISTER_APPLICATION_RESTART_FLAGS(pub u32);
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(1u32);
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(2u32);
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(4u32);
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(8u32);
impl ::core::convert::From<u32> for REGISTER_APPLICATION_RESTART_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REGISTER_APPLICATION_RESTART_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_System_WindowsProgramming`*"]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: ::core::option::Option<super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK>, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRecoveryCallback(precoveycallback: ::windows::core::RawPtr, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows::core::HRESULT;
        }
        RegisterApplicationRecoveryCallback(::core::mem::transmute(precoveycallback), ::core::mem::transmute(pvparameter), ::core::mem::transmute(dwpinginterval), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterApplicationRestart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzcommandline: Param0, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRestart(pwzcommandline: super::super::Foundation::PWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows::core::HRESULT;
        }
        RegisterApplicationRestart(pwzcommandline.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRecoveryCallback() -> ::windows::core::HRESULT;
        }
        UnregisterApplicationRecoveryCallback().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRestart() -> ::windows::core::HRESULT;
        }
        UnregisterApplicationRestart().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
