#![doc = include_str!("../readme.md")]
#![cfg(windows)]

#[expect(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
use bindings::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Mutex, RwLock};

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

    /// An extended command.
    ///
    /// Specific commands will only be received if the `can_accept` methods is called to specify those
    /// commands the service accepts.
    Extended(ExtendedCommand),
}

/// A command not specifically covered by the `Command` enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtendedCommand {
    /// The control code for the command.
    pub control: u32,

    /// The event type, if any.
    pub ty: u32,

    /// The event data, if any.
    pub data: *const c_void,
}

unsafe impl Send for ExtendedCommand {}
unsafe impl Sync for ExtendedCommand {}

/// Possible service states including transitional states.
///
/// This can be useful to query the current state with the `state` method or set the state with
/// the `set_state` method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /// The service continue is pending.
    ContinuePending,
    /// The service is paused.
    Paused,
    /// The service pause is pending.
    PausePending,
    /// The service is running.
    Running,
    /// The service start is pending.
    StartPending,
    /// The service is stopped.
    Stopped,
    /// The service stop is pending.
    StopPending,
}

/// A copyable handle to the running service.
///
/// The handle is passed to the callback for every command. It is `Copy` and `'static`, so the start
/// command can move it into a worker thread to run the service and update its state.
#[derive(Debug, Copy, Clone)]
pub struct ServiceHandle(());

impl ServiceHandle {
    /// The current state of the service.
    pub fn state(&self) -> State {
        from_native(STATUS.read().unwrap().dwCurrentState)
    }

    /// Sets the current state of the service.
    ///
    /// In most cases, the service state is updated automatically and does not need to be set directly.
    pub fn set_state(&self, state: State) {
        // Makes a copy to avoid holding a lock while calling `SetServiceStatus`.
        let status = {
            let mut writer = STATUS.write().unwrap();
            writer.dwCurrentState = to_native(state);
            *writer
        };

        unsafe {
            SetServiceStatus(HANDLE.load(Ordering::Relaxed), &status);
        }
    }

    /// The raw handle representing the service.
    pub fn handle(&self) -> *mut c_void {
        HANDLE.load(Ordering::Relaxed)
    }
}

/// A service builder, providing control over what commands the service supports before the service begins to run.
#[derive(Default)]
pub struct Service {
    accept: u32,
}

impl Service {
    /// Creates a new `Service` object.
    ///
    /// By default, the service does not accept any service commands other than start.
    pub fn new() -> Self {
        Self { accept: 0 }
    }

    /// The service accepts stop and shutdown commands.
    pub fn can_stop(&mut self) -> &mut Self {
        self.accept |= SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN;
        self
    }

    /// The service accepts pause and resume commands.
    pub fn can_pause(&mut self) -> &mut Self {
        self.accept |= SERVICE_ACCEPT_PAUSE_CONTINUE;
        self
    }

    /// The service accepts other specified commands.
    pub fn can_accept(&mut self, accept: u32) -> &mut Self {
        self.accept |= accept;
        self
    }

    /// Runs the service with the given callback closure to receive commands sent by the service
    /// control manager.
    ///
    /// This method blocks for the life of the service and never returns. The process is terminated
    /// when the service stops. If the process was not started by the service control manager,
    /// instructions for installing and controlling the service are printed before exiting.
    ///
    /// The callback should return promptly. To perform ongoing work, the start command can spawn a
    /// worker thread, moving the `Copy` [`ServiceHandle`] into it to run the service and update its
    /// state.
    pub fn run<F: FnMut(ServiceHandle, Command) + Send + 'static>(&self, callback: F) -> ! {
        STATUS.write().unwrap().dwControlsAccepted = self.accept;
        *CALLBACK.lock().unwrap() = Some(Box::new(callback));

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        // The control handler terminates the process when the service stops, so the dispatcher only
        // returns when the process was not started by the service control manager.
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

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let handle =
        unsafe { RegisterServiceCtrlHandlerExW(std::ptr::null(), Some(handler), std::ptr::null()) };

    HANDLE.store(handle, Ordering::Relaxed);

    let service = ServiceHandle(());
    service.set_state(State::StartPending);
    command(service, Command::Start);
    service.set_state(State::Running);

    // Keep the service alive; the control handler terminates the process when the service stops.
    loop {
        std::thread::park();
    }
}

extern "system" fn handler(control: u32, ty: u32, data: *mut c_void, _context: *mut c_void) -> u32 {
    let service = ServiceHandle(());

    match control {
        SERVICE_CONTROL_CONTINUE if service.state() == State::Paused => {
            service.set_state(State::ContinuePending);
            command(service, Command::Resume);
            service.set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if service.state() == State::Running => {
            service.set_state(State::PausePending);
            command(service, Command::Pause);
            service.set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            service.set_state(State::StopPending);
            command(service, Command::Stop);
            service.set_state(State::Stopped);
            std::process::exit(0);
        }
        _ => command(
            service,
            Command::Extended(ExtendedCommand { control, ty, data }),
        ),
    }

    NO_ERROR
}

fn command(service: ServiceHandle, command: Command) {
    if let Some(callback) = CALLBACK.lock().unwrap().as_deref_mut() {
        callback(service, command);
    }
}

fn to_native(state: State) -> SERVICE_STATUS_CURRENT_STATE {
    match state {
        State::ContinuePending => SERVICE_CONTINUE_PENDING,
        State::Paused => SERVICE_PAUSED,
        State::PausePending => SERVICE_PAUSE_PENDING,
        State::Running => SERVICE_RUNNING,
        State::StartPending => SERVICE_START_PENDING,
        State::Stopped => SERVICE_STOPPED,
        State::StopPending => SERVICE_STOP_PENDING,
    }
}

fn from_native(state: SERVICE_STATUS_CURRENT_STATE) -> State {
    match state {
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

// A process hosts a single service, so the live service state is shared globally.
// `StartServiceCtrlDispatcherW` has no user-defined context, so these globals are how the service
// callbacks reach the user's callback.
static HANDLE: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static CALLBACK: Mutex<Option<Box<dyn FnMut(ServiceHandle, Command) + Send>>> = Mutex::new(None);
static STATUS: RwLock<SERVICE_STATUS> = RwLock::new(SERVICE_STATUS {
    dwServiceType: SERVICE_WIN32_OWN_PROCESS,
    dwCurrentState: SERVICE_STOPPED,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
});
