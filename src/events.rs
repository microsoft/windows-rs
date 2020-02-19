use crate::*;

// The EventGuard avoids the generic parameter from the EventToken to make storing EventGuards more convenient
// as you don't have to figure out what type it is. The EventToken uses a generic parameter as this type doesn't
// need to be uttered directly and this is more efficient especially for cases where the guard isn't used.
// The EventGuard will attempt to store a weak ref otherwise will fall back to strong ref

pub struct EventToken<T> {
    token: i64,
    source: ComPtr,
    offset: u32, // offset of remove virtual function
    __0: std::marker::PhantomData<T>,
}

impl<T: QueryType> EventToken<T> {
    pub fn new(source: &ComPtr, token: i64, offset: u32) -> EventToken<T> {
        EventToken {
            token,
            source: source.clone(),
            offset,
            __0: std::marker::PhantomData,
        }
    }

    pub fn guard(mut self) -> EventGuard {
        unsafe {
            let weak_source = self.source.query::<IWeakReferenceSource>();
            let weak = !weak_source.is_null();
            if weak {
                ((*(*(weak_source.get() as *const *const IWeakReferenceSource))).weak)(
                    weak_source.get(),
                    self.source.set(),
                )
                .unwrap();
            }
            EventGuard {
                guid: *T::type_guid(),
                token: self.token,
                source: self.source,
                offset: self.offset,
                weak: weak,
            }
        }
    }
}

pub struct EventGuard {
    guid: Guid, // IID of interface contaiing
    token: i64,
    source: ComPtr,
    offset: u32, // offset of remove virtual function
    weak: bool,  // whether source is a weak/strong ref
}

impl Drop for EventGuard {
    fn drop(&mut self) {
        unsafe {
            if self.weak {
                let mut ptr = std::ptr::null_mut();
                ((*(*(self.source.get() as *const *const IWeakReference))).strong)(
                    self.source.get(),
                    &self.guid,
                    &mut ptr,
                );
                self.source = std::mem::transmute(ptr);
            }
            if !self.source.is_null() {
                // TODO: find the offset function pointer and call it
            }
        }
    }
}
