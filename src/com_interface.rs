use crate::*;

/// A ComInterface
///
/// # Safety
/// Implementors of this trait must be transparent wrappers
/// around pointers to the associated VTable for the interface.
/// In other words, implementators must be exactly equivalent to
/// *const *const Self::VTable.
///
/// VTables must be COM compliant VTables where the first three function
/// pointers are the IUknown methods.
pub unsafe trait ComInterface: Sized {
    const GUID: Guid;
    type VTable;

    fn as_vtable(&self) -> *const *const Self::VTable {
        unsafe { std::mem::transmute_copy(self) }
    }

    fn query<Into: ComInterface>(&self) -> Into {
        unsafe { unsafe_query(self, &Into::GUID) }
    }
}
