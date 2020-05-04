use crate::*;

/// A COM interface
///
/// # Safety
/// Implementors of this trait must be transparent wrappers
/// around pointers to the associated VTable for the interface.
/// In other words, implementators must be exactly equivalent to
/// *mut *mut Self::VTable.
///
/// VTables must be COM compliant VTables where the first three function
/// pointers are the IUknown methods.
///
/// Additionally, because ComInterfaces are just pointers to vtables,
/// it must be safe to zero initialize the interface.
pub unsafe trait ComInterface: Sized {
    const IID: Guid;
    type VTable;

    #[inline(always)]
    fn as_raw(&self) -> RawComPtr<Self> {
        unsafe { std::mem::transmute_copy(self) }
    }

    #[inline(always)]
    fn as_iunknown(&self) -> RawComPtr<IUnknown> {
        self.as_raw() as _
    }

    #[inline(always)]
    fn query<Into: ComInterface>(&self) -> Into {
        unsafe { self.query_with_iid(&Into::IID) }
    }

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_raw().is_null()
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
    unsafe fn query_with_iid<Into: ComInterface>(&self, guid: &Guid) -> Into {
        let mut into: Into = std::mem::zeroed();
        self.raw_query(guid, &mut into);
        into
    }

    unsafe fn raw_query<T: ComInterface>(&self, guid: &Guid, ppv: &mut T) {
        let from = self.as_iunknown();
        if !from.is_null() {
            ((*(*(from))).unknown_query_interface)(from, guid, ppv as *mut _ as _);
        }
    }
}

/// A non-reference-counted pointer to a COM interface
pub type RawComPtr<T> = *const *const <T as ComInterface>::VTable;
