use super::bindings::{DispatcherQueue, DispatcherQueueHandler, DispatcherQueuePriority};
use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    thread::{ThreadId, current},
};
use windows_core::{InRef, RuntimeType};

thread_local! {
    static VALUES: RefCell<HashMap<u64, Rc<dyn Any>>> = RefCell::new(HashMap::new());
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

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

pub(super) fn clear() {
    let values = VALUES.with(|values| std::mem::take(&mut *values.borrow_mut()));
    drop(values);
}

struct Local<T> {
    id: u64,
    owner: ThreadId,
    dispatcher: DispatcherQueue,
    marker: PhantomData<fn() -> T>,
}

impl<T: 'static> Local<T> {
    fn new(value: T) -> Self {
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
    use std::sync::{Arc, Mutex};

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
        std::thread::spawn(move || {
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
}
