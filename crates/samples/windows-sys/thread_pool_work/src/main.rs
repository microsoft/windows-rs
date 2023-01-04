use windows_sys::{Win32::Foundation::*, Win32::System::Threading::*};

static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

fn main() {
    unsafe {
        let work = CreateThreadpoolWork(Some(callback), std::ptr::null_mut(), std::ptr::null());

        if work.is_null() {
            println!("{:?}", GetLastError());
            return;
        }

        for _ in 0..10 {
            SubmitThreadpoolWork(work);
        }

        WaitForThreadpoolWorkCallbacks(work, 0);
    }

    let counter = COUNTER.read().unwrap();
    println!("counter: {}", *counter);
}

extern "system" fn callback(
    _: *mut TP_CALLBACK_INSTANCE,
    _: *mut std::ffi::c_void,
    _: *mut TP_WORK,
) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}
