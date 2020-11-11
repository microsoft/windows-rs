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
        &(*(*(this as *mut *mut _) as *mut _))
    }

    // TODO: picked cast rather than query because there's a WinRT method named query but not one named cast
    fn cast<T: Interface>(&self) -> Result<T> {
        unsafe {
            let mut result = None;

            (self.assume_vtable::<IUnknown>().0)(
                std::mem::transmute_copy(self),
                &T::IID,
                &mut result as *mut _ as _,
            )
            .and_some(result)
        }
    }
}
