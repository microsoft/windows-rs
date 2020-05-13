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
    type VTable;

    // TODO: this should be a const function returning &'static Guid
    fn iid() -> Guid;

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
        unsafe {
            let mut into: Into = std::mem::zeroed();
            self.raw_query(&Into::iid(), &mut into);
            into
        }
    }

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_raw().is_null()
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
