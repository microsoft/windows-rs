use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use storage::*;
use windows_core::*;

type Value<O> = <<O as VectorOwner>::Storage as Storage>::Value;

pub(crate) trait VectorOwner: ComObjectInner + Send + Sync + 'static {
    type Storage: Storage;

    fn values(&self) -> &[<Self::Storage as Storage>::Stored];

    fn get_at(&self, index: usize) -> Result<Value<Self>> {
        Self::Storage::get(
            self.values()
                .get(index)
                .ok_or_else(|| Error::from(E_BOUNDS))?,
        )
    }

    fn size(&self) -> Result<u32> {
        Ok(self.values().len().try_into()?)
    }

    fn get_many(&self, start: u32, items: &mut [<Value<Self> as Type>::Default]) -> Result<u32> {
        Self::Storage::get_many(self.values(), start as usize, items)
    }
}

pub(crate) fn iterator<O: VectorOwner>(owner: ComObject<O>) -> IIterator<Value<O>> {
    ComObject::new(ImmutableVectorIterator {
        owner,
        current: AtomicUsize::new(0),
    })
    .into_interface()
}

struct ImmutableVectorIterator<O: VectorOwner> {
    owner: ComObject<O>,
    current: AtomicUsize,
}

implement_decl! {
    impl<O> ImmutableVectorIterator as ImmutableVectorIterator_Impl: [IIterator<Value<O>>]
    where O: VectorOwner
}

impl<O: VectorOwner> IIterator_Impl<Value<O>> for ImmutableVectorIterator_Impl<O> {
    fn Current(&self) -> Result<Value<O>> {
        self.owner
            .get()
            .get_at(self.current.load(Ordering::Relaxed))
    }

    fn HasCurrent(&self) -> Result<bool> {
        Ok(self.current.load(Ordering::Relaxed) < self.owner.get().values().len())
    }

    fn MoveNext(&self) -> Result<bool> {
        Ok(move_next(&self.current, || self.owner.get().values().len()))
    }

    fn GetMany(&self, items: &mut [<Value<O> as Type>::Default]) -> Result<u32> {
        O::Storage::iterator_many(self.owner.get().values(), &self.current, items)
    }
}
