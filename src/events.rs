use crate::*;

// The EventGuard avoids the generic parameter from the EventToken to make storing EventGuards more convenient
// as you don't have to figure out what type it is. The EventToken uses a generic parameter as this type doesn't
// need to be uttered directly and this is more efficient especially for cases where the guard isn't used.
// The EventGuard will attempt to store a weak ref otherwise will fall back to strong ref

pub struct EventToken<T: com::ComInterface> {
    token: i64,
    source: com::InterfaceRc<T>,
    offset: u32, // offset of remove virtual function
}

impl<T: com::ComInterface> EventToken<T> {
    pub fn new(source: &com::InterfaceRc<T>, token: i64, offset: u32) -> EventToken<T> {
        EventToken { token, source: source.clone(), offset }
    }

    pub fn guard(mut self) -> EventGuard {
        let source = self.source.get_interface::<dyn IWeakReferenceSource>().map(|source| {
            let weak = std::ptr::null_mut();
            unsafe {
                source.get_weak_reference(weak);
                com::InterfaceRc::from_raw(weak)
            }
        });

        EventGuard { iid: T::IID, token: self.token, source, offset: self.offset }
    }
}

pub struct EventGuard {
    iid: com::sys::IID,
    token: i64,
    source: Option<com::InterfaceRc<dyn IWeakReference>>,
    offset: u32, // offset of remove virtual function
}

impl Drop for EventGuard {
    fn drop(&mut self) {
        if let Some(ref weak) = self.source {
            let strong = std::ptr::null_mut();
            unsafe {
                weak.resolve(&self.iid, strong);
                com::InterfaceRc::<dyn IInspectable>::from_raw(strong);
            }
        }
        // TODO: find the offset of the remove function pointer and call it
    }
}
