use windows_sys::{
    core::*, Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::System::Services::*,
};

// Sample logs to this file for illustration purposes.
const LOG_FILE: PCWSTR = w!("D:\\service.txt");

// The service spawns a thread and calls `service_thread` to do some actual work.
fn service_thread() {
    for i in 0..10 {
        log(&format!("...{i}\n"));

        // Replace with whatever work the service needs to do.
        std::thread::sleep(std::time::Duration::from_millis(1000));

        if stop_request() {
            break;
        }
    }
}

// Minimal state required to support the service sample.
struct State {
    // Service handle returned by `RegisterServiceCtrlHandlerW` and used to update the service status.
    handle: SERVICE_STATUS_HANDLE,

    // The join handle used to wait for the service thread to stop.
    thread: Option<std::thread::JoinHandle<()>>,

    // The service status and supported service bevavior for `SetServiceStatus`.
    status: SERVICE_STATUS,
}

impl State {
    const fn new() -> Self {
        Self {
            handle: std::ptr::null_mut(),
            thread: None,
            status: SERVICE_STATUS {
                dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                dwCurrentState: SERVICE_STOPPED,
                dwControlsAccepted: SERVICE_ACCEPT_PAUSE_CONTINUE
                    | SERVICE_ACCEPT_STOP
                    | SERVICE_ACCEPT_SHUTDOWN,
                dwWin32ExitCode: 0,
                dwServiceSpecificExitCode: 0,
                dwCheckPoint: 0,
                dwWaitHint: 0,
            },
        }
    }
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

static STATE: std::sync::RwLock<State> = std::sync::RwLock::new(State::new());

fn main() {
    let table = [
        SERVICE_TABLE_ENTRYW {
            lpServiceName: &mut 0,
            lpServiceProc: Some(service_main),
        },
        SERVICE_TABLE_ENTRYW::default(),
    ];

    unsafe {
        if StartServiceCtrlDispatcherW(table.as_ptr()) == 0 {
            println!(
                r#"Use Service Control Manager to start service.

Install:
  > sc create rust binPath= "{}"

Start:
  > sc start rust

Query status:
  > sc query rust

Stop:
  > sc stop rust

Delete (uninstall):
  > sc delete rust
"#,
                std::env::current_exe().unwrap().display()
            );
        }
    }

    log("service exit\n");
}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    unsafe {
        set_handle(RegisterServiceCtrlHandlerW(std::ptr::null(), Some(handler)));
    }

    set_state(SERVICE_START_PENDING);
    log("service start pending\n");
    set_thread();
    set_state(SERVICE_RUNNING);
    log("service running\n");
}

extern "system" fn handler(control: u32) {
    match control {
        SERVICE_CONTROL_CONTINUE if state() == SERVICE_PAUSED => {
            set_state(SERVICE_CONTINUE_PENDING);
            log("service continue pending\n");
            set_thread();
            set_state(SERVICE_RUNNING);
            log("service running\n");
        }
        SERVICE_CONTROL_PAUSE if state() == SERVICE_RUNNING => {
            set_state(SERVICE_PAUSE_PENDING);
            log("service pause pending\n");
            join_thread();
            set_state(SERVICE_PAUSED);
            log("service paused\n");
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            if state() == SERVICE_RUNNING {
                set_state(SERVICE_STOP_PENDING);
                log("service stop pending\n");
                join_thread();
            }

            set_state(SERVICE_STOPPED);
            log("service stopped\n");
        }
        _ => {
            log(&format!("ignored control: {control}\n"));
        }
    }
}

fn set_handle(handle: SERVICE_STATUS_HANDLE) {
    let mut writer = STATE.write().unwrap();
    writer.handle = handle;
}

fn set_state(state: SERVICE_STATUS_CURRENT_STATE) {
    let mut writer = STATE.write().unwrap();
    writer.status.dwCurrentState = state;

    // Makes a copy to avoid holding a lock while calling `SetServiceStatus`.
    let handle = writer.handle;
    let status = writer.status;
    drop(writer);

    unsafe {
        SetServiceStatus(handle, &status);
    }
}

fn set_thread() {
    let thread = std::thread::spawn(|| {
        service_thread();

        // If the service thread returns without an external request, the service will signal that
        // it has stopped and cause the process to terminate normally.
        if state() == SERVICE_RUNNING {
            set_state(SERVICE_STOPPED);
            log("service stopped\n");
        }
    });

    let mut writer = STATE.write().unwrap();
    debug_assert!(writer.thread.is_none());
    writer.thread = Some(thread);
}

fn state() -> SERVICE_STATUS_CURRENT_STATE {
    let reader = STATE.read().unwrap();
    reader.status.dwCurrentState
}

fn stop_request() -> bool {
    matches!(state(), SERVICE_PAUSE_PENDING | SERVICE_STOP_PENDING)
}

fn join_thread() {
    let mut writer = STATE.write().unwrap();

    // Takes the join handle and drops the lock before calling `join`.
    let thread = writer.thread.take();
    drop(writer);

    thread.unwrap().join().unwrap();
}

// Simply appends the string to the log file.
fn log(message: &str) {
    unsafe {
        let file = CreateFileW(
            LOG_FILE,
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
