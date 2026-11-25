use super::*;
use immutable_vector::VectorOwner;
use storage::*;
use windows_core::*;

struct StockIterable<S: Storage> {
    values: Vec<S::Stored>,
}

// SAFETY: immutable values follow the factory's threading contract described in storage.
unsafe impl<S: Storage> Send for StockIterable<S> {}
unsafe impl<S: Storage> Sync for StockIterable<S> {}

implement_decl! {
    impl<S> StockIterable as StockIterable_Impl: [IIterable<S::Value>]
    where S: Storage
}

impl<S: Storage> VectorOwner for StockIterable<S> {
    type Storage = S;

    fn values(&self) -> &[S::Stored] {
        &self.values
    }
}

impl<S: Storage> IIterable_Impl<S::Value> for StockIterable_Impl<S> {
    fn First(&self) -> Result<IIterator<S::Value>> {
        Ok(immutable_vector::iterator(self.to_object()))
    }
}

impl<T: RuntimeType> From<Vec<T::Default>> for IIterable<T>
where
    T::Default: Send + Sync,
{
    /// Creates an iterable with directly stored, thread-safe values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IIterable;
    /// use windows_core::IInspectable;
    /// let _ = IIterable::<IInspectable>::from(vec![None]);
    /// ```
    fn from(values: Vec<T::Default>) -> Self {
        // The retained values have a static cross-thread storage contract.
        unsafe { Self::from_unchecked(values) }
    }
}

impl<T: RuntimeType> IIterable<T> {
    /// Creates an iterable with directly stored values.
    ///
    /// # Safety
    ///
    /// All retained values must support cloning, comparison, access, and release on every thread
    /// and in every apartment where this collection or its derived objects are used. The caller
    /// must enforce this for their complete lifetime; `IAgileObject` alone does not prove it.
    pub unsafe fn from_unchecked(values: Vec<T::Default>) -> Self {
        ComObject::new(StockIterable::<Direct<T>> { values }).into_interface()
    }
}

impl<T: Interface + RuntimeType<Default = Option<T>>> IIterable<T> {
    /// Creates an iterable retaining interface values through runtime marshaling.
    ///
    /// Nulls are preserved. Creation and access can fail; COM initialization and apartment
    /// dispatch remain the caller's responsibility. Non-null values require Windows.
    pub fn from_marshaled(values: Vec<Option<T>>) -> Result<Self> {
        Ok(ComObject::new(StockIterable::<Marshaled<T>> {
            values: prepare::<Marshaled<T>>(values)?,
        })
        .into_interface())
    }
}
