use super::*;
use std::{
    mem::ManuallyDrop,
    thread::{self, ThreadId},
};

/// Keeps callback state on its creating STA. Use only inside non-agile COM implementations.
pub(crate) struct OwnerThread<T> {
    owner: ThreadId,
    value: ManuallyDrop<T>,
}

// Only the owner can obtain a reference to T. ManuallyDrop also prevents unwinding from
// destroying T on a foreign thread. Non-agile STA marshaling routes ordinary COM releases
// to the owner; bypassing that contract fails before accessing or destroying the value.
unsafe impl<T> Send for OwnerThread<T> {}
unsafe impl<T> Sync for OwnerThread<T> {}

impl<T> OwnerThread<T> {
    pub(crate) fn new(value: T) -> Result<Self> {
        let mut apartment = Default::default();
        let mut qualifier = Default::default();
        let result = unsafe { CoGetApartmentType(&mut apartment, &mut qualifier) };
        if result.is_err() {
            return Err(Error::from_hresult(result));
        }
        if apartment != APTTYPE_STA && apartment != APTTYPE_MAINSTA {
            return Err(Error::from_hresult(RPC_E_WRONG_THREAD));
        }
        Ok(Self {
            owner: thread::current().id(),
            value: ManuallyDrop::new(value),
        })
    }

    pub(crate) fn invoke(&self, callback: impl FnOnce(&T) -> HRESULT) -> HRESULT {
        assert_eq!(
            thread::current().id(),
            self.owner,
            "WebView callback accessed outside its owner thread"
        );
        callback(&self.value)
    }
}

impl<T> Drop for OwnerThread<T> {
    fn drop(&mut self) {
        assert_eq!(
            thread::current().id(),
            self.owner,
            "WebView callback dropped outside its owner thread"
        );
        unsafe { ManuallyDrop::drop(&mut self.value) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        marker::PhantomData,
        panic::{AssertUnwindSafe, catch_unwind},
        rc::Rc,
        sync::Mutex,
    };

    #[test]
    fn foreign_access_is_rejected() {
        let state = Rc::new(Cell::new(0));
        let value = OwnerThread {
            owner: thread::current().id(),
            value: ManuallyDrop::new(state.clone()),
        };
        thread::scope(|scope| {
            scope.spawn(|| {
                assert!(
                    catch_unwind(AssertUnwindSafe(|| {
                        value.invoke(|state| {
                            state.set(100);
                            HRESULT(0)
                        })
                    }))
                    .is_err()
                );
            });
            assert_eq!(
                value.invoke(|state| {
                    state.set(1);
                    HRESULT(0)
                }),
                HRESULT(0)
            );
        });
        assert_eq!(state.get(), 1);
        drop(value);
        assert_eq!(Rc::strong_count(&state), 1);
    }

    #[test]
    fn foreign_drop_does_not_destroy_the_payload() {
        struct DropProbe<'a>(&'a Mutex<bool>, PhantomData<Rc<()>>);

        impl Drop for DropProbe<'_> {
            fn drop(&mut self) {
                *self.0.lock().unwrap() = true;
            }
        }

        let dropped = Mutex::new(false);
        let value = OwnerThread {
            owner: thread::current().id(),
            value: ManuallyDrop::new(DropProbe(&dropped, PhantomData)),
        };
        let result = thread::scope(|scope| {
            scope
                .spawn(move || catch_unwind(AssertUnwindSafe(|| drop(value))))
                .join()
                .unwrap()
        });
        assert!(result.is_err());
        assert!(!*dropped.lock().unwrap());
    }
}
