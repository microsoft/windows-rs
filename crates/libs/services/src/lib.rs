#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(clippy::needless_doctest_main)]

mod bindings;
use bindings::*;
use std::boxed::Box;
use std::sync::{OnceLock, RwLock};

/// The commands are sent by the service control manager to the service through the closure or callback
/// passed to the service `run` method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    /// The start command is sent when the service first starts.
    Start,

    /// The stop command is sent when the service is stopping just prior to process termination.
    ///
    /// This command will only be sent if the `can_stop` method is called as part of construction.
    Stop,

    /// The pause command is sent when the service is being paused but not stopping.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Pause,

    /// The resume command is sent when the service is being resumed following a pause.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Resume,
}

/// Possible service states including transitional states.
///
/// This can be useful to query the current state with the `state` function or set the state with
/// the `set_state` function.
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

/// The current state the service.
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

/// Sets the current state of the service.
///
/// In most cases, the service state is updated automatically and does not need to be set directly.
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
        SetServiceStatus(HANDLE.get().unwrap().0, &status);
    }
}

/// A service builder, providing control over what commands the service supports before the service begins to run.
#[derive(Default)]
pub struct Service(u32);

impl Service {
    /// Creates a new `Service` object.
    ///
    /// By default, the service does not accept any service commands other than start.
    pub fn new() -> Self {
        Self(0)
    }

    /// The service accepts stop and shutdown commands.
    pub fn can_stop(&mut self) -> &mut Self {
        self.0 |= SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
        self
    }

    /// The service accepts pause and resume commands.
    pub fn can_pause(&mut self) -> &mut Self {
        self.0 |= SERVICE_ACCEPT_PAUSE_CONTINUE;
        self
    }

    /// Runs the service with the given callback closure to receive commands sent by the service
    /// control manager.
    ///
    /// This method will block for the life of the service. It will never return and immediately
    /// terminate the current process after indicating to the service control manager that the
    /// service has stopped.
    pub fn run<F: FnMut(Command) + Send + Sync>(&self, callback: F) -> ! {
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
                r#"Use service control manager to start service.
    
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

#[derive(Debug)]
struct Handle(SERVICE_STATUS_HANDLE);
static HANDLE: OnceLock<Handle> = OnceLock::new();
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

#[derive(Debug)]
struct Callback(*mut (dyn FnMut(Command) + Send + Sync));
static CALLBACK: OnceLock<Callback> = OnceLock::new();
unsafe impl Send for Callback {}
unsafe impl Sync for Callback {}

static STATUS: RwLock<SERVICE_STATUS> = RwLock::new(SERVICE_STATUS {
    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
    dwCurrentState: SERVICE_STOPPED,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
});

fn callback(command: Command) {
    unsafe {
        (*CALLBACK.get().unwrap().0)(command);
    }
}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let handle = unsafe { RegisterServiceCtrlHandlerW(std::ptr::null(), Some(handler)) };

    HANDLE.set(Handle(handle)).unwrap();
    set_state(State::StartPending);
    callback(Command::Start);
    set_state(State::Running);
}

extern "system" fn handler(control: u32) {
    match control {
        SERVICE_CONTROL_CONTINUE if state() == State::Paused => {
            set_state(State::ContinuePending);
            callback(Command::Resume);
            set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if state() == State::Running => {
            set_state(State::PausePending);
            callback(Command::Pause);
            set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            set_state(State::StopPending);
            callback(Command::Stop);
            set_state(State::Stopped);
        }
        _ => {}
    }
}
