pub const BYTE_ERROR: u32 = 255;
pub const BYTE_MAX: u32 = 255;
pub const DWORD64_MAX: i32 = -1;
pub const DWORDLONG_ERROR: i32 = -1;
pub const DWORDLONG_MAX: i32 = -1;
pub const DWORD_ERROR: u32 = 4294967295;
pub const DWORD_MAX: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const DWORD_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DWORD_PTR_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const DWORD_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DWORD_PTR_MAX: i32 = -1;
pub const INT16_ERROR: i32 = -1;
pub const INT16_MAX: u32 = 32767;
pub const INT16_MIN: i32 = -32768;
pub const INT32_ERROR: i32 = -1;
pub const INT32_MAX: u32 = 2147483647;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_ERROR: i32 = -1;
pub const INT64_MAX: i32 = -1;
pub const INT64_MIN: u32 = 0;
pub const INT8_ERROR: i32 = -1;
pub const INT8_MAX: u32 = 127;
pub const INT8_MIN: i32 = -128;
pub const INTSAFE_E_ARITHMETIC_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x80070216_u32 as _);
pub const INT_ERROR: i32 = -1;
pub const INT_PTR_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const INT_PTR_MAX: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INT_PTR_MAX: i32 = -1;
#[cfg(target_arch = "x86")]
pub const INT_PTR_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INT_PTR_MIN: u32 = 0;
pub const LONG64_ERROR: i32 = -1;
pub const LONG64_MAX: i32 = -1;
pub const LONG64_MIN: u32 = 0;
pub const LONGLONG_ERROR: i32 = -1;
pub const LONGLONG_MAX: i32 = -1;
pub const LONGLONG_MIN: u32 = 0;
pub const LONG_ERROR: i32 = -1;
pub const LONG_PTR_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const LONG_PTR_MAX: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LONG_PTR_MAX: i32 = -1;
#[cfg(target_arch = "x86")]
pub const LONG_PTR_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LONG_PTR_MIN: u32 = 0;
pub const PTRDIFF_T_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const PTRDIFF_T_MAX: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PTRDIFF_T_MAX: i32 = -1;
#[cfg(target_arch = "x86")]
pub const PTRDIFF_T_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PTRDIFF_T_MIN: u32 = 0;
pub const SHORT_ERROR: i32 = -1;
pub const SHORT_MAX: u32 = 32767;
pub const SHORT_MIN: i32 = -32768;
#[cfg(target_arch = "x86")]
pub const SIZE_T_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZE_T_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const SIZE_T_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZE_T_MAX: i32 = -1;
pub const SSIZE_T_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const SSIZE_T_MAX: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SSIZE_T_MAX: i32 = -1;
#[cfg(target_arch = "x86")]
pub const SSIZE_T_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SSIZE_T_MIN: u32 = 0;
pub const UINT16_ERROR: u32 = 65535;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_ERROR: i32 = -1;
pub const UINT32_MAX: i32 = -1;
pub const UINT64_ERROR: i32 = -1;
pub const UINT64_MAX: i32 = -1;
pub const UINT8_ERROR: u32 = 255;
pub const UINT8_MAX: u32 = 255;
pub const UINT_ERROR: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const UINT_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UINT_PTR_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const UINT_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UINT_PTR_MAX: i32 = -1;
pub const ULONG64_ERROR: i32 = -1;
pub const ULONG64_MAX: i32 = -1;
pub const ULONGLONG_ERROR: i32 = -1;
pub const ULONGLONG_MAX: i32 = -1;
pub const ULONG_ERROR: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const ULONG_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ULONG_PTR_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const ULONG_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ULONG_PTR_MAX: i32 = -1;
pub const USHORT_ERROR: u32 = 65535;
pub const USHORT_MAX: u32 = 65535;
pub const WORD_ERROR: u32 = 65535;
pub const WORD_MAX: u32 = 65535;
