use crate::*;

pub struct Array<T> {
    data: *mut T,
    len: u32,
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array {
            data: std::ptr::null_mut(),
            len: 0,
        }
    }
}

impl<T: RuntimeType> Array<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        // TODO: drop members, CoTastkMemFree, zero members
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.len as usize) }
    }

    pub unsafe fn set_abi_len(&mut self) -> *mut u32 {
        &mut self.len
    }

    pub unsafe fn set_abi(&mut self) -> *mut *mut T::Abi {
        self.clear();
        &mut self.data as *mut _ as *mut _
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // TODO: CoTaskMemFree
    }
}
