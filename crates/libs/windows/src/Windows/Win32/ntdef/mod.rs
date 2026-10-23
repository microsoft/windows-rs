#[cfg(feature = "shared")]
pub type CLONG = super::ULONG;
pub type CSHORT = i16;
#[cfg(feature = "shared")]
pub type KIRQL = super::UCHAR;
#[cfg(feature = "shared")]
pub type LOGICAL = super::ULONG;
pub const NtProductLanManNt: _NT_PRODUCT_TYPE = 2;
pub const NtProductServer: _NT_PRODUCT_TYPE = 3;
pub const NtProductWinNt: _NT_PRODUCT_TYPE = 1;
pub type PNT_PRODUCT_TYPE = *mut _NT_PRODUCT_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_ADDRESS = *mut super::LARGE_INTEGER;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PUTF8_STRING = super::PSTRING;
pub type _NT_PRODUCT_TYPE = i32;
