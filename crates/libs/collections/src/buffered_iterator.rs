use super::IIterator;

/// Number of elements fetched per `GetMany` call. A traversal makes roughly
/// `count / BLOCK` virtual calls instead of three per element.
const BLOCK: usize = 128;

/// An iterator that reads elements from an [`IIterator`] in batches via
/// `GetMany` rather than one at a time.
///
/// The naive [`IIterator`] iteration calls `HasCurrent`, `Current`, and
/// `MoveNext` across the ABI for every element. `BufferedIterator` instead
/// fills a small fixed buffer with a single `GetMany` call and yields from it,
/// cutting the per-element virtual-call cost by orders of magnitude. This is
/// the iterator produced when a collection is iterated directly (for example
/// `for value in &vector`).
pub struct BufferedIterator<T: windows_core::RuntimeType + 'static> {
    iterator: IIterator<T>,
    buffer: [<T as windows_core::Type<T>>::Default; BLOCK],
    index: usize,
    len: usize,
}

impl<T: windows_core::RuntimeType + 'static> BufferedIterator<T> {
    pub fn new(iterator: IIterator<T>) -> Self {
        Self {
            iterator,
            // SAFETY: a zeroed buffer is a valid default for every WinRT `Default`
            // type (a null interface/string or a zero scalar), matching the
            // `core::mem::zeroed` the projection relies on throughout. `GetMany`
            // writes into and `Drop` releases these values, so they must be
            // initialized rather than `MaybeUninit`.
            buffer: unsafe { core::mem::zeroed() },
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

        let result = <T as windows_core::Type<T>>::from_default(&self.buffer[self.index]).ok();
        self.index += 1;
        result
    }
}
