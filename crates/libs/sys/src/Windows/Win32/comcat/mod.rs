#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct CATEGORYINFO {
    pub catid: CATID,
    pub lcid: super::LCID,
    pub szDescription: [super::OLECHAR; 128],
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CATID = windows_sys::core::GUID;
pub const CATID_NULL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPCATEGORYINFO = *mut CATEGORYINFO;
pub type LPCATINFORMATION = *mut core::ffi::c_void;
pub type LPCATREGISTER = *mut core::ffi::c_void;
pub type LPENUMCATEGORYINFO = *mut core::ffi::c_void;
pub type LPENUMGUID = *mut core::ffi::c_void;
