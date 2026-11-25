use super::*;
use immutable_vector::VectorOwner;
use storage::*;
use windows_core::*;

pub(crate) struct StockVectorView<S: Compare> {
    pub(crate) values: Vec<S::Stored>,
}

// SAFETY: immutable values retain the factory's threading contract, including in snapshots.
unsafe impl<S: Compare> Send for StockVectorView<S> {}
unsafe impl<S: Compare> Sync for StockVectorView<S> {}

implement_decl! {
    impl<S> StockVectorView as pub(crate) StockVectorView_Impl: [
        IVectorView<S::Value>,
        IIterable<S::Value>,
    ]
    where S: Compare
}

impl<S: Compare> IIterable_Impl<S::Value> for StockVectorView_Impl<S> {
    fn First(&self) -> Result<IIterator<S::Value>> {
        Ok(immutable_vector::iterator(self.to_object()))
    }
}

impl<S: Compare> VectorOwner for StockVectorView<S> {
    type Storage = S;

    fn values(&self) -> &[S::Stored] {
        &self.values
    }
}

impl<S: Compare> IVectorView_Impl<S::Value> for StockVectorView_Impl<S> {
    fn GetAt(&self, index: u32) -> Result<S::Value> {
        self.get_at(index as usize)
    }

    fn Size(&self) -> Result<u32> {
        self.size()
    }

    fn IndexOf(&self, value: Ref<S::Value>, result: &mut u32) -> Result<bool> {
        *result = 0;
        Ok(index_result(
            S::index(&self.values, ref_as_default::<S::Value>(&value))?,
            result,
        ))
    }

    fn GetMany(&self, start: u32, items: &mut [<S::Value as Type>::Default]) -> Result<u32> {
        self.get_many(start, items)
    }
}

impl<T: RuntimeType> From<Vec<T::Default>> for IVectorView<T>
where
    T::Default: PartialEq + Send + Sync,
{
    /// Creates a read-only view with directly stored, thread-safe values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IVectorView;
    /// use windows_core::IInspectable;
    /// let _ = IVectorView::<IInspectable>::from(vec![None]);
    /// ```
    fn from(values: Vec<T::Default>) -> Self {
        unsafe { Self::from_unchecked(values) }
    }
}

impl<T: RuntimeType> IVectorView<T>
where
    T::Default: PartialEq,
{
    /// Creates a read-only view with directly stored values.
    ///
    /// # Safety
    ///
    /// All retained values must support cloning, comparison, access, and release on every thread
    /// and in every apartment where this collection or its derived objects are used. The caller
    /// must enforce this for their complete lifetime; `IAgileObject` alone does not prove it.
    pub unsafe fn from_unchecked(values: Vec<T::Default>) -> Self {
        ComObject::new(StockVectorView::<Direct<T>> { values }).into_interface()
    }
}

impl<T: Interface + RuntimeType<Default = Option<T>>> IVectorView<T> {
    /// Creates a read-only view retaining interface values through runtime marshaling.
    ///
    /// Nulls are preserved. Creation and access can fail; COM initialization and apartment
    /// dispatch remain the caller's responsibility. Non-null values require Windows.
    pub fn from_marshaled(values: Vec<Option<T>>) -> Result<Self> {
        Ok(ComObject::new(StockVectorView::<Marshaled<T>> {
            values: prepare::<Marshaled<T>>(values)?,
        })
        .into_interface())
    }
}
