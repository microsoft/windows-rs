use std::cell::Cell;

/// Visits during a synchronous update, not distinct objects or native rendering work.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UpdateWork {
    pub views: usize,
    pub expanded: usize,
    pub validated: usize,
    pub compared: usize,
    pub reconciled: usize,
    pub refreshed: usize,
    pub window_scopes: usize,
    pub window_pending: usize,
}

thread_local! {
    static WORK: Cell<Option<UpdateWork>> = const { Cell::new(None) };
}

/// Counts work on the calling thread. Measurements cannot be nested.
pub fn measure_work<T>(operation: impl FnOnce() -> T) -> (T, UpdateWork) {
    struct Reset;

    impl Drop for Reset {
        fn drop(&mut self) {
            WORK.set(None);
        }
    }

    assert!(WORK.get().is_none(), "work measurement is already active");
    WORK.set(Some(UpdateWork::default()));
    let _reset = Reset;
    let result = operation();
    (result, WORK.get().unwrap())
}

pub(crate) fn record(update: impl FnOnce(&mut UpdateWork)) {
    if let Some(mut work) = WORK.get() {
        update(&mut work);
        WORK.set(Some(work));
    }
}
