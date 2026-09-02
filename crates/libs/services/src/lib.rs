#![doc = include_str!("../readme.md")]
#![cfg(windows)]

#[expect(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]
mod bindings;
use bindings::*;
use std::boxed::Box;
use std::ffi::c_void;
use std::sync::RwLock;

/// A command sent by the service control manager.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Command {
    /// Starts the service.
    Start,

    /// Stops the service before process termination.
    ///
    /// This command will only be sent if the `can_stop` method is called as part of construction.
    Stop,

    /// Pauses the service.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Pause,

    /// Resumes the service.
    ///
    /// This command will only be sent if the `can_pause` method is called as part of construction.
    Resume,

    /// An extended command.
    ///
    /// The service receives only controls enabled with [`Service::can_accept`].
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

/// A service state, including transitional states.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /// Resume is pending.
    ContinuePending,
    /// The service is paused.
    Paused,
    /// Pause is pending.
    PausePending,
    /// The service is running.
    Running,
    /// Start is pending.
    StartPending,
    /// The service is stopped.
    Stopped,
    /// Stop is pending.
    StopPending,
}

/// Configures and runs a Windows service.
pub struct Service<'a> {
    accept: u32,
    fallback: Option<Box<dyn FnOnce(&Service) + 'a>>,
    handle: RwLock<SERVICE_STATUS_HANDLE>,
    callback: RwLock<Option<Box<dyn FnMut(&Service, Command) + Send + Sync + 'a>>>,
    status: RwLock<SERVICE_STATUS>,
}

impl Default for Service<'_> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Service<'_> {}
unsafe impl Sync for Service<'_> {}

impl<'a> Service<'a> {
    /// Creates a service that accepts only the start command.
    pub fn new() -> Self {
        Self {
            accept: 0,
            fallback: None,
            handle: RwLock::new(std::ptr::null_mut()),
            callback: RwLock::new(None),
            status: RwLock::new(SERVICE_STATUS {
                dwServiceType: SERVICE_WIN32_OWN_PROCESS as u32,
                dwCurrentState: SERVICE_STOPPED as u32,
                dwControlsAccepted: 0,
                dwWin32ExitCode: 0,
                dwServiceSpecificExitCode: 0,
                dwCheckPoint: 0,
                dwWaitHint: 0,
            }),
        }
    }

    /// The service accepts stop and shutdown commands.
    pub fn can_stop(&mut self) -> &mut Self {
        self.accept |= (SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN) as u32;
        self
    }

    /// The service accepts pause and resume commands.
    pub fn can_pause(&mut self) -> &mut Self {
        self.accept |= SERVICE_ACCEPT_PAUSE_CONTINUE as u32;
        self
    }

    /// The service accepts other specified commands.
    pub fn can_accept(&mut self, accept: u32) -> &mut Self {
        self.accept |= accept;
        self
    }

    /// Runs the fallback closure if the service is not started by the Service Control Manager.
    pub fn can_fallback<F: FnOnce(&Service) + Send + 'a>(&mut self, f: F) -> &mut Self {
        self.fallback = Some(Box::new(f));
        self
    }

    /// Runs the service with the given callback closure to receive commands sent by the service
    /// control manager.
    ///
    /// This method will block for the life of the service. It will never return and immediately
    /// terminate the current process after indicating to the service control manager that the
    /// service has stopped.
    pub fn run<F: FnMut(&Service, Command) + Send + Sync + 'a>(
        &mut self,
        callback: F,
    ) -> Result<(), &'static str> {
        debug_assert!(self.status.read().unwrap().dwCurrentState == SERVICE_STOPPED as u32);
        self.status.write().unwrap().dwControlsAccepted = self.accept;

        {
            let mut write = self.callback.write().unwrap();

            assert!(!write.is_some(), "`run` was already called");

            *write = Some(Box::new(callback));
        }

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        SERVICE_CONTEXT.write().unwrap().0 = self as *const _ as _;

        let fallback = unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) == 0 };

        if fallback {
            if let Some(fallback) = self.fallback.take() {
                self.set_state(State::StartPending);
                self.command(Command::Start);
                self.set_state(State::Running);
                fallback(self);
                self.set_state(State::StopPending);
                self.command(Command::Stop);
            } else {
                return Err("Use service control manager to start service");
            }
        }

        Ok(())
    }

    /// Sets the current state of the service.
    ///
    /// In most cases, the service state is updated automatically and does not need to be set
    /// directly.
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
        } as u32;

        // Makes a copy to avoid holding a lock while calling `SetServiceStatus`.
        let status: SERVICE_STATUS = *writer;
        drop(writer);

        unsafe {
            SetServiceStatus(self.handle(), &status);
        }
    }

    /// The raw handle representing the service.
    pub fn handle(&self) -> *mut c_void {
        *self.handle.read().unwrap()
    }

    /// The current state the service.
    pub fn state(&self) -> State {
        let reader = self.status.read().unwrap();

        match reader.dwCurrentState as i32 {
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
    pub fn command(&self, command: Command) {
        let mut write = self.callback.write().unwrap();
        (write.as_deref_mut().unwrap())(self, command);
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

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    let service: &Service = unsafe { &*(SERVICE_CONTEXT.read().unwrap().0 as *const Service) };

    *service.handle.write().unwrap() = unsafe {
        RegisterServiceCtrlHandlerExW(std::ptr::null(), Some(handler), service as *const _ as _)
    };

    service.set_state(State::StartPending);
    service.command(Command::Start);
    service.set_state(State::Running);
}

extern "system" fn handler(control: u32, ty: u32, data: *mut c_void, context: *mut c_void) -> u32 {
    let service = unsafe { &*(context as *const Service) };

    match control as i32 {
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

    NO_ERROR as u32
}

// The dispatcher callback has no user-defined context parameter.
// This lock allows us to safely retrieve the service instance from the service callback.
#[derive(Debug)]
struct ServiceContext(*const c_void);
static SERVICE_CONTEXT: RwLock<ServiceContext> = RwLock::new(ServiceContext(std::ptr::null()));
unsafe impl Send for ServiceContext {}
unsafe impl Sync for ServiceContext {}
