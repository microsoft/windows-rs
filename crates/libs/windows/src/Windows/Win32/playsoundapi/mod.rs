#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PlaySoundA<P0>(pszsound: P0, hmod: Option<super::HMODULE>, fdwsound: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_core::PCSTR, hmod : super::HMODULE, fdwsound : super::DWORD) -> windows_core::BOOL);
    unsafe { PlaySoundA(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn PlaySoundW<P0>(pszsound: P0, hmod: Option<super::HMODULE>, fdwsound: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_core::PCWSTR, hmod : super::HMODULE, fdwsound : super::DWORD) -> windows_core::BOOL);
    unsafe { PlaySoundW(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn sndPlaySoundA<P0>(pszsound: P0, fusound: super::UINT) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_core::PCSTR, fusound : super::UINT) -> windows_core::BOOL);
    unsafe { sndPlaySoundA(pszsound.param().abi(), fusound) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn sndPlaySoundW<P0>(pszsound: P0, fusound: super::UINT) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_core::PCWSTR, fusound : super::UINT) -> windows_core::BOOL);
    unsafe { sndPlaySoundW(pszsound.param().abi(), fusound) }
}
pub const SND_ALIAS: i32 = 65536;
pub const SND_ALIAS_ID: i32 = 1114112;
pub const SND_ALIAS_START: i32 = 0;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMASTERISK: super::DWORD = 10835;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMDEFAULT: super::DWORD = 17491;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMEXCLAMATION: super::DWORD = 8531;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMEXIT: super::DWORD = 17747;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMHAND: super::DWORD = 18515;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMQUESTION: super::DWORD = 16211;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMSTART: super::DWORD = 21331;
#[cfg(feature = "shared")]
pub const SND_ALIAS_SYSTEMWELCOME: super::DWORD = 22355;
pub const SND_APPLICATION: i32 = 128;
pub const SND_ASYNC: i32 = 1;
pub const SND_FILENAME: i32 = 131072;
pub const SND_LOOP: i32 = 8;
pub const SND_MEMORY: i32 = 4;
pub const SND_NODEFAULT: i32 = 2;
pub const SND_NOSTOP: i32 = 16;
pub const SND_NOWAIT: i32 = 8192;
pub const SND_PURGE: i32 = 64;
pub const SND_RESOURCE: i32 = 262148;
pub const SND_RING: i32 = 1048576;
pub const SND_SENTRY: i32 = 524288;
pub const SND_SYNC: i32 = 0;
pub const SND_SYSTEM: i32 = 2097152;
