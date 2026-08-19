use super::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::app::Reactor;
use crate::bindings::{IApplicationOverrides, IXamlMetadataProvider};

enum Host {
    Reactor {
        _reactor: Rc<RefCell<Reactor<WinUiRuntime>>>,
    },
}

thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
    static RESOURCE_REACTOR: RefCell<Option<std::rc::Weak<RefCell<Reactor<WinUiRuntime>>>>> =
        const { RefCell::new(None) };
}

pub fn run_reactor_winui(title: &str, root: Element) -> WindowsResult<()> {
    run_reactor_winui_with(title, root, || {})
}

pub fn run_reactor_winui_app(root: Element) -> WindowsResult<()> {
    run_reactor_winui_core(root, |_| Ok(()), |_| Ok(()))
}

pub fn run_reactor_winui_with<F>(title: &str, root: Element, on_ready: F) -> WindowsResult<()>
where
    F: FnOnce() + 'static,
{
    run_reactor_winui_core(
        single_window_application(title, root, false),
        |_| Ok(()),
        move |_| {
            on_ready();
            Ok(())
        },
    )
}

#[cfg(test)]
#[path = "../../testing/private/winui/host_support.rs"]
mod testing;

#[cfg(test)]
pub(in crate::winui) use testing::run_reactor_winui_async_fixture;
#[cfg(all(test, feature = "canvas"))]
pub(in crate::winui) use testing::run_reactor_winui_configured_async_fixture;
#[cfg(test)]
pub(crate) use testing::run_reactor_winui_performance;

pub(super) fn run_reactor_winui_core<C, F>(
    root: Element,
    configure: C,
    on_ready: F,
) -> WindowsResult<()>
where
    C: FnOnce(&mut Reactor<WinUiRuntime>) -> WindowsResult<()> + 'static,
    F: FnOnce(&mut Reactor<WinUiRuntime>) -> WindowsResult<()> + 'static,
{
    initialize_ui_thread()?;

    let root = RefCell::new(Some(root));
    let configure = RefCell::new(Some(configure));
    let on_ready = RefCell::new(Some(on_ready));
    let initialization_error: Rc<RefCell<Option<windows_core::Error>>> =
        Rc::new(RefCell::new(None));
    let callback_error = Rc::clone(&initialization_error);
    let start = bindings::Application::Start(&bindings::ApplicationInitializationCallback::new(
        move |_params| {
            let result = (|| -> WindowsResult<()> {
                let root = root.borrow_mut().take().unwrap();
                let configure = configure.borrow_mut().take().unwrap();
                let on_ready = on_ready.borrow_mut().take().unwrap();
                let launched_error = Rc::clone(&callback_error);
                let launched = Box::new(move || -> WindowsResult<()> {
                    let result = (|| -> WindowsResult<()> {
                        let app = bindings::Application::Current()?;
                        install_resources(&app)?;
                        let runtime = WinUiRuntime::new(Rc::new(terminate_host))?;
                        let mut reactor = Reactor::new(runtime, root);
                        configure(&mut reactor)?;
                        let reactor = Rc::new(RefCell::new(reactor));
                        install_reactor_waker(&reactor)?;
                        reactor.borrow_mut().pump();
                        HOST.with(|slot| {
                            *slot.borrow_mut() = Some(Host::Reactor {
                                _reactor: Rc::clone(&reactor),
                            });
                        });
                        on_ready(&mut reactor.borrow_mut())?;
                        reactor.borrow().engine.runtime.request_exit_if_empty();
                        Ok(())
                    })();
                    if let Err(error) = result {
                        launched_error.borrow_mut().get_or_insert(error);
                        terminate_host();
                    }
                    Ok(())
                });
                let app = create_application(launched)?;
                drop(app);
                Ok(())
            })();
            if let Err(error) = result {
                callback_error.borrow_mut().get_or_insert(error);
            }
        },
    ));

    HOST.with(|slot| {
        slot.borrow_mut().take();
    });
    start?;
    if let Some(error) = initialization_error.borrow_mut().take() {
        return Err(error);
    }
    Ok(())
}

fn single_window_application(title: &str, root: Element, fullscreen: bool) -> Element {
    let title = title.to_string();
    let root = Rc::new(RefCell::new(Some(root)));
    component(move |cx| {
        let open = cx.use_state(|| true);
        let windows = if open.get().unwrap() {
            let close = open;
            let window = Window::new(
                title.clone(),
                root.borrow_mut()
                    .take()
                    .expect("single-window content rendered more than once"),
                move || {
                    close.set(false);
                },
            );
            let window = if fullscreen {
                window.presenter(WindowPresenter::FullScreen)
            } else {
                window
            };
            vec![window.build().key(0)]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    })
}

fn install_reactor_waker(reactor: &Rc<RefCell<Reactor<WinUiRuntime>>>) -> WindowsResult<()> {
    let dispatcher = bindings::DispatcherQueue::GetForCurrentThread()?;
    RESOURCE_REACTOR.with(|slot| {
        slot.borrow_mut().replace(Rc::downgrade(reactor));
    });
    let weak = Rc::downgrade(reactor);
    let scheduled = Rc::new(Cell::new(false));
    let render_dispatcher = dispatcher.clone();
    let wake: Rc<dyn Fn()> = Rc::new(move || {
        if scheduled.replace(true) {
            return;
        }
        let weak = weak.clone();
        let handler_scheduled = Rc::clone(&scheduled);
        let handler = bindings::DispatcherQueueHandler::new(move || {
            let Some(reactor) = weak.upgrade() else {
                handler_scheduled.set(false);
                return;
            };
            reactor.borrow_mut().pump();
            handler_scheduled.set(false);
            reactor.borrow().wake_pending_canvas_work();
        });
        assert!(
            render_dispatcher
                .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Normal, &handler)
                .unwrap(),
            "failed to enqueue windows-reactor pump"
        );
    });
    let resource_scheduled = Arc::new(AtomicBool::new(false));
    let resource_wake: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        if resource_scheduled.swap(true, Ordering::AcqRel) {
            return;
        }
        let scheduled = Arc::clone(&resource_scheduled);
        let handler = bindings::DispatcherQueueHandler::new(move || {
            scheduled.store(false, Ordering::Release);
            RESOURCE_REACTOR.with(|slot| {
                if let Some(reactor) = slot.borrow().as_ref().and_then(std::rc::Weak::upgrade) {
                    reactor.borrow_mut().pump();
                    reactor.borrow().wake_pending_canvas_work();
                }
            });
        });
        let queued = dispatcher
            .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Normal, &handler)
            .unwrap();
        if !queued {
            resource_scheduled.store(false, Ordering::Release);
            panic!("failed to enqueue windows-reactor resource pump");
        }
    });
    let mut reactor = reactor.borrow_mut();
    reactor.set_waker(wake);
    reactor.set_resource_waker(resource_wake);
    Ok(())
}

pub(crate) fn terminate_host() {
    RESOURCE_REACTOR.with(|slot| {
        slot.borrow_mut().take();
    });
    HOST.with(|slot| {
        slot.borrow_mut().take();
    });
    _ = bindings::Application::Current().and_then(|application| application.Exit());
}

fn install_resources(app: &bindings::Application) -> WindowsResult<()> {
    let controls = bindings::XamlControlsResources::new()?;
    let dictionary: bindings::ResourceDictionary = controls.cast()?;
    app.Resources()?.MergedDictionaries()?.Append(&dictionary)
}

fn create_application(
    on_launched: Box<dyn FnOnce() -> WindowsResult<()>>,
) -> WindowsResult<bindings::Application> {
    bindings::Application::compose(ApplicationOverrides::new(on_launched))
}

windows_core::implement_decl! {
    impl ApplicationOverrides as pub ApplicationOverrides_Impl:
        [IApplicationOverrides, IXamlMetadataProvider]
}

pub struct ApplicationOverrides {
    provider: RefCell<Option<bindings::XamlControlsXamlMetaDataProvider>>,
    on_launched: RefCell<Option<Box<dyn FnOnce() -> WindowsResult<()>>>>,
}

impl ApplicationOverrides {
    fn new(on_launched: Box<dyn FnOnce() -> WindowsResult<()>>) -> Self {
        Self {
            provider: RefCell::new(None),
            on_launched: RefCell::new(Some(on_launched)),
        }
    }

    fn provider(&self) -> WindowsResult<bindings::XamlControlsXamlMetaDataProvider> {
        if let Some(provider) = self.provider.borrow().as_ref() {
            return Ok(provider.clone());
        }
        let provider = bindings::XamlControlsXamlMetaDataProvider::new()?;
        *self.provider.borrow_mut() = Some(provider.clone());
        Ok(provider)
    }
}

impl bindings::IApplicationOverrides_Impl for ApplicationOverrides_Impl {
    fn OnLaunched(
        &self,
        _args: windows_core::Ref<bindings::LaunchActivatedEventArgs>,
    ) -> WindowsResult<()> {
        if let Some(callback) = self.on_launched.borrow_mut().take() {
            callback()?;
        }
        Ok(())
    }
}

impl bindings::IXamlMetadataProvider_Impl for ApplicationOverrides_Impl {
    fn GetXamlType(&self, value: &bindings::TypeName) -> WindowsResult<bindings::IXamlType> {
        self.provider()?.GetXamlType(value)
    }

    fn GetXamlTypeByFullName(
        &self,
        value: &windows_core::HSTRING,
    ) -> WindowsResult<bindings::IXamlType> {
        self.provider()?
            .GetXamlTypeByFullName(&value.to_string_lossy())
    }

    fn GetXmlnsDefinitions(&self) -> WindowsResult<windows_core::Array<bindings::XmlnsDefinition>> {
        self.provider()?.GetXmlnsDefinitions()
    }
}

fn initialize_ui_thread() -> WindowsResult<()> {
    unsafe {
        // An embedding host may already have selected its process-wide DPI mode.
        _ = bindings::SetProcessDpiAwarenessContext(
            bindings::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        );
    }
    let result = unsafe {
        bindings::CoInitializeEx(std::ptr::null(), bindings::COINIT_APARTMENTTHREADED as u32)
    };
    result.ok()
}
