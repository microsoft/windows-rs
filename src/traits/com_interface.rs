use crate::*;

/// A COM interface.
///
/// # Safety
/// Implementors of this trait must be transparent wrappers
/// around pointers to the associated vtable for the interface.
/// In other words, implementators must be exactly equivalent to
/// `*mut *mut Self::VTable`.
///
/// `VTable` must be a COM compliant vtable where the first three function
/// pointers are the `IUnknown` methods. And because ComInterfaces are just
/// pointers to vtables, it must be safe to zero-initialize the interface.
pub unsafe trait ComInterface: Sized + GetAbi {
    type Vtable;

    const IID: Guid;

    unsafe fn vtable(&self) -> &Self::Vtable {
        self.vtable_of::<Self>()
    }

    unsafe fn vtable_of<T: ComInterface>(&self) -> &T::Vtable {
        let this = self.as_raw_ptr();
        &(*(*(this as *mut *mut <T as ComInterface>::Vtable) as *mut <T as ComInterface>::Vtable))
    }

    unsafe fn as_raw_ptr(&self) -> RawPtr {
        std::mem::transmute_copy(self)
    }

    fn query<T: ComInterface>(&self) -> Result<T> {
        if let Some(result) = self.try_query::<T>() {
            Ok(result)
        } else {
            Err(ErrorCode::E_NOINTERFACE.into())
        }
    }

    fn try_query<T: ComInterface>(&self) -> Option<T> {
        unsafe {
            let this = std::mem::transmute_copy(self);
            let mut result: Option<T> = None;
            (self.vtable_of::<IUnknown>().0)(this, &T::IID, &mut result as *mut _ as _);
            result
        }
    }

    fn is_agile(&self) -> bool {
        self.query::<IAgileObject>().is_ok()
    }

    // /// Get the `ComInterface` as a `RawComPtr<IUnknown>`.
    // ///
    // /// Note: `RawComPtr`s do not perform IUnknown reference counting.
    // /// Therefore it is important to only hold on to the returned `RawComPtr`
    // /// for at most the lifetime of `&self`.
    // #[inline(always)]
    // fn as_iunknown(&self) -> RawComPtr<IUnknown> {
    //     // Safe because `ComInterface`s are by definition `RawComPtr`s.
    //     let raw: RawComPtr<Self> = unsafe { std::mem::transmute_copy(self) };
    //     Some(raw?.as_iunknown())
    // }

    // /// Query for a particular interface.
    // #[inline(always)]
    // fn query<Into: ComInterface>(&self) -> Into {
    //     unsafe {
    //         // Safe because `ComInterfaces` must by definition be safe to zero-initialize.
    //         let mut into: Into = std::mem::zeroed();
    //         // Safe because the supplied `IID` is the `IID` of the supplied queried type.
    //         self.raw_query(&Into::IID, &mut into);
    //         into
    //     }
    // }

    // fn try_query<Into: ComInterface>(&self) -> Result<Into> {
    //     let mut into = std::ptr::null_mut();
    //     let from = self.as_iunknown();

    //     if let Some(ptr) = from {
    //         unsafe { (ptr.vtable().query_interface)(ptr, &Into::IID, &mut into).ok()? };

    //         debug_assert!(
    //             !into.is_null(),
    //             "Null pointer found after successful QueryInterface call"
    //         );

    //         unsafe { Ok(std::mem::transmute_copy(&into)) }
    //     } else {
    //         Err(ErrorCode::E_NOINTERFACE.into())
    //     }
    // }

    // /// Check whether the ComInterface is currently null.
    // #[inline(always)]
    // fn is_null(&self) -> bool {
    //     self.as_iunknown().is_none()
    // }

    // /// Query for a particular interface by iid.
    // ///
    // /// # Safety
    // /// The supplied `iid` must be the correct `Guid` for the ComInterface `T`
    // /// This function should only be used for generic types that
    // /// [currently](https://github.com/microsoft/winrt-rs/issues/136) do
    // /// not know their iids at compile time.
    // unsafe fn raw_query<T: ComInterface>(&self, iid: &Guid, ppv: &mut T) {
    //     if let Some(from) = self.as_iunknown() {
    //         (from.vtable().query_interface)(from, iid, ppv as *mut T as _);
    //     }
    // }
}

unsafe impl<T: ComInterface> GetAbi for Option<T> {
    type Abi = RawPtr;

    unsafe fn get_abi(&self) -> RawPtr {
        if let Some(interface) = self {
            interface.as_raw_ptr()
        } else {
            std::ptr::null_mut()
        }
    }
}

unsafe impl<T: ComInterface> SetAbi for Option<T> {
    type Abi = *mut RawPtr;

    unsafe fn set_abi(&mut self) -> *mut RawPtr {
        if let Some(this) = self {
            (this.vtable_of::<IUnknown>().2)(this.as_raw_ptr());
            *self = std::mem::zeroed();
        }
        self as *mut Self as *mut RawPtr
    }
}
