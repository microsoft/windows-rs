use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

thread_local! {
    static HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
}

struct LiveHost {
    _application: Application,
    _window: Window,
    fault: Option<windows_core::Error>,
    pump: Box<dyn LivePump>,
}

trait LivePump {
    fn mount(&mut self) -> windows_core::Result<UIElement>;
    fn dispatch_events(&mut self) -> Result<(), PumpError>;
}

impl<F> LivePump for RenderLoop<WinUiRuntime, F>
where
    F: FnMut(&mut Hooks) -> Element,
{
    fn mount(&mut self) -> windows_core::Result<UIElement> {
        self.run().map_err(pump_error)?;
        let root = self.pump().root().unwrap();
        self.pump().runtime().ui_element(root)
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.dispatch_events().map(|_| ())
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
                let application = Application::new()?;
                let window = Window::new()?;
                let root = root.borrow_mut().take().unwrap();
                let mut pump: Box<dyn LivePump> =
                    Box::new(RenderLoop::new(WinUiRuntime::default(), root));
                window.SetContent(&pump.mount()?)?;
                window.Activate()?;
                HOST.with(|host| {
                    *host.borrow_mut() = Some(LiveHost {
                        _application: application,
                        _window: window,
                        fault: None,
                        pump,
                    });
                });
                Ok(())
            })();
            if let Err(error) = mounted {
                *callback_result.borrow_mut() = Err(error);
            }
        }));

        let callback_result = std::mem::replace(&mut *result.borrow_mut(), Ok(()));
        let host_result = HOST.with(|host| {
            host.borrow_mut()
                .take()
                .and_then(|host| host.fault)
                .map_or(Ok(()), Err)
        });
        start.and(callback_result).and(host_result)
    }
}

pub(crate) fn dispatch_native_events() {
    HOST.with(|host| {
        if let Some(host) = host.borrow_mut().as_mut()
            && host.fault.is_none()
            && let Err(error) = host.pump.dispatch_events()
        {
            let recoverable = matches!(error, PumpError::PropertyApplyFailed(_));
            let error = pump_error(error);
            eprintln!("windows-reactor-next fault: {error}");
            if !recoverable {
                host.fault = Some(error);
            }
        }
    });
}

fn pump_error(error: PumpError) -> windows_core::Error {
    windows_core::Error::new(E_FAIL, format!("{error:?}"))
}
