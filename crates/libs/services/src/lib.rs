#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![expect(
    non_camel_case_types,
    non_snake_case,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms,
    clippy::type_complexity
)]

mod bindings;
use bindings::*;
use std::boxed::Box;
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock, RwLock};

/// The commands are sent by the service control manager to the [`Service`] through the closure or callback
/// passed to the [`ServiceBuilder::run`] method.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    /// The start command is sent when the service first starts.
    Start,

    /// The stop command is sent when the service is stopping just prior to process termination.
    ///
    /// This command will only be sent if the [`ServiceBuilder::can_stop`] method is called as part of construction.
    Stop,

    /// The pause command is sent when the service is being paused but not stopping.
    ///
    /// This command will only be sent if the [`ServiceBuilder::can_pause`] method is called as part of construction.
    Pause,

    /// The resume command is sent when the service is being resumed following a pause.
    ///
    /// This command will only be sent if the [`ServiceBuilder::can_pause`] method is called as part of construction.
    Resume,

    /// An extended command.
    ///
    /// Specific commands will only be received if the [`ServiceBuilder::can_accept`] methods is called to specify those
    /// commands the service accepts.
    Extended(ExtendedCommand),
}

/// A command not specifically covered by the [`Command`] enum.
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
/// This can be useful to query the current state with the [`Service::state`] function or set the state with
/// the [`Service::set_state`] function.
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

/// A struct that holds service api specific state and methods. This is a singleton.
pub struct Service {
    handle: RwLock<SERVICE_STATUS_HANDLE>,
    status: RwLock<SERVICE_STATUS>,
    callback: Mutex<Option<Box<dyn FnMut(&'static Service, Command) + Send + Sync + 'static>>>,
}

unsafe impl Send for Service {}
unsafe impl Sync for Service {}

impl Service {
    fn new() -> Self {
        Self {
            handle: RwLock::new(std::ptr::null_mut()),
            callback: Mutex::new(None),
            status: RwLock::new(SERVICE_STATUS {
                dwServiceType: SERVICE_WIN32_OWN_PROCESS,
                dwCurrentState: SERVICE_STOPPED,
                dwControlsAccepted: 0,
                dwWin32ExitCode: 0,
                dwServiceSpecificExitCode: 0,
                dwCheckPoint: 0,
                dwWaitHint: 0,
            }),
        }
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
            SetServiceStatus(self.handle(), &status);
        }
    }

    /// The raw handle representing the service.
    pub fn handle(&self) -> *mut core::ffi::c_void {
        *self.handle.read().unwrap()
    }

    /// The current state the service.
    pub fn state(&self) -> State {
        let reader = self.status.read().unwrap();

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

    /// Sends the command to the service callback.
    pub fn command(&'static self, command: Command) {
        let mut lock = self.callback.lock().unwrap();
        (lock.as_deref_mut().unwrap())(self, command);
    }

    /// Low-level dispatcher to send control commands directly to the service.
    pub fn handler(&self, control: u32, event_type: u32, event_data: *const c_void) -> u32 {
        handler(
            control,
            event_type,
            event_data as *mut _,
            self as *const _ as _,
        )
    }
}

/// [`ServiceBuilder::run`] errors.
#[derive(Debug)]
pub enum Error {
    /// The [`Service`] Singleton was already constructed and is currently running.
    Running,

    /// The application was invoked as a console application and thus cannot continue as a service.
    NotAService,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Running => f.write_str("Another ServiceBuilder has already been run()."),
            Error::NotAService => f.write_str("Service was invoked as a console application."),
        }
    }
}

impl std::error::Error for Error {}

/// Builder that is used to configure and run the [`Service`] global.
#[derive(Default)]
pub struct ServiceBuilder {
    accept: u32,
}

impl ServiceBuilder {
    /// Builder for a global [`Service`]
    ///
    /// By default, the service does not accept any service commands other than start.
    pub fn new() -> Self {
        Self::default()
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

    /// Creates a [`Service`] singleton and starts it with the given callback closure.
    /// The closure will receive commands sent by the service control manager.
    ///
    /// This method will block for the life of the service. It will never return and immediately
    /// terminate the current process after indicating to the service control manager that the
    /// service has stopped.
    ///
    /// This call will fail if it was called by another instance of a builder or if this is a
    /// console application.
    pub fn run<F: FnMut(&'static Service, Command) + Send + Sync + 'static>(
        &mut self,
        callback: F,
    ) -> Result<(), Error> {
        if SERVICE_CONTEXT.get().is_some() {
            return Err(Error::Running);
        }
        let ctx = SERVICE_CONTEXT.get_or_init(Service::new);
        ctx.status.write().unwrap().dwControlsAccepted = self.accept;
        *ctx.callback.lock().unwrap() = Some(Box::new(callback));

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        if unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) != 0 } {
            Err(Error::NotAService)
        } else {
            Ok(())
        }
    }
}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let service: &Service = SERVICE_CONTEXT.get().unwrap();

    *service.handle.write().unwrap() = unsafe {
        RegisterServiceCtrlHandlerExW(std::ptr::null(), Some(handler), service as *const _ as _)
    };

    service.set_state(State::StartPending);
    service.command(Command::Start);
    service.set_state(State::Running);
}

extern "system" fn handler(control: u32, ty: u32, data: *mut c_void, context: *mut c_void) -> u32 {
    let service = unsafe { &*(context as *const Service) };

    match control {
        SERVICE_CONTROL_CONTINUE if service.state() == State::Paused => {
            service.set_state(State::ContinuePending);
            service.command(Command::Resume);
            service.set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if service.state() == State::Running => {
            service.set_state(State::PausePending);
            service.command(Command::Pause);
            service.set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            service.set_state(State::StopPending);
            service.command(Command::Stop);
            service.set_state(State::Stopped);
        }
        _ => service.command(Command::Extended(ExtendedCommand { control, ty, data })),
    }

    NO_ERROR
}

static SERVICE_CONTEXT: OnceLock<Service> = OnceLock::new();
