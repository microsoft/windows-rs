#[cfg(all(feature = "propidl", feature = "shared"))]
windows_link::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbprop : super::ULONG, codepage : super::USHORT, breserved : super::BYTE) -> super::ULONG);
pub type NTPROP = *mut core::ffi::c_void;
