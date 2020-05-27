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

    /// Turn a `ComInterface` into a `RawComPtr`
    ///
    /// Note: `RawComPtr`s do not perform an IUnknown reference counting.
    /// Therefore it is important to only hold on to the returned `RawComPtr`
    /// for at most the lifetime of `&self`
    #[inline(always)]
    fn as_raw(&self) -> RawComPtr<Self> {
        // Safe because ComInterface's are by definition `RawComPtr`s
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Get the `ComInterface` as a `RawComPtr<IUnknown>`
    ///
    /// Note: `RawComPtr`s do not perform an IUnknown reference counting.
    /// Therefore it is important to only hold on to the returned `RawComPtr`
    /// for at most the lifetime of `&self`
    #[inline(always)]
    fn as_iunknown(&self) -> RawComPtr<IUnknown> {
        self.as_raw().map(|s| s.cast())
    }

    #[inline(always)]
    fn query<Into: ComInterface>(&self) -> Into {
        unsafe {
            // Safe because `ComInterfaces` by definition must be safe to zero intialize
            let mut into: Into = std::mem::zeroed();
            // Safe because the supplied `IID` is the `IID` of the supplied queried type
            self.raw_query(&Into::iid(), &mut into);
            into
        }
    }

    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_raw().is_none()
    }

    unsafe fn raw_query<T: ComInterface>(&self, guid: &Guid, ppv: &mut T) {
        let from = self.as_iunknown();
        if let Some(from) = from {
            ((*(*from.as_ptr()).as_ptr()).unknown_query_interface)(
                Some(from),
                guid,
                ppv as *mut _ as _,
            );
        }
    }
}

/// A non-reference-counted pointer to a COM interface
pub type RawComPtr<T> = Option<std::ptr::NonNull<std::ptr::NonNull<<T as ComInterface>::VTable>>>;
