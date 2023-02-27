use windows::{core::Result, Win32::System::Threading::*};

static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

fn main() -> Result<()> {
    unsafe {
        let work = CreateThreadpoolWork(Some(callback), None, None)?;

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, false);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
    Ok(())
}

extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: *mut std::ffi::c_void, _: PTP_WORK) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}
