#[cfg(target_arch = "x86")]
pub const ADDRESS_TAG_BIT: u32 = 2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ADDRESS_TAG_BIT: u32 = 0;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HALF_PTR(pub i16);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HALF_PTR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE64(pub *mut core::ffi::c_void);
impl HANDLE64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HANDLE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE_PTR(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE_PTR(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KAFFINITY(pub usize);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWORD32(pub *mut u32);
impl PDWORD32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDWORD32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWORD64(pub *mut u64);
impl PDWORD64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDWORD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWORD_PTR(pub *mut usize);
impl PDWORD_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDWORD_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHALF_PTR(pub *mut i16);
#[cfg(target_arch = "x86")]
impl PHALF_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PHALF_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHALF_PTR(pub *mut i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PHALF_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PHALF_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHANDLE64(pub *mut *mut core::ffi::c_void);
impl PHANDLE64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHANDLE64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT16(pub *mut i16);
impl PINT16 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINT16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT32(pub *mut i32);
impl PINT32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT64(pub *mut i64);
impl PINT64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINT64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT8(pub *mut i8);
impl PINT8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINT8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT_PTR(pub *mut i32);
#[cfg(target_arch = "x86")]
impl PINT_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PINT_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINT_PTR(pub *mut i64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PINT_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PINT_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKAFFINITY(pub *mut KAFFINITY);
impl PKAFFINITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKAFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONG32(pub *mut i32);
impl PLONG32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLONG32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONG64(pub *mut i64);
impl PLONG64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLONG64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONG_PTR(pub *mut i32);
#[cfg(target_arch = "x86")]
impl PLONG_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PLONG_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONG_PTR(pub *mut i64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PLONG_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PLONG_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POINTER_64_INT(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POINTER_64_INT(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSIZE_T(pub *mut usize);
impl PSIZE_T {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSIZE_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSSIZE_T(pub *mut isize);
impl PSSIZE_T {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSSIZE_T {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUHALF_PTR(pub *mut u16);
#[cfg(target_arch = "x86")]
impl PUHALF_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PUHALF_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUHALF_PTR(pub *mut u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PUHALF_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUHALF_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT16(pub *mut u16);
impl PUINT16 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUINT16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT32(pub *mut u32);
impl PUINT32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUINT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT64(pub *mut u64);
impl PUINT64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUINT64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT8(pub *mut u8);
impl PUINT8 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUINT8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT_PTR(pub *mut u32);
#[cfg(target_arch = "x86")]
impl PUINT_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PUINT_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUINT_PTR(pub *mut u64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PUINT_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUINT_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONG32(pub *mut u32);
impl PULONG32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PULONG32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONG64(pub *mut u64);
impl PULONG64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PULONG64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONG_PTR(pub *mut u32);
#[cfg(target_arch = "x86")]
impl PULONG_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PULONG_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONG_PTR(pub *mut u64);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PULONG_PTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PULONG_PTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SHANDLE_PTR(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SHANDLE_PTR(pub i64);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UHALF_PTR(pub u16);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UHALF_PTR(pub u32);
