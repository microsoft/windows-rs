#[cfg(feature = "propidl")]
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const super::propidl::SERIALIZEDPROPERTYVALUE, cbprop: u32, codepage: u16, breserved: Option<u8>) -> u32 {
    windows_core::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const super::propidl::SERIALIZEDPROPERTYVALUE, cbprop : u32, codepage : u16, breserved : u8) -> u32);
    unsafe { StgPropertyLengthAsVariant(pprop, cbprop, codepage, breserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NTPROP(pub *mut core::ffi::c_void);
impl Default for NTPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
