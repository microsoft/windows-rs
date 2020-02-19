use crate::*;

pub struct Array<T: RuntimeType> {
    data: *mut T,
    len: u32,
}

impl<T: RuntimeType> Array<T> {
    pub fn new() -> Array<T> {
        Array {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        // TODO: drop members, CoTastkMemFree, zero members
    }

    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }

    pub unsafe fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    pub unsafe fn set_abi(&mut self) -> *mut *mut T::Abi {
        self.clear();
        std::mem::transmute(&mut self.data)
    }
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        // TODO: CoTaskMemFree
    }
}
