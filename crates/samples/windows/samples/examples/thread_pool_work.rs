fn main() -> windows::core::Result<()> {
    use windows::{threadpoolapiset::*, winnt::*};

    static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

    unsafe extern "system" fn callback(
        _: *mut TP_CALLBACK_INSTANCE,
        _: *mut std::ffi::c_void,
        _: *mut TP_WORK,
    ) {
        let mut counter = COUNTER.write().unwrap();
        *counter += 1;
    }

    unsafe {
        let work = CreateThreadpoolWork(Some(callback), None, None);
        if work.is_null() {
            return Err(windows::core::Error::from_thread());
        }

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, false);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
    Ok(())
}
