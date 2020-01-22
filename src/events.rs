use crate::*;

pub struct EventToken {
    source: ComPtr,
    token: i64,
    offset: u32, // offset of remove virtual function
    weak: bool,  // whether source is a weak/strong ref
}

impl EventToken {
    pub fn new(source: RawPtr, token: i64, offset: u32) -> EventToken {
        // TODO: query source for
        EventToken { source: source.into(), token, offset, weak: true }
    }

    pub fn guard(self) -> EventGuard {
        // TODO: return error if source is null (indicates source doesn't implement IWeakReferenceSource)
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
