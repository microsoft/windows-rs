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
/// This function panics if the heap allocation fails or if the pointer returned from
/// the heap allocation is not properly aligned to `T`.
///
/// # Safety
/// len must not be less than the number of items in the iterator.
pub unsafe fn string_from_iter<I, T>(iter: I, len: usize) -> *const T
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    let str_len = len + 1;
    let ptr = heap_alloc(str_len * std::mem::size_of::<T>()).expect("could not allocate string") as *mut T;

    // TODO this assert is mostly redundant, HeapAlloc has alignment of 8, we currently only require alignments of 1 or 2.
    // There is no meaningful string type with characters that require an alignment above 8.
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0, "heap allocated buffer is not properly aligned");

    let mut encoder = iter.chain(core::iter::once(T::default()));

    for i in 0..str_len {
        core::ptr::write(
            ptr.add(i),
            match encoder.next() {
                Some(encoded) => encoded,
                None => break,
            },
        );
    }

    // TODO ensure `encoder` is a fused iterator
    assert!(encoder.next().is_none(), "encoder returned more characters than expected");

    ptr
}
