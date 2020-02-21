use crate::weak::abi::{IWeakReference, IWeakReferenceSource};
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
        let weak_source = self
            .source
            .ptr
            .as_ref()
            .and_then(|i| i.get_interface::<dyn IWeakReferenceSource>());

        let source = if let Some(weak_source) = weak_source {
            unsafe {
                weak_source.get_weak_reference(self.source.set()).unwrap();
                let ptr = self.source.get();
                debug_assert!(
                    !ptr.is_null(),
                    "Pointer was null after successful `get_weak_reference` call"
                );

                EventGuardSource::Weak(com::ComPtr::new(ptr as *mut _))
            }
        } else {
            EventGuardSource::Strong(self.source)
        };
        EventGuard {
            guid: *T::type_guid(),
            token: self.token,
            source,
            offset: self.offset,
        }
    }
}

pub struct EventGuard {
    guid: Guid, // IID of interface contaiing
    token: i64,
    source: EventGuardSource,
    offset: u32, // offset of remove virtual function
}

enum EventGuardSource {
    Strong(ComPtr),
    Weak(com::ComPtr<dyn IWeakReference>),
}

impl Drop for EventGuard {
    fn drop(&mut self) {
        unsafe {
            if let EventGuardSource::Weak(ref weak) = self.source {
                let mut ptr = std::ptr::null_mut();
                weak.resolve(&self.guid, &mut ptr);

                self.source = EventGuardSource::Strong(std::mem::transmute(ptr));
            }
            // if !self.source.is_null() {
            // TODO: find the offset function pointer and call it
            // }
        }
    }
}
