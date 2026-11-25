use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use storage::*;
use windows_core::*;

struct StockObservableVector<S: Compare> {
    values: VectorStorage<S>,
    handlers: Event<VectorChangedEventHandler<S::Value>>,
}

implement_decl! {
    impl<S> StockObservableVector as StockObservableVector_Impl: [
        IObservableVector<S::Value>,
        IVector<S::Value>,
        IIterable<S::Value>,
    ]
    where S: Compare
}

impl<S: Compare> IObservableVector_Impl<S::Value> for StockObservableVector_Impl<S> {
    fn VectorChanged(&self, handler: Ref<VectorChangedEventHandler<S::Value>>) -> Result<i64> {
        self.handlers.add(handler.ok()?)
    }

    fn RemoveVectorChanged(&self, token: i64) -> Result<()> {
        self.handlers.remove(token);
        Ok(())
    }
}

impl<S: Compare> IIterable_Impl<S::Value> for StockObservableVector_Impl<S> {
    fn First(&self) -> Result<IIterator<S::Value>> {
        Ok(ComObject::new(StockObservableVectorIterator {
            owner: self.to_object(),
            current: AtomicUsize::new(0),
        })
        .into_interface())
    }
}

impl<S: Compare> IVector_Impl<S::Value> for StockObservableVector_Impl<S> {
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
            .set(index as usize, ref_as_default::<S::Value>(&value))?;
        self.fire_changed(CollectionChange::ItemChanged, index);
        Ok(())
    }

    fn InsertAt(&self, index: u32, value: Ref<S::Value>) -> Result<()> {
        self.values
            .insert(index as usize, ref_as_default::<S::Value>(&value))?;
        self.fire_changed(CollectionChange::ItemInserted, index);
        Ok(())
    }

    fn RemoveAt(&self, index: u32) -> Result<()> {
        self.values.remove(index as usize)?;
        self.fire_changed(CollectionChange::ItemRemoved, index);
        Ok(())
    }

    fn Append(&self, value: Ref<S::Value>) -> Result<()> {
        let index = self.values.append(ref_as_default::<S::Value>(&value))?;
        self.fire_changed(CollectionChange::ItemInserted, index);
        Ok(())
    }

    fn RemoveAtEnd(&self) -> Result<()> {
        let index = self.values.remove_last()?;
        self.fire_changed(CollectionChange::ItemRemoved, index);
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        self.values.clear();
        self.fire_changed(CollectionChange::Reset, 0);
        Ok(())
    }

    fn GetMany(&self, start: u32, items: &mut [<S::Value as Type>::Default]) -> Result<u32> {
        self.values.many(start as usize, items)
    }

    fn ReplaceAll(&self, items: &[<S::Value as Type>::Default]) -> Result<()> {
        self.values.replace(items)?;
        self.fire_changed(CollectionChange::Reset, 0);
        Ok(())
    }
}

impl<S: Compare> StockObservableVector_Impl<S> {
    fn fire_changed(&self, change: CollectionChange, index: u32) {
        let observable: IObservableVector<S::Value> = self.to_object().into_interface();
        let args: IVectorChangedEventArgs =
            ComObject::new(StockVectorChangedEventArgs { change, index }).into_interface();
        self.handlers
            .call(|handler: &VectorChangedEventHandler<S::Value>| {
                handler.Invoke(&observable, &args)
            });
    }
}

struct StockVectorChangedEventArgs {
    change: CollectionChange,
    index: u32,
}

implement_decl! {
    impl StockVectorChangedEventArgs as StockVectorChangedEventArgs_Impl: [IVectorChangedEventArgs]
}

impl IVectorChangedEventArgs_Impl for StockVectorChangedEventArgs_Impl {
    fn CollectionChange(&self) -> Result<CollectionChange> {
        Ok(self.change)
    }

    fn Index(&self) -> Result<u32> {
        Ok(self.index)
    }
}

struct StockObservableVectorIterator<S: Compare> {
    owner: ComObject<StockObservableVector<S>>,
    current: AtomicUsize,
}

implement_decl! {
    impl<S> StockObservableVectorIterator as StockObservableVectorIterator_Impl: [IIterator<S::Value>]
    where S: Compare
}

impl<S: Compare> IIterator_Impl<S::Value> for StockObservableVectorIterator_Impl<S> {
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

impl<T: RuntimeType> From<Vec<T::Default>> for IObservableVector<T>
where
    T::Default: PartialEq + Send + Sync,
{
    /// Creates an observable vector with directly stored, thread-safe values.
    ///
    /// Erased interfaces need runtime marshaling or an explicit unsafe storage contract.
    ///
    /// ```compile_fail
    /// use windows_collections::IObservableVector;
    /// use windows_core::IInspectable;
    /// let _ = IObservableVector::<IInspectable>::from(vec![None]);
    /// ```
    fn from(values: Vec<T::Default>) -> Self {
        unsafe { Self::from_unchecked(values) }
    }
}

impl<T: RuntimeType> IObservableVector<T>
where
    T::Default: PartialEq,
{
    /// Creates an observable vector with directly stored values.
    ///
    /// # Safety
    ///
    /// All retained values must support cloning, comparison, access, and release on every thread
    /// and in every apartment where this collection or its derived objects are used. This includes
    /// all future insertions and replacements through any native or Rust writer. The caller must
    /// enforce this for their complete lifetime; `IAgileObject` alone does not prove it.
    pub unsafe fn from_unchecked(values: Vec<T::Default>) -> Self {
        ComObject::new(StockObservableVector::<Direct<T>> {
            values: VectorStorage::new(values),
            handlers: Event::new(),
        })
        .into_interface()
    }
}

impl<T: Interface + RuntimeType<Default = Option<T>>> IObservableVector<T> {
    /// Creates an observable vector retaining interface values through runtime marshaling.
    ///
    /// Nulls are preserved. Creation and access can fail; COM initialization and apartment
    /// dispatch remain the caller's responsibility. Non-null values require Windows.
    /// Failed marshaling leaves the collection unchanged and emits no notification.
    pub fn from_marshaled(values: Vec<Option<T>>) -> Result<Self> {
        Ok(ComObject::new(StockObservableVector::<Marshaled<T>> {
            values: VectorStorage::new(prepare::<Marshaled<T>>(values)?),
            handlers: Event::new(),
        })
        .into_interface())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector_tests::*;

    fn check(mutation: Mutation) {
        let vector = ComObject::new(StockObservableVector::<Direct<IInspectable>> {
            values: VectorStorage::new(Vec::new()),
            handlers: Event::new(),
        });
        check_release(
            vector.clone(),
            move || vector.values.unlocked(),
            mutation,
            true,
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
