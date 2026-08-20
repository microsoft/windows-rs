use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

thread_local! {
    static HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
}

struct LiveHost {
    fault: Option<windows_core::Error>,
    pump: Box<dyn LivePump>,
}

trait LivePump {
    fn mount(&mut self) -> Result<(), PumpError>;
    fn dispatch_events(&mut self) -> Result<(), PumpError>;
    fn shutdown(&mut self);
}

impl<F> LivePump for RenderLoop<WinUiRuntime, F>
where
    F: FnMut(&mut Hooks) -> Element,
{
    fn mount(&mut self) -> Result<(), PumpError> {
        self.run()
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.dispatch_events().map(|_| ())
    }

    fn shutdown(&mut self) {
        self.pump_mut().shutdown();
    }
}

pub fn bootstrap() -> windows_core::Result<()> {
    bootstrap_runtime()
}

pub struct App;

impl App {
    pub fn run<F>(root: F) -> windows_core::Result<()>
    where
        F: FnMut(&mut Hooks) -> Element + 'static,
    {
        initialize_ui_thread()?;
        let root = RefCell::new(Some(root));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);

        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let mounted = (|| {
                let root = root.borrow_mut().take().unwrap();
                let mut pump: Box<dyn LivePump> =
                    Box::new(RenderLoop::new(WinUiRuntime::default(), root));
                let property_fault = match pump.mount() {
                    Ok(()) => None,
                    Err(error) if error.recoverable() => Some(error),
                    Err(error) => return Err(pump_error(error)),
                };
                HOST.with(|host| {
                    *host.borrow_mut() = Some(LiveHost { fault: None, pump });
                });
                if let Some(error) = property_fault {
                    eprintln!("windows-reactor-next fault: {}", pump_error(error));
                }
                Ok(())
            })();
            if let Err(error) = mounted {
                *callback_result.borrow_mut() = Err(error);
                exit_ui_thread();
            }
        }));

        let callback_result = std::mem::replace(&mut *result.borrow_mut(), Ok(()));
        let host = HOST.with(|host| host.borrow_mut().take());
        let host_result = host.and_then(|host| host.fault).map_or(Ok(()), Err);
        start.and(callback_result).and(host_result)
    }
}

pub(crate) fn dispatch_native_events() {
    HOST.with(|host| {
        let Some(mut live) = host.borrow_mut().take() else {
            return;
        };
        if live.fault.is_none()
            && let Err(error) = live.pump.dispatch_events()
        {
            let recoverable = error.recoverable();
            let error = pump_error(error);
            eprintln!("windows-reactor-next fault: {error}");
            if !recoverable {
                live.fault = Some(error);
                live.pump.shutdown();
                exit_ui_thread();
            }
        }
        *host.borrow_mut() = Some(live);
    });
}

fn pump_error(error: PumpError) -> windows_core::Error {
    windows_core::Error::new(E_FAIL, format!("{error:?}"))
}
