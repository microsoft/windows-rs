use crate::*;

pub struct Array<T: RuntimeType> {
    data: *mut T,
    len: u32,
}

impl<T: RuntimeType> Drop for Array<T> {
    fn drop(&mut self) {
        // CoTaskMemFree
    }
}
