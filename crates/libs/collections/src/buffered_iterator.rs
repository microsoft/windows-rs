extern crate alloc;

use super::IIterator;
use alloc::vec::Vec;

/// Elements per `GetMany` block, sized to keep the buffer in the 1-2 KB range
/// regardless of element size: 128 elements (~1 KB pointer-sized), fewer for
/// large value structs. A traversal makes roughly `count / block` virtual calls
/// instead of three per element.
fn block<T: windows_core::RuntimeType>() -> usize {
    (2048 / size_of::<T::Default>()).clamp(1, 128)
}

/// An iterator that reads elements from an [`IIterator`] in batches via
/// `GetMany` rather than one at a time.
///
/// The naive [`IIterator`] iteration calls `HasCurrent`, `Current`, and
/// `MoveNext` across the ABI for every element. `BufferedIterator` instead
/// fills a small buffer with a single `GetMany` call and yields from it, cutting
/// the per-element virtual-call cost by orders of magnitude. This is the iterator
/// produced when a collection is iterated directly (for example `for value in
/// &vector`).
pub struct BufferedIterator<T: windows_core::RuntimeType + 'static> {
    iterator: IIterator<T>,
    buffer: Vec<T::Default>,
    index: usize,
    len: usize,
}

impl<T: windows_core::RuntimeType + 'static> BufferedIterator<T> {
    pub fn new(iterator: IIterator<T>) -> Self {
        // A zeroed default is valid for every WinRT `Default` type (a null
        // interface/string or a zero scalar). `GetMany` writes into and `Drop`
        // releases these values, so the buffer is initialized, not uninhabited.
        let mut buffer = Vec::new();
        buffer.resize_with(block::<T>(), || unsafe { core::mem::zeroed() });
        Self {
            iterator,
            buffer,
            index: 0,
            len: 0,
        }
    }
}

impl<T: windows_core::RuntimeType + 'static> Iterator for BufferedIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            self.index = 0;
            self.len = self.iterator.GetMany(&mut self.buffer).unwrap_or(0) as usize;
            self.len = self.len.min(self.buffer.len());
            if self.len == 0 {
                return None;
            }
        }

        // Move the element out of the buffer rather than cloning it, leaving a zeroed slot
        // behind. For interface elements (such as the `IKeyValuePair` yielded by map iteration)
        // this hands the buffer's existing reference to the caller, skipping the `AddRef` a
        // clone would take and the matching `Release` when the slot is later overwritten. Slots
        // left unconsumed (early-drop, or beyond a short `GetMany`) are released by the `Vec`.
        let slot = core::mem::replace(&mut self.buffer[self.index], unsafe { core::mem::zeroed() });
        self.index += 1;
        T::from_default_owned(slot).ok()
    }
}
