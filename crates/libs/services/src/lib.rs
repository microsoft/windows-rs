#![doc = include_str!("../readme.md")]
#![cfg(windows)]

mod bindings;
use bindings::*;
use std::boxed::Box;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Request {
    Pause,
    Resume,
    Start,
    Stop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    ContinuePending,
    Paused,
    PausePending,
    Running,
    StartPending,
    Stopped,
    StopPending,
}

#[derive(Debug)]
struct Handle(SERVICE_STATUS_HANDLE);
static HANDLE: OnceLock<Handle> = OnceLock::new();
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

fn handle() -> SERVICE_STATUS_HANDLE {
    HANDLE.get().unwrap().0
}

#[derive(Debug)]
struct Callback(*mut (dyn FnMut(Request) + Send + Sync));
static CALLBACK: OnceLock<Callback> = OnceLock::new();
unsafe impl Send for Callback {}
unsafe impl Sync for Callback {}

fn callback(request: Request) {
    unsafe {
        (*CALLBACK.get().unwrap().0)(request);
    }
}

static STATUS: RwLock<SERVICE_STATUS> = RwLock::new(SERVICE_STATUS {
    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
    dwCurrentState: SERVICE_STOPPED,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
});

pub fn set_state(state: State) {
    let mut writer = STATUS.write().unwrap();
    writer.dwCurrentState = match state {
        State::ContinuePending => SERVICE_CONTINUE_PENDING,
        State::Paused => SERVICE_PAUSED,
        State::PausePending => SERVICE_PAUSE_PENDING,
        State::Running => SERVICE_RUNNING,
        State::StartPending => SERVICE_START_PENDING,
        State::Stopped => SERVICE_STOPPED,
        State::StopPending => SERVICE_STOP_PENDING,
    };

    // Makes a copy to avoid holding a lock while calling `SetServiceStatus`.
    let status: SERVICE_STATUS = *writer;
    drop(writer);

    unsafe {
        SetServiceStatus(handle(), &status);
    }
}

pub fn state() -> State {
    let reader = STATUS.read().unwrap();

    match reader.dwCurrentState {
        SERVICE_CONTINUE_PENDING => State::ContinuePending,
        SERVICE_PAUSED => State::Paused,
        SERVICE_PAUSE_PENDING => State::PausePending,
        SERVICE_RUNNING => State::Running,
        SERVICE_START_PENDING => State::StartPending,
        SERVICE_STOPPED => State::Stopped,
        SERVICE_STOP_PENDING => State::StopPending,
        _ => panic!("unexpected state"),
    }
}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let handle = unsafe { RegisterServiceCtrlHandlerW(std::ptr::null(), Some(handler)) };

    HANDLE.set(Handle(handle)).unwrap();
    set_state(State::StartPending);
    callback(Request::Start);
    set_state(State::Running);
}

extern "system" fn handler(control: u32) {
    match control {
        SERVICE_CONTROL_CONTINUE if state() == State::Paused => {
            set_state(State::ContinuePending);
            callback(Request::Resume);
            set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if state() == State::Running => {
            set_state(State::PausePending);
            callback(Request::Pause);
            set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            set_state(State::StopPending);
            callback(Request::Stop);
            set_state(State::Stopped);
        }
        _ => {}
    }
}

#[derive(Default)]
pub struct Options(u32);

pub fn options() -> Options {
    Options::new()
}

impl Options {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn can_stop(&mut self) -> &mut Self {
        self.0 |= SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
        self
    }

    pub fn can_pause(&mut self) -> &mut Self {
        self.0 |= SERVICE_ACCEPT_PAUSE_CONTINUE;
        self
    }

    pub fn run<F: FnMut(Request) + Send + Sync>(&self, callback: F) -> ! {
        STATUS.write().unwrap().dwControlsAccepted = self.0;
        CALLBACK
            .set(Callback(Box::into_raw(Box::new(callback)) as *mut _))
            .unwrap();

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        if unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) } == 0 {
            println!(
                r#"Use Service Request Manager to start service.
    
Install:
    > sc create ServiceName binPath= "{}"

Start:
    > sc start ServiceName

Query status:
    > sc query ServiceName

Stop:
    > sc stop ServiceName

Delete (uninstall):
    > sc delete ServiceName
"#,
                std::env::current_exe().unwrap().display()
            );
        }

        std::process::exit(0);
    }
}
