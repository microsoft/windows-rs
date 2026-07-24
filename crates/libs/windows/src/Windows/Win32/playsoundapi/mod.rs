#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PlaySoundA<P0>(pszsound: P0, hmod: Option<super::HMODULE>, fdwsound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_core::PCSTR, hmod : super::HMODULE, fdwsound : u32) -> windows_core::BOOL);
    unsafe { PlaySoundA(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn PlaySoundW<P0>(pszsound: P0, hmod: Option<super::HMODULE>, fdwsound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_core::PCWSTR, hmod : super::HMODULE, fdwsound : u32) -> windows_core::BOOL);
    unsafe { PlaySoundW(pszsound.param().abi(), hmod.unwrap_or(core::mem::zeroed()) as _, fdwsound) }
}
#[inline]
pub unsafe fn sndPlaySoundA<P0>(pszsound: P0, fusound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_core::PCSTR, fusound : u32) -> windows_core::BOOL);
    unsafe { sndPlaySoundA(pszsound.param().abi(), fusound) }
}
#[inline]
pub unsafe fn sndPlaySoundW<P0>(pszsound: P0, fusound: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_core::PCWSTR, fusound : u32) -> windows_core::BOOL);
    unsafe { sndPlaySoundW(pszsound.param().abi(), fusound) }
}
pub const SND_ALIAS: i32 = 65536;
pub const SND_ALIAS_ID: i32 = 1114112;
pub const SND_ALIAS_START: i32 = 0;
pub const SND_ALIAS_SYSTEMASTERISK: u32 = 10835;
pub const SND_ALIAS_SYSTEMDEFAULT: u32 = 17491;
pub const SND_ALIAS_SYSTEMEXCLAMATION: u32 = 8531;
pub const SND_ALIAS_SYSTEMEXIT: u32 = 17747;
pub const SND_ALIAS_SYSTEMHAND: u32 = 18515;
pub const SND_ALIAS_SYSTEMQUESTION: u32 = 16211;
pub const SND_ALIAS_SYSTEMSTART: u32 = 21331;
pub const SND_ALIAS_SYSTEMWELCOME: u32 = 22355;
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
