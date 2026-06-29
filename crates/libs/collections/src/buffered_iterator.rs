extern crate alloc;

use super::IIterator;
use alloc::vec::Vec;

/// Elements per `GetMany` block, sized to keep the buffer near 2 KB regardless of
/// element size: ~128 pointer-sized elements, fewer for large value structs. A
/// traversal makes roughly `count / block` virtual calls instead of three per
/// element.
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
            // Release the previous block's values before refilling. Resetting to
            // a zeroed default drops the held references and leaves a valid
            // buffer for `GetMany` to overwrite.
            for slot in &mut self.buffer[..self.len] {
                *slot = unsafe { core::mem::zeroed() };
            }
            self.index = 0;
            self.len = self.iterator.GetMany(&mut self.buffer).unwrap_or(0) as usize;
            self.len = self.len.min(self.buffer.len());
            if self.len == 0 {
                return None;
            }
        }

        let result = T::from_default(&self.buffer[self.index]).ok();
        self.index += 1;
        result
    }
}
