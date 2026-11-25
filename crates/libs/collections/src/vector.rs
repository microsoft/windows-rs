use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use storage::*;
use windows_core::*;

struct StockVector<S: Compare> {
    values: VectorStorage<S>,
}

implement_decl! {
    impl<S> StockVector as StockVector_Impl: [IVector<S::Value>, IIterable<S::Value>]
    where S: Compare
}

impl<S: Compare> IIterable_Impl<S::Value> for StockVector_Impl<S> {
    fn First(&self) -> Result<IIterator<S::Value>> {
        Ok(ComObject::new(StockVectorIterator {
            owner: self.to_object(),
            current: AtomicUsize::new(0),
        })
        .into_interface())
    }
}

impl<S: Compare> IVector_Impl<S::Value> for StockVector_Impl<S> {
    fn GetAt(&self, index: u32) -> Result<S::Value> {
        self.values.get(index as usize)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.values.len().try_into()?)
    }

    fn GetView(&self) -> Result<IVectorView<S::Value>> {
        Ok(ComObject::new(vector_view::StockVectorView::<S> {
            values: self.values.snapshot(),
        })
        .into_interface())
    }

    fn IndexOf(&self, value: Ref<S::Value>, result: &mut u32) -> Result<bool> {
        self.values
            .index(ref_as_default::<S::Value>(&value), result)
    }

    fn SetAt(&self, index: u32, value: Ref<S::Value>) -> Result<()> {
        self.values
            .set(index as usize, ref_as_default::<S::Value>(&value))
    }

    fn InsertAt(&self, index: u32, value: Ref<S::Value>) -> Result<()> {
        self.values
            .insert(index as usize, ref_as_default::<S::Value>(&value))
    }

    fn RemoveAt(&self, index: u32) -> Result<()> {
        self.values.remove(index as usize)
    }

    fn Append(&self, value: Ref<S::Value>) -> Result<()> {
        self.values.append(ref_as_default::<S::Value>(&value))?;
        Ok(())
    }

    fn RemoveAtEnd(&self) -> Result<()> {
        self.values.remove_last()?;
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        self.values.clear();
        Ok(())
    }

    fn GetMany(&self, start: u32, items: &mut [<S::Value as Type>::Default]) -> Result<u32> {
        self.values.many(start as usize, items)
    }

    fn ReplaceAll(&self, items: &[<S::Value as Type>::Default]) -> Result<()> {
        self.values.replace(items)
    }
}

struct StockVectorIterator<S: Compare> {
    owner: ComObject<StockVector<S>>,
    current: AtomicUsize,
}

implement_decl! {
    impl<S> StockVectorIterator as StockVectorIterator_Impl: [IIterator<S::Value>]
    where S: Compare
}

impl<S: Compare> IIterator_Impl<S::Value> for StockVectorIterator_Impl<S> {
    fn Current(&self) -> Result<S::Value> {
        self.owner.values.get(self.current.load(Ordering::Relaxed))
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(self.current.load(Ordering::Relaxed) < self.owner.values.len())
    }

    fn MoveNext(&self) -> Result<bool> {
        Ok(move_next(&self.current, || self.owner.values.len()))
    }

    fn GetMany(&self, items: &mut [<S::Value as Type>::Default]) -> Result<u32> {
        self.owner.values.iterator_many(&self.current, items)
    }
}

impl<T: RuntimeType> From<Vec<T::Default>> for IVector<T>
where
    T::Default: PartialEq + Send + Sync,
{
    /// Creates a mutable vector with directly stored, thread-safe values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IVector;
    /// use windows_core::IInspectable;
    /// let _ = IVector::<IInspectable>::from(vec![None]);
    /// ```
    ///
    /// `Send` alone is insufficient for retained values.
    ///
    /// ```compile_fail
    /// use windows_collections::IVector;
    /// use windows_core::{RuntimeType, imp};
    /// #[derive(Clone, PartialEq)]
    /// struct Value(std::cell::Cell<u32>);
    /// impl imp::TypeKind for Value { type TypeKind = imp::CloneType; }
    /// impl RuntimeType for Value {
    ///     const SIGNATURE: imp::ConstBuffer =
    ///         imp::ConstBuffer::from_slice(b"struct(Example.Value;u4)");
    /// }
    /// fn assert_send<T: Send>() {}
    /// assert_send::<Value>();
    /// let _ = IVector::<Value>::from(Vec::new());
    /// ```
    ///
    /// `Sync` alone is also insufficient.
    ///
    /// ```compile_fail
    /// use windows_collections::IVector;
    /// use windows_core::{RuntimeType, imp};
    /// #[derive(Clone, PartialEq)]
    /// struct Value(u32, std::marker::PhantomData<std::sync::MutexGuard<'static, ()>>);
    /// impl imp::TypeKind for Value { type TypeKind = imp::CloneType; }
    /// impl RuntimeType for Value {
    ///     const SIGNATURE: imp::ConstBuffer =
    ///         imp::ConstBuffer::from_slice(b"struct(Example.Value;u4)");
    /// }
    /// fn assert_sync<T: Sync>() {}
    /// assert_sync::<Value>();
    /// let _ = IVector::<Value>::from(Vec::new());
    /// ```
    fn from(values: Vec<T::Default>) -> Self {
        unsafe { Self::from_unchecked(values) }
    }
}

impl<T: RuntimeType> IVector<T>
where
    T::Default: PartialEq,
{
    /// Creates a mutable vector with directly stored values.
    ///
    /// # Safety
    ///
    /// All retained values must support cloning, comparison, access, and release on every thread
    /// and in every apartment where this collection or its derived objects are used. This includes
    /// all future insertions and replacements through any native or Rust writer. The caller must
    /// enforce this for their complete lifetime; `IAgileObject` alone does not prove it.
    pub unsafe fn from_unchecked(values: Vec<T::Default>) -> Self {
        ComObject::new(StockVector::<Direct<T>> {
            values: VectorStorage::new(values),
        })
        .into_interface()
    }
}

impl<T: Interface + RuntimeType<Default = Option<T>>> IVector<T> {
    /// Creates a mutable vector retaining interface values through runtime marshaling.
    ///
    /// Nulls are preserved. Creation and access can fail; COM initialization and apartment
    /// dispatch remain the caller's responsibility. Non-null values require Windows.
    /// Every insertion or replacement is marshaled before changing the collection.
    pub fn from_marshaled(values: Vec<Option<T>>) -> Result<Self> {
        Ok(ComObject::new(StockVector::<Marshaled<T>> {
            values: VectorStorage::new(prepare::<Marshaled<T>>(values)?),
        })
        .into_interface())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_tests::*;

    fn check(mutation: Mutation) {
        let vector = ComObject::new(StockVector::<Direct<IInspectable>> {
            values: VectorStorage::new(Vec::new()),
        });
        check_release(
            vector.clone(),
            move || vector.values.unlocked(),
            mutation,
            false,
        );
    }

    #[test]
    fn set_release_reenters() {
        check(Mutation::Set);
    }
    #[test]
    fn remove_release_reenters() {
        check(Mutation::Remove);
    }
    #[test]
    fn remove_last_release_reenters() {
        check(Mutation::RemoveLast);
    }
    #[test]
    fn clear_release_reenters() {
        check(Mutation::Clear);
    }
    #[test]
    fn replace_all_release_reenters() {
        check(Mutation::ReplaceAll);
    }
}
