use super::*;
use bindings::*;

// TODO: why not Option<RawPtr>
pub fn heap_alloc(bytes: usize) -> Result<RawPtr> {
    let ptr = unsafe { HeapAlloc(GetProcessHeap(), HEAP_NONE, bytes) };

    if ptr.is_null() {
        Err(E_OUTOFMEMORY.into())
    } else {
        Ok(ptr)
    }
}

/// # Safety
pub unsafe fn heap_free(ptr: RawPtr) {
    HeapFree(GetProcessHeap(), HEAP_NONE, ptr);
}
