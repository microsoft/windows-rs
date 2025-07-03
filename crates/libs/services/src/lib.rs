#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(
    non_camel_case_types,
    non_snake_case,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]

mod bindings;
use bindings::*;
use std::boxed::Box;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock, RwLock};

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

/// A service builder, providing control over what commands the service supports before the service begins to run.
pub struct Service<'a> {
    accept: u32,
    fallback: Option<Box<dyn FnOnce() + 'a>>,
    handle: OnceLock<SERVICE_STATUS_HANDLE>,
    callback: OnceLock<*mut (dyn FnMut(&Service, Command) + Send + Sync)>,
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
    /// Creates a new `Service` object.
    ///
    /// By default, the service does not accept any service commands other than start.
    pub fn new() -> Self {
        Self {
            accept: 0,
            fallback: None,
            handle: OnceLock::new(),
            callback: OnceLock::new(),
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
    pub fn run<F: FnMut(&Service, Command) + Send + Sync>(&mut self, callback: F) {
        debug_assert!(self.status.read().unwrap().dwCurrentState == SERVICE_STOPPED);
        self.status.write().unwrap().dwControlsAccepted = self.accept;

        self.callback
            .set(Box::into_raw(Box::new(callback)) as *mut _)
            .unwrap();

        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: &mut 0,
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW::default(),
        ];

        SERVICES.lock().unwrap().0.push_back(self as *const _ as _);

        let fallback = unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) == 0 };

        if fallback {
            if let Some(fallback) = self.fallback.take() {
                service_main(0, std::ptr::null_mut());
                fallback();
                self.set_state(State::StopPending);
                self.callback(Command::Stop);
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

        std::process::exit(0);
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
            SetServiceStatus(*self.handle.get().unwrap(), &status);
        }
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

    fn callback(&self, command: Command) {
        unsafe {
            (**self.callback.get().unwrap())(self, command);
        }
    }
}

#[derive(Debug)]
struct ServiceDeque(VecDeque<*const c_void>);
static SERVICES: Mutex<ServiceDeque> = Mutex::new(ServiceDeque(VecDeque::new()));
unsafe impl Send for ServiceDeque {}
unsafe impl Sync for ServiceDeque {}

extern "system" fn service_main(_len: u32, _args: *mut PWSTR) {
    unsafe {
        let service: &Service =
            &*(SERVICES.lock().unwrap().0.pop_front().unwrap() as *const Service);

        let handle = RegisterServiceCtrlHandlerExW(
            std::ptr::null(),
            Some(handler),
            service as *const _ as _,
        );

        service.handle.set(handle).unwrap();

        service.set_state(State::StartPending);
        service.callback(Command::Start);
        service.set_state(State::Running);
    }
}

extern "system" fn handler(
    control: u32,
    _event_type: u32,
    _event_data: *mut c_void,
    context: *mut c_void,
) -> u32 {
    let service = unsafe { &*(context as *const Service) };

    match control {
        SERVICE_CONTROL_CONTINUE if service.state() == State::Paused => {
            service.set_state(State::ContinuePending);
            service.callback(Command::Resume);
            service.set_state(State::Running);
        }
        SERVICE_CONTROL_PAUSE if service.state() == State::Running => {
            service.set_state(State::PausePending);
            service.callback(Command::Pause);
            service.set_state(State::Paused);
        }
        SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => {
            service.set_state(State::StopPending);
            service.callback(Command::Stop);
            service.set_state(State::Stopped);
        }
        _ => {}
    }

    NO_ERROR
}
