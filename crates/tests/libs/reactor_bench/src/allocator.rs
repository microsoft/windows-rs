use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);
pub(crate) static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
pub(crate) static CURRENT_BYTES: AtomicU64 = AtomicU64::new(0);

struct CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        CURRENT_BYTES.fetch_sub(layout.size() as u64, Ordering::Relaxed);
        unsafe { System.dealloc(pointer, layout) };
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let pointer = unsafe { System.realloc(pointer, layout, size) };
        if !pointer.is_null() && size > layout.size() {
            ALLOCATED_BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
            CURRENT_BYTES.fetch_add((size - layout.size()) as u64, Ordering::Relaxed);
        } else if !pointer.is_null() {
            CURRENT_BYTES.fetch_sub((layout.size() - size) as u64, Ordering::Relaxed);
        }
        pointer
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

pub(crate) fn allocated_bytes() -> u64 {
    ALLOCATED_BYTES.load(Ordering::Relaxed)
}
