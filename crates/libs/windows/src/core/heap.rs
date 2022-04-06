use super::*;
use bindings::*;

// TODO: why not Option<RawPtr>
pub fn heap_alloc(bytes: usize) -> Result<RawPtr> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap()?, HEAP_NONE, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        Ok(ptr)
    }
}

/// # Safety
pub unsafe fn heap_free(ptr: RawPtr) {
    if let Ok(heap) = GetProcessHeap() {
        HeapFree(heap, HEAP_NONE, ptr);
    }
}

pub fn heap_string<T: Copy + Default + Sized>(value: &[T]) -> *const T {
    let buffer = heap_alloc((value.len() + 1) * std::mem::size_of::<T>()).expect("Could not allocate string") as *mut T;
    let slice = unsafe { std::slice::from_raw_parts_mut(buffer, value.len() + 1) };
    let (string, terminator) = slice.split_at_mut(value.len());
    string.copy_from_slice(value);
    terminator[0] = T::default();
    buffer
}
