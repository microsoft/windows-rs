#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(clippy::needless_doctest_main)]

mod bindings;
use bindings::*;
use std::boxed::Box;
use std::sync::{ RwLock};

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
        SetServiceStatus(HANDLE.read().unwrap().0, &status);
    }
}

/// A service builder, providing control over what commands the service supports before the service begins to run.
#[derive(Default)]
pub struct Service<'a> {
    commands: u32,
    fallback: Option<Box<dyn FnOnce() + 'a>>,
    handle: Option<Handle>,
    callback: Option<Callback>,
    status: RwLock<SERVICE_STATUS>,
}

impl<'a> Service<'a> {
    /// Creates a new `Service` object.
    ///
    /// By default, the service does not accept any service commands other than start.
    pub fn new() -> Self {
        Self::default()
    }

    /// The service accepts stop and shutdown commands.
    pub fn can_stop(&mut self) -> &mut Self {
        self.commands |= SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
        self
    }

    /// The service accepts pause and resume commands.
    pub fn can_pause(&mut self) -> &mut Self {
        self.commands |= SERVICE_ACCEPT_PAUSE_CONTINUE;
        self
    }

    pub fn can_fallback<F: FnOnce() + Send + 'a>(&mut self, f: F) -> &mut Self {
        self.fallback = Some(Box::new(f));
        self
    }

    /// Runs the service with the given callback closure to receive commands sent by the service
    /// control manager.
    ///
    /// This method will block for the life of the service. It will never return and immediately
    /// terminate the current process after indicating to the service control manager that the
    /// service has stopped.
    pub fn run<F: FnMut(Command) + Send + Sync>(&mut self, f: F) {
        debug_assert!(RUN.read().unwrap().is_none());
        debug_assert!(HANDLE.read().unwrap().0.is_null());
        debug_assert!(CALLBACK.read().unwrap().is_none());
        debug_assert!(STATUS.read().unwrap().dwCurrentState == SERVICE_STOPPED);

        STATUS.write().unwrap().dwControlsAccepted = self.commands;
        *CALLBACK
            .write().unwrap() = Some(Callback(Box::into_raw(Box::new(f)) as *mut _));

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        *RUN.write().unwrap() = Some(Run(self as *mut _ as *mut _));
        let fallback = unsafe { StartServiceCtrlDispatcherW(table.as_ptr())  == 0 };
        *RUN.write().unwrap() = None;

        if fallback {
            if let Some(fallback) = self.fallback.take() {
                service_main(0, std::ptr::null_mut());
                fallback();
                set_state(State::StopPending);
                callback(Command::Stop);
            } else {
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
        }

        HANDLE.write().unwrap().0 = std::ptr::null_mut();
        *CALLBACK.write().unwrap() = None;
        STATUS.write().unwrap().dwCurrentState = SERVICE_STOPPED;        
    }    

    /// Sets the current state of the service.
///
/// In most cases, the service state is updated automatically and does not need to be set directly.
    pub fn set_state(&self, state: State) {
        let mut writer = self.status.write().unwrap();
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
            SetServiceStatus(self.handle.read().unwrap().0, &status);
        }
    }
}

#[derive(Debug)]
struct Run(*mut std::ffi::c_void);
static RUN: RwLock<Option<Run>> = RwLock::new(None);
unsafe impl Send for Run {}
unsafe impl Sync for Run {}

#[derive(Debug)]
struct Handle(SERVICE_STATUS_HANDLE);
static HANDLE: RwLock<Handle> = RwLock::new(Handle(std::ptr::null_mut()));
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

#[derive(Debug)]
struct Callback(*mut (dyn FnMut(Command) + Send + Sync));
static CALLBACK: RwLock<Option<Callback>> = RwLock::new(None);
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
        (*CALLBACK.read().unwrap().as_ref().unwrap().0)(command);
    }
}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let handle = unsafe { RegisterServiceCtrlHandlerExW(std::ptr::null(), Some(handler), std::ptr::null()) };

    *HANDLE.write().unwrap() = Handle(handle);
    set_state(State::StartPending);
    callback(Command::Start);
    set_state(State::Running);
}

pub extern "system" fn handler(control: u32, _event_type: u32, _event_data: *mut std::ffi::c_void, _context: *mut std::ffi::c_void) -> u32 {
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
    0 
}
