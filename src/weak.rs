use crate::*;

#[repr(C)]
pub(crate) struct IWeakReference {
    __0: usize,
    __1: usize,
    __2: usize,
    resolve: extern "system" fn(&Guid, *mut RawPtr) -> ErrorCode,
}

#[repr(C)]
pub(crate) struct IWeakReferenceSource {
    __0: usize,
    __1: usize,
    __2: usize,
    reference: extern "system" fn(*mut RawPtr) -> ErrorCode,
}
