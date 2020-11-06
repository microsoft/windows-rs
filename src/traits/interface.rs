use crate::*;

pub unsafe trait Interface: Sized + Abi {
    type Vtable;
    const IID: Guid;

    unsafe fn vtable(&self) -> &Self::Vtable {
        self.vtable_of::<Self>()
    }

    unsafe fn vtable_of<T: Interface>(&self) -> &T::Vtable {
        let this: RawPtr = std::mem::transmute_copy(self);
        &(*(*(this as *mut *mut <T as Interface>::Vtable) as *mut <T as Interface>::Vtable))
    }

    fn query<T: Interface>(&self) -> Result<T> {
        unsafe {
            let mut result: Option<T> = None;

            (self.vtable_of::<IUnknown>().0)(
                std::mem::transmute_copy(self),
                &T::IID,
                &mut result as *mut _ as _,
            );

            match result {
                Some(result) => Ok(result),
                None => Err(Error::just_code(ErrorCode::E_NOINTERFACE)),
            }
        }
    }

    // TODO: Takes its name from assume_init and used internally (hide from docs)
    unsafe fn assume_query<T: Interface>(&self) -> T {
        let mut result = std::mem::zeroed();
        (self.vtable_of::<IUnknown>().0)(
            std::mem::transmute_copy(self),
            &T::IID,
            &mut result as *mut _ as _,
        );
        result
    }

    fn is_agile(&self) -> bool {
        self.query::<IAgileObject>().is_ok()
    }
}
