use crate::*;

pub unsafe trait ComInterface: Sized {
    const GUID: Guid;
    type VTable;

    fn as_vtable(&self) -> *const *const Self::VTable {
        unsafe { std::mem::transmute_copy(self) }
    }
}
