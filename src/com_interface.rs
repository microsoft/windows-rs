use crate::*;

/// A COM interface
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

    fn as_vtable(&self) -> ComInterfacePtr<Self> {
        unsafe { std::mem::transmute_copy(self) }
    }

    fn as_iunknown(&self) -> ComInterfacePtr<IUnknown> {
        self.as_vtable() as _
    }

    fn query<Into: ComInterface>(&self) -> Into {
        unsafe { self.query_with_guid(&Into::GUID) }
    }

    fn is_null(&self) -> bool {
        self.as_vtable().is_null()
    }

    /// Use QueryInterface to cast a ComInterface into another.
    ///
    /// If the call to QueryInterface fails, the returned ComInterface will be null.
    ///
    /// # Safety
    /// The guid parameter must be a valid guid for the returned ComInterface.
    /// Normally, the ComInterface has an associated GUID that you can use. When this
    /// is the case, prefer using `ComInterface::query`. `query_with_guid` only exists
    /// to support generic interface queries that aren't yet supported by Rust because
    /// it lacks good const generics support to work out the guids in a generic ComInterface
    /// where the GUID depends on the concrete type of the generic paramter.
    ///
    /// Once const generics support arrives, we should be able to remove this function and
    /// rely on ComInterface to calculate the guid for all types.
    unsafe fn query_with_guid<Into: ComInterface>(&self, guid: &Guid) -> Into {
        let mut into: Into = std::mem::zeroed();
        let from = self.as_iunknown();
        if !from.is_null() {
            ((*(*(from))).query)(from, guid, &mut into as *mut _ as _);
        }
        into
    }
}

/// A non-reference-counter pointer to a COM interface
pub type ComInterfacePtr<T> = *const *const <T as ComInterface>::VTable;
