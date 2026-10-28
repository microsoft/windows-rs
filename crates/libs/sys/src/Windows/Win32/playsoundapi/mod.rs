#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn PlaySoundA(pszsound : windows_sys::core::PCSTR, hmod : super::HMODULE, fdwsound : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn PlaySoundW(pszsound : windows_sys::core::PCWSTR, hmod : super::HMODULE, fdwsound : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn sndPlaySoundA(pszsound : windows_sys::core::PCSTR, fusound : super::UINT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn sndPlaySoundW(pszsound : windows_sys::core::PCWSTR, fusound : super::UINT) -> windows_sys::core::BOOL);
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
