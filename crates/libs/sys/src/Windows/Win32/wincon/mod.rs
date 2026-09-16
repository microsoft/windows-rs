#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub const CONSOLE_REAL_INPUT_HANDLE: super::HANDLE = -3 as _;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub const CONSOLE_REAL_OUTPUT_HANDLE: super::HANDLE = -2 as _;
pub const CONSOLE_TEXTMODE_BUFFER: i32 = 1;
