use crate::*;

pub struct EventToken {
    source: ComPtr,
    token: i64,
    offset: u32, // offset of remove virtual function
    weak: bool,  // whether source is a weak/strong ref
}

impl EventToken {
    pub fn new(source: RawPtr, token: i64, offset: u32) -> EventToken {
        let mut source : ComPtr = source.into();
        let weak_source : ComPtr = source.query::<IWeakReferenceSource>().into();
        let weak = !weak_source.is_null();
        if weak {
            (weak_source.deref::<IWeakReferenceSource>().reference)(source.set()).unwrap();
        }
        EventToken { source: source, token, offset, weak }
    }

    pub fn guard(self) -> EventGuard {
        EventGuard { token: self }
    }
}

pub struct EventGuard {
    token: EventToken,
}

impl Drop for EventGuard {
    fn drop(&mut self) {
        // TODO: resolve weak ref and remove event registration
    }
}
