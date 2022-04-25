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

/// Copy an iterator of `T` into a freshly allocated buffer with an additional default `T` at the end.
///
/// Returns a pointer to the beginning of the buffer. This pointer must be freed when done using `heap_free`.
///
/// # Panics
///
/// This function panics if the heap allocation fails, the alignment requirements of 'T' surpass
/// 8 (HeapAlloc's alignment) or if len is less than the number of items in the iterator.
pub fn string_from_iter<I, T>(iter: I, len: usize) -> *const T
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    // alignment of memory returned by HeapAlloc is at least 8
    // Source: https://docs.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc
    // Ensure that T has sufficient alignment requirements
    assert!(std::mem::align_of::<T>() <= 8, "T alignment surpasses HeapAlloc alignment");

    let len = len + 1;
    let ptr = heap_alloc(len * std::mem::size_of::<T>()).expect("could not allocate string") as *mut T;
    let mut encoder = iter.chain(core::iter::once(T::default()));

    for i in 0..len {
        // SAFETY: ptr points to an allocation object of size `len`, indices accessed are always lower than `len`
        unsafe {
            core::ptr::write(
                ptr.add(i),
                match encoder.next() {
                    Some(encoded) => encoded,
                    None => break,
                },
            );
        }
    }

    assert!(encoder.next().is_none(), "encoder returned more characters than expected");

    ptr
}
