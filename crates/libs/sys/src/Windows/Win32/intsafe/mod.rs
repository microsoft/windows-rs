pub const BYTE_ERROR: i32 = 255;
pub const BYTE_MAX: i32 = 255;
pub const DWORD64_MAX: u64 = 18446744073709551615;
pub const DWORDLONG_ERROR: u64 = 18446744073709551615;
pub const DWORD_ERROR: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const DWORD_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DWORD_PTR_ERROR: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const DWORD_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DWORD_PTR_MAX: u64 = 18446744073709551615;
pub const INT16_ERROR: i32 = -1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT16_MAX: i16 = 32767;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT16_MIN: i32 = -32768;
pub const INT32_ERROR: i32 = -1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT32_MAX: i32 = 2147483647;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_ERROR: i64 = -1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT64_MAX: i64 = 9223372036854775807;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT64_MIN: i64 = -9223372036854775808;
pub const INT8_ERROR: i32 = -1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT8_MAX: i8 = 127;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const INT8_MIN: i32 = -128;
pub const INTSAFE_E_ARITHMETIC_OVERFLOW: windows_sys::core::HRESULT = 0x80070216_u32 as _;
pub const INT_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const INT_PTR_ERROR: i32 = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INT_PTR_ERROR: i64 = -1;
#[cfg(target_arch = "x86")]
pub const INT_PTR_MAX: i32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INT_PTR_MAX: i64 = 9223372036854775807;
#[cfg(target_arch = "x86")]
pub const INT_PTR_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INT_PTR_MIN: i64 = -9223372036854775808;
pub const LONG64_ERROR: i64 = -1;
pub const LONG64_MAX: i64 = 9223372036854775807;
pub const LONG64_MIN: i64 = -9223372036854775808;
pub const LONGLONG_ERROR: i64 = -1;
pub const LONGLONG_MAX: i64 = 9223372036854775807;
pub const LONGLONG_MIN: i64 = -9223372036854775808;
pub const LONG_ERROR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const LONG_PTR_ERROR: i32 = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LONG_PTR_ERROR: i64 = -1;
#[cfg(target_arch = "x86")]
pub const LONG_PTR_MAX: i32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LONG_PTR_MAX: i64 = 9223372036854775807;
#[cfg(target_arch = "x86")]
pub const LONG_PTR_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LONG_PTR_MIN: i64 = -9223372036854775808;
#[cfg(target_arch = "x86")]
pub const PTRDIFF_T_ERROR: i32 = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PTRDIFF_T_ERROR: i64 = -1;
#[cfg(target_arch = "x86")]
pub const PTRDIFF_T_MAX: i32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PTRDIFF_T_MAX: i64 = 9223372036854775807;
#[cfg(target_arch = "x86")]
pub const PTRDIFF_T_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PTRDIFF_T_MIN: i64 = -9223372036854775808;
pub const SHORT_ERROR: i32 = -1;
pub const SHORT_MAX: i32 = 32767;
pub const SHORT_MIN: i32 = -32768;
#[cfg(target_arch = "x86")]
pub const SIZE_T_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZE_T_ERROR: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const SIZE_T_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZE_T_MAX: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const SSIZE_T_ERROR: i32 = -1;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SSIZE_T_ERROR: i64 = -1;
#[cfg(target_arch = "x86")]
pub const SSIZE_T_MAX: i32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SSIZE_T_MAX: i64 = 9223372036854775807;
#[cfg(target_arch = "x86")]
pub const SSIZE_T_MIN: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SSIZE_T_MIN: i64 = -9223372036854775808;
pub const UINT16_ERROR: u16 = 65535;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const UINT16_MAX: u16 = 65535;
pub const UINT32_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_ERROR: u64 = 18446744073709551615;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const UINT64_MAX: u64 = 18446744073709551615;
pub const UINT8_ERROR: u8 = 255;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const UINT8_MAX: u8 = 255;
pub const UINT_ERROR: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const UINT_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UINT_PTR_ERROR: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const UINT_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UINT_PTR_MAX: u64 = 18446744073709551615;
pub const ULONG64_ERROR: u64 = 18446744073709551615;
pub const ULONG64_MAX: u64 = 18446744073709551615;
pub const ULONGLONG_ERROR: u64 = 18446744073709551615;
pub const ULONGLONG_MAX: u64 = 18446744073709551615;
pub const ULONG_ERROR: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const ULONG_PTR_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ULONG_PTR_ERROR: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const ULONG_PTR_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ULONG_PTR_MAX: u64 = 18446744073709551615;
pub const USHORT_ERROR: i32 = 65535;
pub const USHORT_MAX: i32 = 65535;
pub const WORD_ERROR: i32 = 65535;
pub const WORD_MAX: i32 = 65535;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const _ARM64_MULT_INTRINS_SUPPORTED: i32 = 0;
#[cfg(target_arch = "aarch64")]
pub const _ARM64_MULT_INTRINS_SUPPORTED: i32 = 1;
#[cfg(target_arch = "x86")]
pub const _SIZE_T_ERROR: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const _SIZE_T_ERROR: u64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const _SIZE_T_MAX: u32 = 4294967295;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const _SIZE_T_MAX: u64 = 18446744073709551615;
