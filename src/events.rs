use crate::*;

// The EventGuard drops the generic parameter from the EventToken to make storing EventGuards more convenient 
// as you don't have to figure out what type it is. The EventToken uses a generic parameter as this type doesn't
// need to be used directly and this save some stack space every time an event handler is registered

pub struct EventToken<T> {
    token: i64,
    source: ComPtr,
    offset: u32, // offset of remove virtual function
    __0: std::marker::PhantomData<T>,
}

impl<T: TypeGuid> EventToken<T> {
    pub fn new(source: RawPtr, token: i64, offset: u32) -> EventToken<T> {
        EventToken { token, source: ComPtr::addref(source), offset, __0: std::marker::PhantomData }
    }

    pub fn guard(mut self) -> EventGuard {
        unsafe {
            let weak_source = self.source.query::<IWeakReferenceSource>();
            let weak = !weak_source.is_null();
            if weak {
                ((*(*(weak_source.get() as *const *const IWeakReferenceSource))).weak)(weak_source.get(), self.source.set()).unwrap();
            }
            EventGuard { guid: *T::type_guid(), token: self.token, source: self.source, offset: self.offset, weak: weak }
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
        // let source = if self.token.weak {
        //     let mut ptr = std::ptr::null_mut();
        //     ((*(*(self.token.source.0 as *const *const IWeakReference))).strong)(self.token.source.0, T::type_guid(), &mut ptr);
        //     ptr.into()
        // } else {
        //     self.token.source
        // };
        // if !source.is_null() {

        // }
    }
}
