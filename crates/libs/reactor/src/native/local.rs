use super::bindings::*;
use std::{
    any::*, cell::*, collections::*, marker::PhantomData, rc::*, sync::atomic::*, thread::*,
};
use windows_core::{Result, *};

thread_local! {
    static VALUES: RefCell<HashMap<u64, Rc<dyn Any>>> = RefCell::new(HashMap::new());
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
const RPC_E_WRONG_THREAD: HRESULT = HRESULT(0x8001010Eu32 as i32);
const RO_E_CLOSED: HRESULT = HRESULT(0x80000013u32 as i32);

fn register<T: 'static>(value: T) -> u64 {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    VALUES.with(|values| assert!(values.borrow_mut().insert(id, Rc::new(value)).is_none()));
    id
}

fn remove(id: u64) {
    let value = VALUES.with(|values| values.borrow_mut().remove(&id));
    drop(value);
}

fn with_value<T: 'static>(id: u64, owner: ThreadId, callback: impl FnOnce(&T)) {
    assert_eq!(current().id(), owner, "callback invoked off its UI thread");
    let value = VALUES.with(|values| values.borrow().get(&id).cloned());
    // App shutdown cancels registrations even if native code still holds the delegate.
    if let Some(value) = value {
        callback(value.downcast_ref::<T>().unwrap());
    }
}

fn try_with_value<T: 'static, R>(
    id: u64,
    owner: ThreadId,
    callback: impl FnOnce(&T) -> R,
) -> Result<R> {
    if current().id() != owner {
        return Err(Error::from_hresult(RPC_E_WRONG_THREAD));
    }
    let value = VALUES.with(|values| values.borrow().get(&id).cloned());
    let value = value.ok_or_else(|| Error::from_hresult(RO_E_CLOSED))?;
    Ok(callback(value.downcast_ref::<T>().unwrap()))
}

pub(super) fn clear() {
    let values = VALUES.with(|values| std::mem::take(&mut *values.borrow_mut()));
    drop(values);
}

pub(super) struct Local<T> {
    id: u64,
    owner: ThreadId,
    dispatcher: DispatcherQueue,
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Local<T> {
    pub(super) fn new(value: T) -> Self {
        let dispatcher = DispatcherQueue::GetForCurrentThread().unwrap();
        Self {
            id: register(value),
            owner: current().id(),
            dispatcher,
            marker: PhantomData,
        }
    }

    fn with(&self, callback: impl FnOnce(&T)) {
        with_value(self.id, self.owner, callback);
    }

    pub(super) fn try_with<R>(&self, callback: impl FnOnce(&T) -> R) -> Result<R> {
        try_with_value(self.id, self.owner, callback)
    }
}

impl<T> Drop for Local<T> {
    fn drop(&mut self) {
        if current().id() == self.owner {
            remove(self.id);
        } else {
            let id = self.id;
            let handler = DispatcherQueueHandler::new(move || remove(id));
            // Rejected cleanup is covered by the UI thread's shutdown clear.
            _ = self
                .dispatcher
                .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)
                .unwrap();
        }
    }
}

pub(super) fn task(callback: impl Fn() + 'static) -> impl Fn() + Send + Sync {
    let callback = Local::new(callback);
    move || callback.with(|callback| callback())
}

pub(super) fn handler<A, B, F>(callback: F) -> impl Fn(InRef<'_, A>, InRef<'_, B>) + Send + Sync
where
    A: RuntimeType + 'static,
    B: RuntimeType + 'static,
    F: Fn(InRef<'_, A>, InRef<'_, B>) + 'static,
{
    let callback = Local::new(callback);
    move |sender, args| callback.with(|callback| callback(sender, args))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::*;

    struct Capture(Arc<Mutex<Vec<ThreadId>>>);

    impl Drop for Capture {
        fn drop(&mut self) {
            self.0.lock().unwrap().push(current().id());
        }
    }

    #[test]
    fn owner_thread_access_and_destruction() {
        let owner = current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let id = register(Capture(drops.clone()));
        spawn(move || {
            assert!(std::panic::catch_unwind(|| with_value::<Capture>(id, owner, |_| {})).is_err());
        })
        .join()
        .unwrap();
        assert!(drops.lock().unwrap().is_empty());
        remove(id);
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }

    #[test]
    fn removal_during_callback() {
        let owner = current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let id = register(Capture(drops.clone()));
        with_value::<Capture>(id, owner, |_| {
            remove(id);
            assert!(drops.lock().unwrap().is_empty());
            with_value::<Capture>(id, owner, |_| panic!("canceled callback"));
        });
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }

    #[test]
    fn shutdown_cancels_callbacks() {
        let owner = current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let id = register(Capture(drops.clone()));
        clear();
        with_value::<Capture>(id, owner, |_| panic!("callback after shutdown"));
        remove(id);
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }

    #[cfg(all(windows, not(miri), not(windows_slim_errors)))]
    #[test]
    fn canceled_access_preserves_native_error_info() {
        let owner = current().id();
        let id = register(());
        remove(id);
        let code: HRESULT =
            Error::new(HRESULT(0x80004005_u32 as i32), "pending native error").into();
        with_value::<()>(id, owner, |_| panic!("canceled callback"));
        let error = try_with_value(id, owner, |_: &()| panic!("canceled access")).unwrap_err();
        assert_eq!(error.code(), RO_E_CLOSED);
        assert_eq!(Error::from(code).message(), "pending native error");
    }

    #[test]
    fn com_access_rejects_wrong_thread_and_canceled_state() {
        let owner = current().id();
        let id = register(Rc::new(Cell::new(7)));
        assert_eq!(
            try_with_value(id, owner, |value: &Rc<Cell<i32>>| value.get()).unwrap(),
            7
        );
        spawn(move || {
            let result = try_with_value(id, owner, |_: &Rc<Cell<i32>>| {
                panic!("wrong-thread access reached UI state");
            });
            assert_eq!(result.unwrap_err().code(), RPC_E_WRONG_THREAD);
        })
        .join()
        .unwrap();
        remove(id);
        let result = try_with_value(id, owner, |_: &Rc<Cell<i32>>| {
            panic!("canceled access reached UI state");
        });
        assert_eq!(result.unwrap_err().code(), RO_E_CLOSED);
    }
}
