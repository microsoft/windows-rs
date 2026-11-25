use super::*;
use std::sync::{
    RwLock,
    atomic::{AtomicUsize, Ordering},
};
pub(crate) use windows_core::imp::Type;
use windows_core::*;

// Collection factories establish a threading contract for retained values through static bounds,
// marshaling, or the caller's unsafe lifetime guarantee. Direct storage keeps its original layout
// and types, so owners assert that contract rather than requiring Stored: Send + Sync. Snapshots,
// iterators, pairs, and event arguments inherit it; mutable owners also cover future writes.
pub(crate) trait Storage: Sized + 'static {
    type Value: RuntimeType + 'static;
    type Stored: Clone;

    fn prepare(value: <Self::Value as Type>::Default) -> Result<Self::Stored>;

    fn prepare_ref(value: &<Self::Value as Type>::Default) -> Result<Self::Stored> {
        Self::prepare(value.clone())
    }

    fn replace_locked(
        values: &RwLock<Vec<Self::Stored>>,
        items: &[<Self::Value as Type>::Default],
    ) -> Result<()> {
        let prepared = items
            .iter()
            .map(Self::prepare_ref)
            .collect::<Result<Vec<_>>>()?;
        let retired = core::mem::replace(&mut *values.write().unwrap(), prepared);
        drop(retired);
        Ok(())
    }

    fn resolve(value: &Self::Stored) -> Result<<Self::Value as Type>::Default>;

    fn resolve_owned(value: Self::Stored) -> Result<<Self::Value as Type>::Default> {
        Self::resolve(&value)
    }

    fn get(value: &Self::Stored) -> Result<Self::Value> {
        Self::Value::from_default_owned(Self::resolve(value)?)
    }

    fn get_owned(value: Self::Stored) -> Result<Self::Value> {
        Self::Value::from_default_owned(Self::resolve_owned(value)?)
    }

    fn get_many(
        values: &[Self::Stored],
        start: usize,
        items: &mut [<Self::Value as Type>::Default],
    ) -> Result<u32> {
        Ok(write_many::<Self::Value>(
            items,
            resolve_many::<Self>(values, start, items.len())?,
        ))
    }

    fn get_many_locked(
        values: &RwLock<Vec<Self::Stored>>,
        start: usize,
        items: &mut [<Self::Value as Type>::Default],
    ) -> Result<u32> {
        Ok(write_many::<Self::Value>(
            items,
            resolve_many_locked::<Self>(values, start, items.len())?,
        ))
    }

    fn iterator_many(
        values: &[Self::Stored],
        current: &AtomicUsize,
        items: &mut [<Self::Value as Type>::Default],
    ) -> Result<u32> {
        iterator_many::<Self::Value>(current, items, |start, count| {
            resolve_many::<Self>(values, start, count)
        })
    }

    fn iterator_many_locked(
        values: &RwLock<Vec<Self::Stored>>,
        current: &AtomicUsize,
        items: &mut [<Self::Value as Type>::Default],
    ) -> Result<u32> {
        iterator_many::<Self::Value>(current, items, |start, count| {
            resolve_many_locked::<Self>(values, start, count)
        })
    }
}

pub(crate) trait Compare: Storage {
    fn index(
        values: &[Self::Stored],
        value: &<Self::Value as Type>::Default,
    ) -> Result<Option<u32>>;

    fn index_locked(
        values: &RwLock<Vec<Self::Stored>>,
        value: &<Self::Value as Type>::Default,
    ) -> Result<Option<u32>> {
        let snapshot = values.read().unwrap().clone();
        Self::index(&snapshot, value)
    }
}

pub(crate) struct Direct<T>(core::marker::PhantomData<T>);

impl<T: RuntimeType + 'static> Storage for Direct<T> {
    type Value = T;
    type Stored = T::Default;

    fn prepare(value: T::Default) -> Result<Self::Stored> {
        Ok(value)
    }

    fn replace_locked(values: &RwLock<Vec<Self::Stored>>, items: &[T::Default]) -> Result<()> {
        if !core::mem::needs_drop::<Self::Stored>() {
            let mut values = values.write().unwrap();
            values.clear();
            values.extend_from_slice(items);
        } else {
            let prepared = items.to_vec();
            let retired = core::mem::replace(&mut *values.write().unwrap(), prepared);
            drop(retired);
        }
        Ok(())
    }

    fn resolve(value: &Self::Stored) -> Result<T::Default> {
        Ok(value.clone())
    }

    fn resolve_owned(value: Self::Stored) -> Result<T::Default> {
        Ok(value)
    }

    fn get_many(values: &[Self::Stored], start: usize, items: &mut [T::Default]) -> Result<u32> {
        let start = start.min(values.len());
        let count = items.len().min(values.len() - start);
        items[..count].clone_from_slice(&values[start..start + count]);
        Ok(count as u32)
    }

    fn get_many_locked(
        values: &RwLock<Vec<Self::Stored>>,
        start: usize,
        items: &mut [T::Default],
    ) -> Result<u32> {
        Self::get_many(&values.read().unwrap(), start, items)
    }

    fn iterator_many(
        values: &[Self::Stored],
        current: &AtomicUsize,
        items: &mut [T::Default],
    ) -> Result<u32> {
        let Some(range) = reserve(current, values.len(), items.len()) else {
            return Ok(0);
        };
        let count = range.len();
        items[..count].clone_from_slice(&values[range]);
        Ok(count as u32)
    }

    fn iterator_many_locked(
        values: &RwLock<Vec<Self::Stored>>,
        current: &AtomicUsize,
        items: &mut [T::Default],
    ) -> Result<u32> {
        Self::iterator_many(&values.read().unwrap(), current, items)
    }
}

impl<T: RuntimeType + 'static> Compare for Direct<T>
where
    T::Default: PartialEq,
{
    fn index(values: &[Self::Stored], value: &T::Default) -> Result<Option<u32>> {
        Ok(values
            .iter()
            .position(|item| item == value)
            .map(|i| i as u32))
    }

    fn index_locked(values: &RwLock<Vec<Self::Stored>>, value: &T::Default) -> Result<Option<u32>> {
        Self::index(&values.read().unwrap(), value)
    }
}

pub(crate) struct Marshaled<T>(core::marker::PhantomData<T>);

impl<T: Interface + RuntimeType<Default = Option<T>> + 'static> Storage for Marshaled<T> {
    type Value = T;
    type Stored = Option<AgileReference<T>>;

    fn prepare(value: Option<T>) -> Result<Self::Stored> {
        Self::prepare_ref(&value)
    }

    fn prepare_ref(value: &Option<T>) -> Result<Self::Stored> {
        #[cfg(windows)]
        {
            value.as_ref().map(AgileReference::new).transpose()
        }
        #[cfg(not(windows))]
        {
            if value.is_some() {
                Err(HRESULT(0x80004001_u32 as _).into())
            } else {
                Ok(None)
            }
        }
    }

    fn resolve(value: &Self::Stored) -> Result<Option<T>> {
        value.as_ref().map(AgileReference::resolve).transpose()
    }
}

impl<T: Interface + RuntimeType<Default = Option<T>> + 'static> Compare for Marshaled<T> {
    fn index(values: &[Self::Stored], value: &Option<T>) -> Result<Option<u32>> {
        identity_index::<T, Self>(values, value.as_ref())
    }
}

pub(crate) fn identity_index<T, S>(values: &[S::Stored], value: Option<&T>) -> Result<Option<u32>>
where
    T: Interface + RuntimeType<Default = Option<T>>,
    S: Storage<Value = T>,
{
    let identity = value.map(Interface::cast::<IUnknown>).transpose()?;
    for (index, item) in values.iter().enumerate() {
        let item = S::resolve(item)?
            .as_ref()
            .map(Interface::cast::<IUnknown>)
            .transpose()?;
        if item == identity {
            return Ok(Some(index as u32));
        }
    }
    Ok(None)
}

pub(crate) fn prepare<S: Storage>(
    values: impl IntoIterator<Item = <S::Value as Type>::Default>,
) -> Result<Vec<S::Stored>> {
    values.into_iter().map(S::prepare).collect()
}

pub(crate) fn resolve_many<S: Storage>(
    values: &[S::Stored],
    start: usize,
    count: usize,
) -> Result<Vec<<S::Value as Type>::Default>> {
    values
        .iter()
        .skip(start)
        .take(count)
        .map(S::resolve)
        .collect()
}

fn resolve_many_locked<S: Storage>(
    values: &RwLock<Vec<S::Stored>>,
    start: usize,
    count: usize,
) -> Result<Vec<<S::Value as Type>::Default>> {
    let snapshot: Vec<_> = values
        .read()
        .unwrap()
        .iter()
        .skip(start)
        .take(count)
        .cloned()
        .collect();
    snapshot.into_iter().map(S::resolve_owned).collect()
}

pub(crate) fn write_many<T: Type>(items: &mut [T::Default], values: Vec<T::Default>) -> u32 {
    let count = values.len() as u32;
    for (item, value) in items.iter_mut().zip(values) {
        *item = value;
    }
    count
}

pub(crate) fn index_result(index: Option<u32>, result: &mut u32) -> bool {
    *result = index.unwrap_or(0);
    index.is_some()
}

pub(crate) fn move_next(current: &AtomicUsize, len: impl Fn() -> usize) -> bool {
    loop {
        let start = current.load(Ordering::Relaxed);
        let len = len();
        if start >= len {
            return false;
        }
        if current
            .compare_exchange(start, start + 1, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return start + 1 < len;
        }
    }
}

// For infallible copies. Mutable callers keep the structural read guard through copying.
pub(crate) fn reserve(
    current: &AtomicUsize,
    len: usize,
    count: usize,
) -> Option<core::ops::Range<usize>> {
    loop {
        let start = current.load(Ordering::Relaxed);
        let count = count.min(len.saturating_sub(start));
        if count == 0 {
            return None;
        }
        let end = start + count;
        if current
            .compare_exchange(start, end, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return Some(start..end);
        }
    }
}

pub(crate) fn iterator_many<T: Type>(
    current: &AtomicUsize,
    items: &mut [T::Default],
    resolve: impl Fn(usize, usize) -> Result<Vec<T::Default>>,
) -> Result<u32> {
    loop {
        let start = current.load(Ordering::Relaxed);
        let values = resolve(start, items.len())?;
        if current
            .compare_exchange(
                start,
                start + values.len(),
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            return Ok(write_many::<T>(items, values));
        }
    }
}

pub(crate) struct VectorStorage<S: Storage> {
    values: RwLock<Vec<S::Stored>>,
}

// SAFETY: retained values follow the factory's threading contract; structural access is locked.
unsafe impl<S: Storage> Send for VectorStorage<S> {}
unsafe impl<S: Storage> Sync for VectorStorage<S> {}

impl<S: Storage> VectorStorage<S> {
    #[cfg(test)]
    pub(crate) fn unlocked(&self) -> bool {
        self.values.try_write().is_ok()
    }

    pub(crate) fn new(values: Vec<S::Stored>) -> Self {
        Self {
            values: RwLock::new(values),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.values.read().unwrap().len()
    }

    pub(crate) fn snapshot(&self) -> Vec<S::Stored> {
        self.values.read().unwrap().clone()
    }

    pub(crate) fn get(&self, index: usize) -> Result<S::Value> {
        let value = self
            .values
            .read()
            .unwrap()
            .get(index)
            .cloned()
            .ok_or_else(|| Error::from(E_BOUNDS))?;
        S::get_owned(value)
    }

    pub(crate) fn index(
        &self,
        value: &<S::Value as Type>::Default,
        result: &mut u32,
    ) -> Result<bool>
    where
        S: Compare,
    {
        *result = 0;
        Ok(index_result(S::index_locked(&self.values, value)?, result))
    }

    pub(crate) fn set(&self, index: usize, value: &<S::Value as Type>::Default) -> Result<()> {
        let value = S::prepare_ref(value)?;
        let retired = {
            let mut values = self.values.write().unwrap();
            let item = values.get_mut(index).ok_or_else(|| Error::from(E_BOUNDS))?;
            core::mem::replace(item, value)
        };
        drop(retired);
        Ok(())
    }

    pub(crate) fn insert(&self, index: usize, value: &<S::Value as Type>::Default) -> Result<()> {
        let value = S::prepare_ref(value)?;
        let mut values = self.values.write().unwrap();
        if index > values.len() {
            return Err(E_BOUNDS.into());
        }
        values.insert(index, value);
        Ok(())
    }

    pub(crate) fn remove(&self, index: usize) -> Result<()> {
        let retired = {
            let mut values = self.values.write().unwrap();
            if index >= values.len() {
                return Err(E_BOUNDS.into());
            }
            values.remove(index)
        };
        drop(retired);
        Ok(())
    }

    pub(crate) fn append(&self, value: &<S::Value as Type>::Default) -> Result<u32> {
        let value = S::prepare_ref(value)?;
        let mut values = self.values.write().unwrap();
        let index = values.len() as u32;
        values.push(value);
        Ok(index)
    }

    pub(crate) fn remove_last(&self) -> Result<u32> {
        let (index, retired) = {
            let mut values = self.values.write().unwrap();
            let retired = values.pop().ok_or_else(|| Error::from(E_BOUNDS))?;
            (values.len() as u32, retired)
        };
        drop(retired);
        Ok(index)
    }

    pub(crate) fn clear(&self) {
        let retired = {
            let mut values = self.values.write().unwrap();
            if !core::mem::needs_drop::<S::Stored>() {
                values.clear();
                return;
            }
            core::mem::take(&mut *values)
        };
        drop(retired);
    }

    pub(crate) fn replace(&self, items: &[<S::Value as Type>::Default]) -> Result<()> {
        S::replace_locked(&self.values, items)
    }

    pub(crate) fn many(
        &self,
        start: usize,
        items: &mut [<S::Value as Type>::Default],
    ) -> Result<u32> {
        S::get_many_locked(&self.values, start, items)
    }

    pub(crate) fn iterator_many(
        &self,
        current: &AtomicUsize,
        items: &mut [<S::Value as Type>::Default],
    ) -> Result<u32> {
        S::iterator_many_locked(&self.values, current, items)
    }
}
