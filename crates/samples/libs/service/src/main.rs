//use std::io::Write;
use windows_services::*;

// TODO: example of hosting a COM/WinRT server as well

// fn main() {
//     let mut log = std::fs::File::create("D:\\service.txt").unwrap();

//     options().can_pause().can_stop().control(|control| {
//         writeln!(log, "Service: {control:?}").unwrap();
//     });
// }

fn service_thread() {
    //let mut log = std::fs::File::create("D:\\service.txt").unwrap();

    for i in 0..10 {
        // writeln!(log, "...{i}\n").unwrap();

        log(&format!("t:{}...{i}\n", windows_threading::thread_id()));

        // Replace with whatever work the service needs to do.
        std::thread::sleep(std::time::Duration::from_millis(1000));

        if matches!(state(), State::StopPending | State::PausePending) {
            return;
        }
    }

    set_state(State::Stopped);
}

fn main() {
    let mut thread = None;

    options().can_pause().can_stop().run(|request| {
        log(&format!("Request: {request:?}\n"));

        match request {
            Request::Start | Request::Resume => thread = Some(std::thread::spawn(service_thread)),
            Request::Pause | Request::Stop => thread.take().unwrap().join().unwrap(),
        }
    });
}

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
