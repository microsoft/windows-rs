use super::*;
use immutable_vector::VectorOwner;
use storage::*;
use windows_core::*;

impl IVector<IInspectable> {
    /// Creates a read-only vector that retains its values through agile references.
    ///
    /// Values are resolved in the calling apartment on access. COM initialization and apartment
    /// dispatch remain the caller's responsibility; creating or resolving a reference can fail.
    /// Null values are preserved. Views and iterators share the immutable storage.
    /// Non-null values require Windows.
    ///
    /// All mutation methods return `E_NOTIMPL`.
    pub fn from_read_only(values: impl IntoIterator<Item = Option<IInspectable>>) -> Result<Self> {
        let values = prepare::<Marshaled<IInspectable>>(values)?;
        let _: u32 = values.len().try_into()?;
        Ok(ComObject::new(ReadOnlyVector::<Marshaled<IInspectable>> { values }).into_interface())
    }

    /// Creates a read-only vector with directly stored interface values.
    ///
    /// Null values are preserved. Views and iterators share immutable storage, without a data lock
    /// or runtime marshaling. All mutation methods return `E_NOTIMPL`.
    ///
    /// # Safety
    ///
    /// All retained values must support cloning, comparison, access, and release on every thread
    /// and in every apartment where this collection or its derived objects are used. The caller
    /// must enforce this for their complete lifetime; `IAgileObject` alone does not prove it.
    pub unsafe fn from_read_only_unchecked(
        values: impl IntoIterator<Item = Option<IInspectable>>,
    ) -> Result<Self> {
        let values: Vec<_> = values.into_iter().collect();
        let _: u32 = values.len().try_into()?;
        Ok(ComObject::new(ReadOnlyVector::<Direct<IInspectable>> { values }).into_interface())
    }
}

struct ReadOnlyVector<S: Storage<Value = IInspectable>> {
    values: Vec<S::Stored>,
}

// SAFETY: immutable values follow the factory's threading contract described in storage.
unsafe impl<S: Storage<Value = IInspectable>> Send for ReadOnlyVector<S> {}
unsafe impl<S: Storage<Value = IInspectable>> Sync for ReadOnlyVector<S> {}

implement_decl! {
    impl<S> ReadOnlyVector as ReadOnlyVector_Impl: [
        IVector<IInspectable>,
        IVectorView<IInspectable>,
        IIterable<IInspectable>,
    ]
    where S: Storage<Value = IInspectable>
}

impl<S: Storage<Value = IInspectable>> IVectorView_Impl<IInspectable> for ReadOnlyVector_Impl<S> {
    fn GetAt(&self, index: u32) -> Result<IInspectable> {
        self.get_at(index as usize)
    }

    fn Size(&self) -> Result<u32> {
        self.size()
    }

    fn IndexOf(&self, value: Ref<IInspectable>, index: &mut u32) -> Result<bool> {
        *index = 0;
        Ok(index_result(
            identity_index::<IInspectable, S>(&self.values, value.as_ref())?,
            index,
        ))
    }

    fn GetMany(&self, start: u32, items: &mut [Option<IInspectable>]) -> Result<u32> {
        self.get_many(start, items)
    }
}

impl<S: Storage<Value = IInspectable>> IVector_Impl<IInspectable> for ReadOnlyVector_Impl<S> {
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

impl<S: Storage<Value = IInspectable>> IIterable_Impl<IInspectable> for ReadOnlyVector_Impl<S> {
    fn First(&self) -> Result<IIterator<IInspectable>> {
        Ok(immutable_vector::iterator(self.to_object()))
    }
}

impl<S: Storage<Value = IInspectable>> VectorOwner for ReadOnlyVector<S> {
    type Storage = S;

    fn values(&self) -> &[S::Stored] {
        &self.values
    }
}
