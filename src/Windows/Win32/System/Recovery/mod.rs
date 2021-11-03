#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ApplicationRecoveryFinished<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(bsuccess: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryFinished(bsuccess: super::super::Foundation::BOOL);
        }
        ::std::mem::transmute(ApplicationRecoveryFinished(bsuccess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn ApplicationRecoveryInProgress() -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplicationRecoveryInProgress(pbcancelled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        ApplicationRecoveryInProgress(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, precoverycallback: *mut ::std::option::Option<super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK>, ppvparameter: *mut *mut ::std::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut ::windows::runtime::RawPtr, ppvparameter: *mut *mut ::std::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetApplicationRecoveryCallback(hprocess.into_param().abi(), ::std::mem::transmute(precoverycallback), ::std::mem::transmute(ppvparameter), ::std::mem::transmute(pdwpinginterval), ::std::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetApplicationRestartSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pwzcommandline: super::super::Foundation::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: super::super::Foundation::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetApplicationRestartSettings(hprocess.into_param().abi(), ::std::mem::transmute(pwzcommandline), ::std::mem::transmute(pcchsize), ::std::mem::transmute(pdwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct REGISTER_APPLICATION_RESTART_FLAGS(pub u32);
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(1u32);
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(2u32);
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(4u32);
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(8u32);
impl ::std::convert::From<u32> for REGISTER_APPLICATION_RESTART_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for REGISTER_APPLICATION_RESTART_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_System_WindowsProgramming`*"]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: ::std::option::Option<super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK>, pvparameter: *const ::std::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRecoveryCallback(precoveycallback: ::windows::runtime::RawPtr, pvparameter: *const ::std::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        RegisterApplicationRecoveryCallback(::std::mem::transmute(precoveycallback), ::std::mem::transmute(pvparameter), ::std::mem::transmute(dwpinginterval), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Recovery`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RegisterApplicationRestart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwzcommandline: Param0, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterApplicationRestart(pwzcommandline: super::super::Foundation::PWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows::runtime::HRESULT;
        }
        RegisterApplicationRestart(pwzcommandline.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRecoveryCallback() -> ::windows::runtime::HRESULT;
        }
        UnregisterApplicationRecoveryCallback().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Recovery`*"]
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterApplicationRestart() -> ::windows::runtime::HRESULT;
        }
        UnregisterApplicationRestart().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
