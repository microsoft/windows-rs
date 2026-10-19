use crate::{Array, Type};

pub struct ArrayProxy<T: Type<T>> {
    data: *mut *mut T::Default,
    len: *mut u32,
    temp: core::mem::ManuallyDrop<Array<T>>,
}

/// Borrows a WinRT input array, which may be null when `len` is zero.
pub unsafe fn array_slice<'a, T>(data: *const T, len: u32) -> &'a [T] {
    if len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data, len as usize) }
    }
}

/// Borrows a WinRT fill array, which may be null when `len` is zero.
pub unsafe fn array_slice_mut<'a, T>(data: *mut T, len: u32) -> &'a mut [T] {
    if len == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(data, len as usize) }
    }
}

pub unsafe fn array_proxy<T: Type<T>>(data: *mut *mut T::Default, len: *mut u32) -> ArrayProxy<T> {
    ArrayProxy {
        data,
        len,
        temp: core::mem::ManuallyDrop::new(Array::new()),
    }
}

impl<T: Type<T>> Drop for ArrayProxy<T> {
    fn drop(&mut self) {
        unsafe {
            *self.data = self.temp.data;
            *self.len = self.temp.len;
        }
    }
}

impl<T: Type<T>> core::ops::Deref for ArrayProxy<T> {
    type Target = Array<T>;

    fn deref(&self) -> &Array<T> {
        &self.temp
    }
}

impl<T: Type<T>> core::ops::DerefMut for ArrayProxy<T> {
    fn deref_mut(&mut self) -> &mut Array<T> {
        &mut self.temp
    }
}
