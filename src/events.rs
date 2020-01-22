use crate::*;

pub struct EventToken<T> {
    source: ComPtr,
    token: i64,
    offset: u32, // offset of remove virtual function
    weak: bool,  // whether source is a weak/strong ref
    __0: std::marker::PhantomData<T>,
}

impl<T: TypeGuid> EventToken<T> {
    pub fn new(mut source: RawPtr, token: i64, offset: u32) -> EventToken<T> {
        unsafe {
            let weak_source: ComPtr = query::<IWeakReferenceSource>(source).into();
            let weak = !weak_source.is_null();
            if weak {
                ((*(*(weak_source.0 as *const *const IWeakReferenceSource))).weak)(weak_source.0, &mut source).unwrap();
            }
            EventToken { source: source.into(), token, offset, weak, __0: std::marker::PhantomData }
        }
    }

    pub fn guard(self) -> EventGuard<T> {
        EventGuard { token: self }
    }
}

pub struct EventGuard<T: TypeGuid> {
    token: EventToken<T>,
}

impl<T: TypeGuid> Drop for EventGuard<T> {
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
