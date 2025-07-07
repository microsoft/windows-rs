use windows_services::*;

pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;

#[test]
fn start_stop() {
    let mut log = vec![];

    Service::new()
        .can_fallback(|_| {})
        .run(|_, command| {
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
#[should_panic(expected = "Use service control manager to start service")]
fn panic_fallback() {
    Service::new().run(|_, _| {}).unwrap();
}

#[test]
fn recover_fallback() {
    let error = Service::new().run(|_, _| {}).unwrap_err();
    assert_eq!(error, "Use service control manager to start service");
}
