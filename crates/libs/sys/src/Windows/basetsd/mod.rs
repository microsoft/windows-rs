#[cfg(target_arch = "x86")]
pub const ADDRESS_TAG_BIT: u32 = 2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ADDRESS_TAG_BIT: u32 = 0;
#[cfg(target_arch = "x86")]
pub type HALF_PTR = i16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type HALF_PTR = i32;
pub type HANDLE64 = *mut core::ffi::c_void;
#[cfg(target_arch = "x86")]
pub type HANDLE_PTR = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type HANDLE_PTR = u64;
pub type KAFFINITY = usize;
pub const MAXDWORD32: i32 = -1;
pub const MAXDWORD64: i32 = -1;
#[cfg(target_arch = "x86")]
pub const MAXHALF_PTR: u32 = 32767;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXHALF_PTR: u32 = 2147483647;
pub const MAXINT: u32 = 2147483647;
pub const MAXINT16: u32 = 32767;
pub const MAXINT32: u32 = 2147483647;
pub const MAXINT64: i32 = -1;
pub const MAXINT8: u32 = 127;
#[cfg(target_arch = "x86")]
pub const MAXINT_PTR: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXINT_PTR: i32 = -1;
pub const MAXLONG32: u32 = 2147483647;
pub const MAXLONG64: i32 = -1;
#[cfg(target_arch = "x86")]
pub const MAXLONG_PTR: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXLONG_PTR: i32 = -1;
pub const MAXSIZE_T: i32 = -1;
#[cfg(target_arch = "x86")]
pub const MAXSSIZE_T: u32 = 2147483647;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXSSIZE_T: i32 = -1;
#[cfg(target_arch = "x86")]
pub const MAXUHALF_PTR: u32 = 65535;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXUHALF_PTR: i32 = -1;
pub const MAXUINT: i32 = -1;
pub const MAXUINT16: u32 = 65535;
pub const MAXUINT32: i32 = -1;
pub const MAXUINT64: i32 = -1;
pub const MAXUINT8: u32 = 255;
pub const MAXUINT_PTR: i32 = -1;
pub const MAXULONG32: i32 = -1;
pub const MAXULONG64: i32 = -1;
pub const MAXULONGLONG: i32 = -1;
pub const MAXULONG_PTR: i32 = -1;
#[cfg(target_arch = "x86")]
pub const MINHALF_PTR: i32 = -32768;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MINHALF_PTR: i32 = -2147483648;
pub const MININT: i32 = -2147483648;
pub const MININT16: i32 = -32768;
pub const MININT32: i32 = -2147483648;
pub const MININT64: u32 = 0;
pub const MININT8: i32 = -128;
#[cfg(target_arch = "x86")]
pub const MININT_PTR: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MININT_PTR: u32 = 0;
pub const MINLONG32: i32 = -2147483648;
pub const MINLONG64: u32 = 0;
pub const MINLONGLONG: u32 = 0;
#[cfg(target_arch = "x86")]
pub const MINLONG_PTR: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MINLONG_PTR: u32 = 0;
#[cfg(target_arch = "x86")]
pub const MINSSIZE_T: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MINSSIZE_T: u32 = 0;
pub type PDWORD32 = *mut u32;
pub type PDWORD64 = *mut u64;
pub type PDWORD_PTR = *mut usize;
#[cfg(target_arch = "x86")]
pub type PHALF_PTR = *mut i16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PHALF_PTR = *mut i32;
pub type PHANDLE64 = *mut *mut core::ffi::c_void;
pub type PINT16 = *mut i16;
pub type PINT32 = *mut i32;
pub type PINT64 = *mut i64;
pub type PINT8 = *mut i8;
#[cfg(target_arch = "x86")]
pub type PINT_PTR = *mut i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PINT_PTR = *mut i64;
pub type PKAFFINITY = *mut KAFFINITY;
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
pub type PSIZE_T = *mut usize;
pub type PSSIZE_T = *mut isize;
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
pub type PULONG_PTR = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PULONG_PTR = *mut u64;
#[cfg(target_arch = "x86")]
pub type SHANDLE_PTR = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SHANDLE_PTR = i64;
#[cfg(target_arch = "x86")]
pub type UHALF_PTR = u16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type UHALF_PTR = u32;
