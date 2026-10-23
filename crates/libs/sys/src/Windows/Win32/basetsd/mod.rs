#[cfg(target_arch = "x86")]
pub const ADDRESS_TAG_BIT: u32 = 2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ADDRESS_TAG_BIT: u64 = 4398046511104;
pub type DWORD32 = u32;
#[cfg(target_arch = "x86")]
pub type HALF_PTR = i16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type HALF_PTR = i32;
pub type HANDLE64 = super::__windows_bindgen_Pointer64<*mut core::ffi::c_void>;
#[cfg(target_arch = "x86")]
pub type HANDLE_PTR = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type HANDLE_PTR = u64;
#[cfg(target_arch = "x86")]
pub type KAFFINITY = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type KAFFINITY = u64;
pub type LONG32 = i32;
pub const MAXDWORD32: DWORD32 = 4294967295;
#[cfg(feature = "shared")]
pub const MAXDWORD64: super::DWORD64 = 18446744073709551615;
#[cfg(target_arch = "x86")]
pub const MAXHALF_PTR: HALF_PTR = 32767 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXHALF_PTR: HALF_PTR = 2147483647 as _;
#[cfg(feature = "shared")]
pub const MAXINT: super::INT = 0x7FFFFFFF_u32 as _;
#[cfg(feature = "shared")]
pub const MAXINT16: super::INT16 = 32767;
#[cfg(feature = "shared")]
pub const MAXINT32: super::INT32 = 0x7FFFFFFF_u32 as _;
#[cfg(feature = "shared")]
pub const MAXINT64: super::INT64 = 9223372036854775807;
#[cfg(feature = "shared")]
pub const MAXINT8: super::INT8 = 127;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXINT_PTR: super::INT_PTR = 2147483647 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXINT_PTR: super::INT_PTR = 9223372036854775807i64 as _;
pub const MAXLONG32: LONG32 = 0x7FFFFFFF_u32 as _;
#[cfg(feature = "shared")]
pub const MAXLONG64: super::LONG64 = 9223372036854775807;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXLONG_PTR: super::LONG_PTR = 2147483647 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXLONG_PTR: super::LONG_PTR = 9223372036854775807i64 as _;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXSIZE_T: super::SIZE_T = 4294967295u64 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXSIZE_T: super::SIZE_T = 18446744073709551615u64 as _;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXSSIZE_T: super::SSIZE_T = 2147483647 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXSSIZE_T: super::SSIZE_T = 9223372036854775807i64 as _;
#[cfg(target_arch = "x86")]
pub const MAXUHALF_PTR: UHALF_PTR = 65535 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXUHALF_PTR: UHALF_PTR = 4294967295u32 as _;
#[cfg(feature = "shared")]
pub const MAXUINT: super::UINT = 4294967295;
#[cfg(feature = "shared")]
pub const MAXUINT16: super::UINT16 = 65535;
#[cfg(feature = "shared")]
pub const MAXUINT32: super::UINT32 = 4294967295;
#[cfg(feature = "shared")]
pub const MAXUINT64: super::UINT64 = 18446744073709551615;
#[cfg(feature = "shared")]
pub const MAXUINT8: super::UINT8 = 255;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXUINT_PTR: super::UINT_PTR = 4294967295u64 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXUINT_PTR: super::UINT_PTR = 18446744073709551615u64 as _;
pub const MAXULONG32: ULONG32 = 4294967295;
#[cfg(feature = "shared")]
pub const MAXULONG64: super::ULONG64 = 18446744073709551615;
#[cfg(feature = "shared")]
pub const MAXULONGLONG: super::ULONGLONG = 18446744073709551615;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MAXULONG_PTR: super::ULONG_PTR = 4294967295u64 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MAXULONG_PTR: super::ULONG_PTR = 18446744073709551615u64 as _;
#[cfg(target_arch = "x86")]
pub const MINHALF_PTR: i32 = -32768;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MINHALF_PTR: HALF_PTR = -2147483648 as _;
#[cfg(feature = "shared")]
pub const MININT: super::INT = 0x80000000_u32 as _;
#[cfg(feature = "shared")]
pub const MININT16: super::INT16 = -32768;
#[cfg(feature = "shared")]
pub const MININT32: super::INT32 = 0x80000000_u32 as _;
#[cfg(feature = "shared")]
pub const MININT64: super::INT64 = -9223372036854775808;
#[cfg(feature = "shared")]
pub const MININT8: super::INT8 = -128;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MININT_PTR: super::INT_PTR = -2147483648 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MININT_PTR: super::INT_PTR = -9223372036854775808i64 as _;
pub const MINLONG32: LONG32 = 0x80000000_u32 as _;
#[cfg(feature = "shared")]
pub const MINLONG64: super::LONG64 = -9223372036854775808;
#[cfg(feature = "shared")]
pub const MINLONGLONG: super::LONGLONG = -9223372036854775808;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MINLONG_PTR: super::LONG_PTR = -2147483648 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MINLONG_PTR: super::LONG_PTR = -9223372036854775808i64 as _;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const MINSSIZE_T: super::SSIZE_T = -2147483648 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const MINSSIZE_T: super::SSIZE_T = -9223372036854775808i64 as _;
pub type PDWORD32 = *mut u32;
pub type PDWORD64 = *mut u64;
#[cfg(target_arch = "x86")]
pub type PDWORD_PTR = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PDWORD_PTR = *mut u64;
#[cfg(target_arch = "x86")]
pub type PHALF_PTR = *mut i16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PHALF_PTR = *mut i32;
pub type PHANDLE64 = *mut HANDLE64;
pub type PINT16 = *mut i16;
pub type PINT32 = *mut i32;
pub type PINT64 = *mut i64;
pub type PINT8 = *mut i8;
#[cfg(target_arch = "x86")]
pub type PINT_PTR = *mut i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PINT_PTR = *mut i64;
#[cfg(target_arch = "x86")]
pub type PKAFFINITY = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PKAFFINITY = *mut u64;
pub type PLONG32 = *mut i32;
pub type PLONG64 = *mut i64;
#[cfg(target_arch = "x86")]
pub type PLONG_PTR = *mut i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PLONG_PTR = *mut i64;
#[cfg(target_arch = "x86")]
pub type POINTER_64_INT = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type POINTER_64_INT = u64;
#[cfg(target_arch = "x86")]
pub type PSIZE_T = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSIZE_T = *mut u64;
#[cfg(target_arch = "x86")]
pub type PSSIZE_T = *mut i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSSIZE_T = *mut i64;
#[cfg(target_arch = "x86")]
pub type PUHALF_PTR = *mut u16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PUHALF_PTR = *mut u32;
pub type PUINT16 = *mut u16;
pub type PUINT32 = *mut u32;
pub type PUINT64 = *mut u64;
pub type PUINT8 = *mut u8;
#[cfg(target_arch = "x86")]
pub type PUINT_PTR = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PUINT_PTR = *mut u64;
pub type PULONG32 = *mut u32;
pub type PULONG64 = *mut u64;
#[cfg(target_arch = "x86")]
pub type SHANDLE_PTR = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SHANDLE_PTR = i64;
#[cfg(target_arch = "x86")]
pub type UHALF_PTR = u16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type UHALF_PTR = u32;
pub type ULONG32 = u32;
