#[cfg(all(feature = "propidl", feature = "shared"))]
#[inline]
pub unsafe fn StgPropertyLengthAsVariant(pprop: *const super::SERIALIZEDPROPERTYVALUE, cbprop: super::ULONG, codepage: super::USHORT, breserved: Option<super::BYTE>) -> super::ULONG {
    windows_core::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbprop : super::ULONG, codepage : super::USHORT, breserved : super::BYTE) -> super::ULONG);
    unsafe { StgPropertyLengthAsVariant(pprop, cbprop, codepage, breserved.unwrap_or(core::mem::zeroed()) as _) }
}
pub type NTPROP = *mut core::ffi::c_void;
