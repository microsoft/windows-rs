use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_core::*;

impl IVector<IInspectable> {
    /// Creates a read-only vector that retains its values through agile references.
    ///
    /// Values are resolved in the calling apartment on access. COM initialization and apartment
    /// dispatch remain the caller's responsibility; creating or resolving a reference can fail.
    /// Null values are preserved. Views and iterators share the immutable storage.
    ///
    /// All mutation methods return `E_NOTIMPL`.
    pub fn from_read_only(values: impl IntoIterator<Item = Option<IInspectable>>) -> Result<Self> {
        let values = values
            .into_iter()
            .map(|value| value.as_ref().map(AgileReference::new).transpose())
            .collect::<Result<Vec<_>>>()?;
        let _: u32 = values.len().try_into()?;
        Ok(ComObject::new(ReadOnlyVector { values }).into_interface())
    }
}

struct ReadOnlyVector {
    values: Vec<Option<AgileReference<IInspectable>>>,
}

implement_decl! {
    impl ReadOnlyVector as ReadOnlyVector_Impl: [
        IVector<IInspectable>,
        IVectorView<IInspectable>,
        IIterable<IInspectable>,
    ]
}

impl ReadOnlyVector_Impl {
    fn resolve_many(&self, start: usize, count: usize) -> Result<Vec<Option<IInspectable>>> {
        self.values
            .iter()
            .skip(start)
            .take(count)
            .map(|value| value.as_ref().map(AgileReference::resolve).transpose())
            .collect()
    }
}

impl IVectorView_Impl<IInspectable> for ReadOnlyVector_Impl {
    fn GetAt(&self, index: u32) -> Result<IInspectable> {
        self.values
            .get(index as usize)
            .ok_or_else(|| Error::from(E_BOUNDS))?
            .as_ref()
            .ok_or_else(Error::empty)?
            .resolve()
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.values.len() as u32)
    }

    fn IndexOf(&self, value: Ref<IInspectable>, index: &mut u32) -> Result<bool> {
        *index = 0;
        let identity = value
            .as_ref()
            .map(Interface::cast::<IUnknown>)
            .transpose()?;
        for (position, item) in self.values.iter().enumerate() {
            let item = item
                .as_ref()
                .map(|item| item.resolve()?.cast::<IUnknown>())
                .transpose()?;
            if item == identity {
                *index = position as u32;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn GetMany(&self, start: u32, items: &mut [Option<IInspectable>]) -> Result<u32> {
        let values = self.resolve_many(start as usize, items.len())?;
        let count = values.len() as u32;
        for (item, value) in items.iter_mut().zip(values) {
            *item = value;
        }
        Ok(count)
    }
}

impl IVector_Impl<IInspectable> for ReadOnlyVector_Impl {
    fn GetAt(&self, index: u32) -> Result<IInspectable> {
        IVectorView_Impl::GetAt(self, index)
    }

    fn Size(&self) -> Result<u32> {
        IVectorView_Impl::Size(self)
    }

    fn GetView(&self) -> Result<IVectorView<IInspectable>> {
        Ok(self.to_object().into_interface())
    }

    fn IndexOf(&self, value: Ref<IInspectable>, index: &mut u32) -> Result<bool> {
        IVectorView_Impl::IndexOf(self, value, index)
    }

    fn GetMany(&self, start: u32, items: &mut [Option<IInspectable>]) -> Result<u32> {
        IVectorView_Impl::GetMany(self, start, items)
    }

    fn SetAt(&self, _: u32, _: Ref<IInspectable>) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn InsertAt(&self, _: u32, _: Ref<IInspectable>) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn RemoveAt(&self, _: u32) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn Append(&self, _: Ref<IInspectable>) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn RemoveAtEnd(&self) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn Clear(&self) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn ReplaceAll(&self, _: &[Option<IInspectable>]) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

const E_NOTIMPL: HRESULT = HRESULT(0x80004001_u32 as _);

impl IIterable_Impl<IInspectable> for ReadOnlyVector_Impl {
    fn First(&self) -> Result<IIterator<IInspectable>> {
        Ok(ComObject::new(ReadOnlyVectorIterator {
            owner: self.to_object(),
            current: AtomicUsize::new(0),
        })
        .into_interface())
    }
}

struct ReadOnlyVectorIterator {
    owner: ComObject<ReadOnlyVector>,
    current: AtomicUsize,
}

implement_decl! {
    impl ReadOnlyVectorIterator as ReadOnlyVectorIterator_Impl: [IIterator<IInspectable>]
}

impl IIterator_Impl<IInspectable> for ReadOnlyVectorIterator_Impl {
    fn Current(&self) -> Result<IInspectable> {
        IVectorView_Impl::GetAt(&*self.owner, self.current.load(Ordering::Relaxed) as u32)
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(self.current.load(Ordering::Relaxed) < self.owner.values.len())
    }

    fn MoveNext(&self) -> Result<bool> {
        loop {
            let current = self.current.load(Ordering::Relaxed);
            if current == self.owner.values.len() {
                return Ok(false);
            }
            if self
                .current
                .compare_exchange(current, current + 1, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return Ok(current + 1 < self.owner.values.len());
            }
        }
    }

    fn GetMany(&self, items: &mut [Option<IInspectable>]) -> Result<u32> {
        loop {
            let start = self.current.load(Ordering::Relaxed);
            let values = self.owner.resolve_many(start, items.len())?;
            let count = values.len();
            if self
                .current
                .compare_exchange(start, start + count, Ordering::Relaxed, Ordering::Relaxed)
                .is_err()
            {
                continue;
            }
            for (item, value) in items.iter_mut().zip(values) {
                *item = value;
            }
            return Ok(count as u32);
        }
    }
}
