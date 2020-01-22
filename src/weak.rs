use crate::*;

#[repr(C)]
pub(crate) struct IWeakReference {
    __0: usize,
    __1: usize,
    __2: usize,
    pub resolve: extern "system" fn(&Guid, *mut RawPtr) -> ErrorCode,
}

#[repr(C)]
pub(crate) struct IWeakReferenceSource {
    __0: usize,
    __1: usize,
    __2: usize,
    pub reference: extern "system" fn(*mut RawPtr) -> ErrorCode,
}

impl TypeGuid for IWeakReferenceSource {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(0x00000038, 0x0000, 0x0000, &[0xC0,0x00,0x00,0x00,0x00,0x00,0x00,0x46]);
        &GUID
    }
}
