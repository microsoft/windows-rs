use super::*;
use windows_core::*;

#[implement(IObservableVector<T>, IVector<T>, IIterable<T>)]
struct StockObservableVector<T>
where
    T: RuntimeType + 'static,
    T::Default: Clone + PartialEq,
{
    values: std::sync::RwLock<Vec<T::Default>>,
    handlers: Event<VectorChangedEventHandler<T>>,
}

impl<T> IObservableVector_Impl<T> for StockObservableVector_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn VectorChanged(&self, vhnd: Ref<VectorChangedEventHandler<T>>) -> Result<i64> {
        self.handlers.add(vhnd.ok()?)
    }

    fn RemoveVectorChanged(&self, token: i64) -> Result<()> {
        self.handlers.remove(token);
        Ok(())
    }
}

impl<T> IIterable_Impl<T> for StockObservableVector_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        Ok(ComObject::new(StockObservableVectorIterator {
            owner: self.to_object(),
            current: 0.into(),
        })
        .into_interface())
    }
}

impl<T> IVector_Impl<T> for StockObservableVector_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        let values = self.values.read().unwrap();
        let item = values
            .get(index as usize)
            .ok_or_else(|| Error::from(E_BOUNDS))?;
        T::from_default(item)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.values.read().unwrap().len().try_into()?)
    }

    fn GetView(&self) -> Result<IVectorView<T>> {
        let snapshot = self.values.read().unwrap().clone();
        Ok(IVectorView::<T>::from(snapshot))
    }

    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        let values = self.values.read().unwrap();
        match values.iter().position(|element| element == &*value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => {
                *result = 0;
                Ok(false)
            }
        }
    }

    fn SetAt(&self, index: u32, value: Ref<T>) -> Result<()> {
        {
            let mut values = self.values.write().unwrap();
            let item = values
                .get_mut(index as usize)
                .ok_or_else(|| Error::from(E_BOUNDS))?;
            *item = (*value).clone();
        }
        self.fire_changed(CollectionChange::ItemChanged, index);
        Ok(())
    }

    fn InsertAt(&self, index: u32, value: Ref<T>) -> Result<()> {
        {
            let mut values = self.values.write().unwrap();
            let index = index as usize;
            if index > values.len() {
                return Err(Error::from(E_BOUNDS));
            }
            values.insert(index, (*value).clone());
        }
        self.fire_changed(CollectionChange::ItemInserted, index);
        Ok(())
    }

    fn RemoveAt(&self, index: u32) -> Result<()> {
        {
            let mut values = self.values.write().unwrap();
            if index as usize >= values.len() {
                return Err(Error::from(E_BOUNDS));
            }
            values.remove(index as usize);
        }
        self.fire_changed(CollectionChange::ItemRemoved, index);
        Ok(())
    }

    fn Append(&self, value: Ref<T>) -> Result<()> {
        let index = {
            let mut values = self.values.write().unwrap();
            values.push((*value).clone());
            (values.len() - 1) as u32
        };
        self.fire_changed(CollectionChange::ItemInserted, index);
        Ok(())
    }

    fn RemoveAtEnd(&self) -> Result<()> {
        let index = {
            let mut values = self.values.write().unwrap();
            if values.is_empty() {
                return Err(Error::from(E_BOUNDS));
            }
            let index = (values.len() - 1) as u32;
            values.pop();
            index
        };
        self.fire_changed(CollectionChange::ItemRemoved, index);
        Ok(())
    }

    fn Clear(&self) -> Result<()> {
        self.values.write().unwrap().clear();
        self.fire_changed(CollectionChange::Reset, 0);
        Ok(())
    }

    fn GetMany(&self, start_index: u32, items: &mut [T::Default]) -> Result<u32> {
        let values = self.values.read().unwrap();
        let current = start_index as usize;

        if current >= values.len() {
            return Ok(0);
        }

        let actual = std::cmp::min(values.len() - current, items.len());
        let (items, _) = items.split_at_mut(actual);
        items.clone_from_slice(&values[current..current + actual]);
        Ok(actual as u32)
    }

    fn ReplaceAll(&self, items: &[T::Default]) -> Result<()> {
        {
            let mut values = self.values.write().unwrap();
            values.clear();
            values.extend_from_slice(items);
        }
        self.fire_changed(CollectionChange::Reset, 0);
        Ok(())
    }
}

impl<T> StockObservableVector_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn fire_changed(&self, change: CollectionChange, index: u32) {
        let observable: IObservableVector<T> = self.to_object().into_interface();
        let args: IVectorChangedEventArgs =
            ComObject::new(StockVectorChangedEventArgs { change, index }).into_interface();
        self.handlers
            .call(|handler: &VectorChangedEventHandler<T>| handler.Invoke(&observable, &args));
    }
}

#[implement(IVectorChangedEventArgs)]
struct StockVectorChangedEventArgs {
    change: CollectionChange,
    index: u32,
}

impl IVectorChangedEventArgs_Impl for StockVectorChangedEventArgs_Impl {
    fn CollectionChange(&self) -> Result<CollectionChange> {
        Ok(self.change)
    }

    fn Index(&self) -> Result<u32> {
        Ok(self.index)
    }
}

#[implement(IIterator<T>)]
struct StockObservableVectorIterator<T>
where
    T: RuntimeType + 'static,
    T::Default: Clone + PartialEq,
{
    owner: ComObject<StockObservableVector<T>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockObservableVectorIterator_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn Current(&self) -> Result<T> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let values = self.owner.values.read().unwrap();

        if let Some(item) = values.get(current) {
            T::from_default(item)
        } else {
            Err(Error::from(E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let values = self.owner.values.read().unwrap();
        Ok(values.len() > current)
    }

    fn MoveNext(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let values = self.owner.values.read().unwrap();
        let len = values.len();
        drop(values);

        if current < len {
            self.current
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(len > current + 1)
    }

    fn GetMany(&self, items: &mut [T::Default]) -> Result<u32> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let values = self.owner.values.read().unwrap();

        if current >= values.len() {
            return Ok(0);
        }

        let actual = std::cmp::min(values.len() - current, items.len());
        let (items, _) = items.split_at_mut(actual);
        items.clone_from_slice(&values[current..current + actual]);

        self.current
            .fetch_add(actual, std::sync::atomic::Ordering::Relaxed);

        Ok(actual as u32)
    }
}

impl<T> From<Vec<T::Default>> for IObservableVector<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn from(values: Vec<T::Default>) -> Self {
        ComObject::new(StockObservableVector {
            values: std::sync::RwLock::new(values),
            handlers: Event::new(),
        })
        .into_interface()
    }
}
