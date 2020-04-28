use crate::*;

#[repr(C)]
pub(crate) struct IWeakReference {
    __0: usize,
    __1: usize,
    __2: usize,
    pub strong: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
}

#[repr(C)]
pub(crate) struct IWeakReferenceSource {
    __0: usize,
    __1: usize,
    __2: usize,
    pub weak: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}

unsafe impl ComInterface for IWeakReferenceSource {
    type VTable = IWeakReferenceSource;
    const GUID: Guid = Guid::from_values(
        0x0000_0038,
        0x0000,
        0x0000,
        [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}
