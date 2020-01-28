use crate::*;

pub struct Array<T: RuntimeType> {
    data: *mut T,
    len: u32,
}

impl<T: RuntimeType> Array<T> {
    pub unsafe fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    pub unsafe fn set_abi(&mut self) -> *mut *mut T::Abi {
        std::mem::transmute_copy(&mut self.data)
    }
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        // CoTaskMemFree
    }
}
