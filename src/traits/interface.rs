use crate::*;

pub unsafe trait Interface: Sized + Abi {
    type Vtable;
    const IID: Guid;

    unsafe fn vtable(&self) -> &Self::Vtable {
        self.assume_vtable::<Self>()
    }

    // TODO: name from assume_init
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        let this: RawPtr = std::mem::transmute_copy(self);
        &(*(*(this as *mut *mut <T as Interface>::Vtable) as *mut <T as Interface>::Vtable))
    }

    // TODO: picked cast rather than query because there's a WinRT method named query but not one named cast
    fn cast<T: Interface>(&self) -> Option<T> {
        unsafe {
            let mut result: Option<T> = None;

            (self.assume_vtable::<IUnknown>().0)(
                std::mem::transmute_copy(self),
                &T::IID,
                &mut result as *mut _ as _,
            );

            result
        }
    }

    // TODO: just use Result<T> and drop the Option<T> variant
    fn cast_ok<T: Interface>(&self) -> Result<T> {
        self.cast()
            .ok_or_else(|| Error::just_code(ErrorCode::E_NOINTERFACE))
    }
}
