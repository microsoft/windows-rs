#[inline]
pub unsafe fn ApplicationRecoveryFinished(bsuccess: bool) {
    windows_targets::link!("kernel32.dll" "system" fn ApplicationRecoveryFinished(bsuccess : super::super::Foundation:: BOOL));
    unsafe { ApplicationRecoveryFinished(bsuccess.into()) }
}
#[inline]
pub unsafe fn ApplicationRecoveryInProgress() -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("kernel32.dll" "system" fn ApplicationRecoveryInProgress(pbcancelled : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ApplicationRecoveryInProgress(&mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: Option<*mut *mut core::ffi::c_void>, pdwpinginterval: Option<*mut u32>, pdwflags: Option<*mut u32>) -> windows_core::HRESULT {
    windows_targets::link!("kernel32.dll" "system" fn GetApplicationRecoveryCallback(hprocess : super::super::Foundation:: HANDLE, precoverycallback : *mut super::WindowsProgramming:: APPLICATION_RECOVERY_CALLBACK, ppvparameter : *mut *mut core::ffi::c_void, pdwpinginterval : *mut u32, pdwflags : *mut u32) -> windows_core::HRESULT);
    unsafe { GetApplicationRecoveryCallback(hprocess, core::mem::transmute(precoverycallback), core::mem::transmute(ppvparameter.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwpinginterval.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwflags.unwrap_or(core::mem::zeroed()))) }
}
#[inline]
pub unsafe fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: Option<windows_core::PWSTR>, pcchsize: *mut u32, pdwflags: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetApplicationRestartSettings(hprocess : super::super::Foundation:: HANDLE, pwzcommandline : windows_core::PWSTR, pcchsize : *mut u32, pdwflags : *mut u32) -> windows_core::HRESULT);
    unsafe { GetApplicationRestartSettings(hprocess, core::mem::transmute(pwzcommandline.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcchsize), core::mem::transmute(pdwflags.unwrap_or(core::mem::zeroed()))).ok() }
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: Option<*const core::ffi::c_void>, dwpinginterval: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn RegisterApplicationRecoveryCallback(precoveycallback : super::WindowsProgramming:: APPLICATION_RECOVERY_CALLBACK, pvparameter : *const core::ffi::c_void, dwpinginterval : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { RegisterApplicationRecoveryCallback(precoveycallback, core::mem::transmute(pvparameter.unwrap_or(core::mem::zeroed())), dwpinginterval, dwflags).ok() }
}
#[inline]
pub unsafe fn RegisterApplicationRestart<P0>(pwzcommandline: P0, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn RegisterApplicationRestart(pwzcommandline : windows_core::PCWSTR, dwflags : REGISTER_APPLICATION_RESTART_FLAGS) -> windows_core::HRESULT);
    unsafe { RegisterApplicationRestart(pwzcommandline.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnregisterApplicationRecoveryCallback() -> windows_core::HRESULT);
    unsafe { UnregisterApplicationRecoveryCallback().ok() }
}
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnregisterApplicationRestart() -> windows_core::HRESULT);
    unsafe { UnregisterApplicationRestart().ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REGISTER_APPLICATION_RESTART_FLAGS(pub u32);
impl REGISTER_APPLICATION_RESTART_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REGISTER_APPLICATION_RESTART_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REGISTER_APPLICATION_RESTART_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(1u32);
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(2u32);
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(4u32);
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = REGISTER_APPLICATION_RESTART_FLAGS(8u32);
