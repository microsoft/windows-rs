use windows_services::*;

pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;

#[test]
fn test_thread() {
    start_stop();
    pause_resume();
}

fn start_stop() {
    let mut log = vec![];

    Service::new()
        .can_fallback(|| {
            //handler(SERVICE_CONTROL_CONTINUE);
        })
        .run(|command| {
            log.push(format!("{command:?}"));
        });

    assert_eq!(log, [
        "Start",
        "Stop",
    ]);
}

fn pause_resume() {
    let mut log = vec![];

    Service::new()
        .can_fallback(|| {
            handler(SERVICE_CONTROL_PAUSE);
            handler(SERVICE_CONTROL_CONTINUE);
        })
        .run(|command| {
            log.push(format!("{command:?}"));
        });

    assert_eq!(log, [
        "Start",
        "Pause",
        "Resume",
        "Stop",
    ]);
}

#[test]
fn test_no_fallback() {
    // TODO: capture stdout?
}