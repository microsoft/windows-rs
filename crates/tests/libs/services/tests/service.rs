use windows_services::*;
use windows_sys::Win32::System::Services::*;

#[test]
fn start_stop() {
    let mut log = vec![];

    Service::new()
        .can_fallback(|_| {})
        .run(|service, command| {
            assert!(service.handle().is_null());
            log.push(command);
        })
        .unwrap();

    assert_eq!(log, [Command::Start, Command::Stop,]);
}

#[test]
fn pause_resume() {
    let mut log = vec![];

    Service::new()
        .can_fallback(|service| {
            service.handler(SERVICE_CONTROL_PAUSE, 0, std::ptr::null_mut());
            service.handler(SERVICE_CONTROL_CONTINUE, 0, std::ptr::null_mut());
        })
        .run(|_, command| {
            log.push(command);
        })
        .unwrap();

    assert_eq!(
        log,
        [
            Command::Start,
            Command::Pause,
            Command::Resume,
            Command::Stop,
        ]
    );
}

#[test]
fn extended() {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct Data {
        a: u8,
        b: u8,
        c: u8,
    }
    static DATA: Data = Data { a: 7, b: 8, c: 9 };
    const DATA_PTR: *const std::ffi::c_void = &DATA as *const Data as *const std::ffi::c_void;

    let mut log = vec![];

    Service::new()
        .can_fallback(|service| {
            service.handler(123, 456, DATA_PTR);
        })
        .run(|_, command| {
            log.push(command);

            if let Command::Extended(command) = command {
                unsafe {
                    let data = *(command.data as *const Data);
                    assert_eq!(data, Data { a: 7, b: 8, c: 9 });
                }
            }
        })
        .unwrap();

    assert_eq!(
        log,
        [
            Command::Start,
            Command::Extended(ExtendedCommand {
                control: 123,
                ty: 456,
                data: DATA_PTR
            }),
            Command::Stop,
        ]
    );
}

#[test]
#[should_panic(expected = "Use service control manager to start service")]
fn panic_fallback() {
    Service::new().run(|_, _| {}).unwrap();
}

#[test]
fn recover_fallback() {
    let error = Service::new().run(|_, _| {}).unwrap_err();
    assert_eq!(error, "Use service control manager to start service");
}

#[test]
#[should_panic(expected = "`run` was already called")]
fn rerun() {
    let mut service = Service::new();
    service.run(|_, _| {}).unwrap_err();
    _ = service.run(|_, _| {});
}
