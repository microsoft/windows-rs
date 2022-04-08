use super::*;
use bindings::*;

/// Allocate memory of size `bytes` using `HeapAlloc`.
///
/// The memory allocated by this function is uninitialized.
///
/// This function will fail in OOM situations, if the heap is otherwise corrupt,
/// or if getting a handle to the process heap fails.
// TODO: why not return a `Option<RawPtr>`
pub fn heap_alloc(bytes: usize) -> Result<RawPtr> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap()?, HEAP_NONE, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        Ok(ptr)
    }
}

/// Free memory allocated by `HeapAlloc` or `HeapReAlloc`.
///
/// The pointer is allowed to be null. If there is an error getting the process heap,
/// the memory will be leaked.
///
/// # Safety
///
/// `ptr` must be a valid pointer to memory allocated by `HeapAlloc` or `HeapReAlloc`
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
