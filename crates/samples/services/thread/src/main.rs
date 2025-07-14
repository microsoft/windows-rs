use windows_services::*;
use windows_threading::*;

fn main() {
    let pool = Pool::new();
    pool.set_thread_limits(1, 1);

    Service::new()
        .can_pause()
        .can_stop()
        .can_fallback(|_| {
            println!("Press Enter to stop service.");
            use std::io::Read;
            _ = std::io::stdin().read(&mut [0]);
        })
        .run(|service, command| {
            log(&format!("Command: {command:?}\n"));

            match command {
                Command::Start | Command::Resume => pool.submit(|| service_thread(service)),
                Command::Pause | Command::Stop => pool.join(),
                _ => {}
            }
        })
        .unwrap();
}

fn service_thread(service: &Service) {
    for i in 0..10 {
        log(&format!("Thread:{}... iteration:{i}\n", thread_id()));

        // Replace with whatever work the service needs to do.
        sleep(1000);

        // Services can use the `state` function to query the current service state.
        if matches!(service.state(), State::StopPending | State::PausePending) {
            return;
        }
    }

    // Services can use the `set_state` function to update the service state.
    service.set_state(State::Stopped);
}

// Simple log function can be used to observe service behavior.
fn log(message: &str) {
    use windows_sys::{core::*, Win32::Foundation::*, Win32::Storage::FileSystem::*};

    unsafe {
        let file = CreateFileW(
            w!("D:\\service.txt"),
            FILE_APPEND_DATA,
            0,
            std::ptr::null(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut(),
        );

        WriteFile(
            file,
            message.as_ptr(),
            message.len().try_into().unwrap(),
            &mut 0,
            std::ptr::null_mut(),
        );

        CloseHandle(file);
    }
}
