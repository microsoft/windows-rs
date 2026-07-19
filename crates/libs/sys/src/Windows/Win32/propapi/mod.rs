#[cfg(feature = "propidl")]
windows_link::link!("ole32.dll" "system" fn StgPropertyLengthAsVariant(pprop : *const super::SERIALIZEDPROPERTYVALUE, cbprop : u32, codepage : u16, breserved : u8) -> u32);
pub type NTPROP = *mut core::ffi::c_void;
