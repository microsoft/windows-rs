#[expect(nonstandard_style, clippy::upper_case_acronyms)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> windows_result::Result<()> {
    use bindings::*;

    static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

    unsafe extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: PVOID, _: PTP_WORK) {
        let mut counter = COUNTER.write().unwrap();
        *counter += 1;
    }

    unsafe {
        let work = CreateThreadpoolWork(Some(callback), std::ptr::null_mut(), std::ptr::null_mut());
        if work.is_null() {
            return Err(windows_result::Error::from_thread());
        }

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, 0);
        CloseThreadpoolWork(work);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
    Ok(())
}
