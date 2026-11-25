use super::*;
use crate::owner::OwnerThread;
use std::cell::{Cell, RefCell};

#[implement(
    ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler,
    Agile = false
)]
pub(crate) struct EnvironmentCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<Environment>)>>>>,
);

impl EnvironmentCompleted {
    pub(crate) fn create<F: FnOnce(Result<Environment>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<Environment>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler_Impl for EnvironmentCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2Environment>) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode
                .ok()
                .and_then(|()| Ok(Environment(result.ok()?.clone())));
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

#[implement(
    ICoreWebView2CreateCoreWebView2ControllerCompletedHandler,
    Agile = false
)]
pub(crate) struct ControllerCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<Controller>)>>>>,
);

impl ControllerCompleted {
    pub(crate) fn create<F: FnOnce(Result<Controller>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<Controller>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2CreateCoreWebView2ControllerCompletedHandler_Impl for ControllerCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2Controller>) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode
                .ok()
                .and_then(|()| Ok(Controller(result.ok()?.clone())));
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2ExecuteScriptCompletedHandler, Agile = false)]
pub(crate) struct ExecuteScriptCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<String>)>>>>,
);

impl ExecuteScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2ExecuteScriptCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<String>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2ExecuteScriptCompletedHandler_Impl for ExecuteScriptCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: &PCWSTR) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode
                .ok()
                .map(|()| unsafe { result.to_string().unwrap_or_default() });
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

#[implement(
    ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler,
    Agile = false
)]
pub(crate) struct AddScriptCompleted(OwnerThread<Cell<Option<Box<dyn FnOnce(Result<String>)>>>>);

impl AddScriptCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<String>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2AddScriptToExecuteOnDocumentCreatedCompletedHandler_Impl
    for AddScriptCompleted_Impl
{
    fn Invoke(&self, errorcode: HRESULT, id: &PCWSTR) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode
                .ok()
                .map(|()| unsafe { id.to_string().unwrap_or_default() });
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2GetCookiesCompletedHandler, Agile = false)]
pub(crate) struct GetCookiesCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<Vec<Cookie>>)>>>>,
);

impl GetCookiesCompleted {
    pub(crate) fn create<F: FnOnce(Result<Vec<Cookie>>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2GetCookiesCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<Vec<Cookie>>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2GetCookiesCompletedHandler_Impl for GetCookiesCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT, result: Ref<ICoreWebView2CookieList>) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode.ok().and_then(|()| cookie::collect(result.ok()?));
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2ClearBrowsingDataCompletedHandler, Agile = false)]
pub(crate) struct ClearBrowsingDataCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<()>)>>>>,
);

impl ClearBrowsingDataCompleted {
    pub(crate) fn create<F: FnOnce(Result<()>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2ClearBrowsingDataCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<()>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2ClearBrowsingDataCompletedHandler_Impl for ClearBrowsingDataCompleted_Impl {
    fn Invoke(&self, errorcode: HRESULT) -> HRESULT {
        self.0.invoke(|callback| {
            if let Some(handler) = callback.take() {
                handler(errorcode.ok());
            }
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2CallDevToolsProtocolMethodCompletedHandler, Agile = false)]
pub(crate) struct CallDevToolsProtocolMethodCompleted(
    OwnerThread<Cell<Option<Box<dyn FnOnce(Result<String>)>>>>,
);

impl CallDevToolsProtocolMethodCompleted {
    pub(crate) fn create<F: FnOnce(Result<String>) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2CallDevToolsProtocolMethodCompletedHandler> {
        let handler: Box<dyn FnOnce(Result<String>)> = Box::new(handler);
        Ok(Self(OwnerThread::new(Cell::new(Some(handler)))?).into())
    }
}

impl ICoreWebView2CallDevToolsProtocolMethodCompletedHandler_Impl
    for CallDevToolsProtocolMethodCompleted_Impl
{
    fn Invoke(&self, errorcode: HRESULT, result: &PCWSTR) -> HRESULT {
        self.0.invoke(|callback| {
            let outcome = errorcode
                .ok()
                .map(|()| unsafe { result.to_string().unwrap_or_default() });
            if let Some(handler) = callback.take() {
                handler(outcome);
            }
            HRESULT(0)
        })
    }
}

/// Defines adapters from WebView2 event COM interfaces to Rust closures.
macro_rules! event_handler {
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, $args:ident => $wrapper:ident) => {
        event_handler!($name / $impl, $handler / $handler_trait, ICoreWebView2, $args => $wrapper);
    };
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, $sender:ident, $args:ident => $wrapper:ident) => {
        #[implement($handler, Agile = false)]
        pub(crate) struct $name(OwnerThread<RefCell<Box<dyn FnMut($wrapper)>>>);

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> Result<$handler> {
                let handler: Box<dyn FnMut($wrapper)> = Box::new(handler);
                Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
            }
        }

        impl $handler_trait for $impl {
            fn Invoke(&self, _sender: Ref<$sender>, args: Ref<$args>) -> HRESULT {
                self.0.invoke(|callback| {
                    args.ok().map(|args| {
                        let args = $wrapper(args.clone());
                        (*callback.borrow_mut())(args);
                    }).into()
                })
            }
        }
    };
    ($name:ident / $impl:ident, $handler:ident / $handler_trait:ident, sender $sender:ident => $wrapper:ident) => {
        #[implement($handler, Agile = false)]
        pub(crate) struct $name(OwnerThread<RefCell<Box<dyn FnMut($wrapper)>>>);

        impl $name {
            pub(crate) fn create<F: FnMut($wrapper) + 'static>(handler: F) -> Result<$handler> {
                let handler: Box<dyn FnMut($wrapper)> = Box::new(handler);
                Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
            }
        }

        impl $handler_trait for $impl {
            fn Invoke(&self, sender: Ref<$sender>, _args: Ref<IUnknown>) -> HRESULT {
                self.0.invoke(|callback| {
                    sender.ok().map(|sender| {
                        let value = $wrapper(sender.clone());
                        (*callback.borrow_mut())(value);
                    }).into()
                })
            }
        }
    };
}

event_handler!(NavigationCompleted / NavigationCompleted_Impl, ICoreWebView2NavigationCompletedEventHandler / ICoreWebView2NavigationCompletedEventHandler_Impl, ICoreWebView2NavigationCompletedEventArgs => NavigationCompletedArgs);
event_handler!(NavigationStarting / NavigationStarting_Impl, ICoreWebView2NavigationStartingEventHandler / ICoreWebView2NavigationStartingEventHandler_Impl, ICoreWebView2NavigationStartingEventArgs => NavigationStartingArgs);
event_handler!(WebMessageReceived / WebMessageReceived_Impl, ICoreWebView2WebMessageReceivedEventHandler / ICoreWebView2WebMessageReceivedEventHandler_Impl, ICoreWebView2WebMessageReceivedEventArgs => WebMessageReceivedArgs);
event_handler!(ContentLoading / ContentLoading_Impl, ICoreWebView2ContentLoadingEventHandler / ICoreWebView2ContentLoadingEventHandler_Impl, ICoreWebView2ContentLoadingEventArgs => ContentLoadingArgs);
event_handler!(NewWindowRequested / NewWindowRequested_Impl, ICoreWebView2NewWindowRequestedEventHandler / ICoreWebView2NewWindowRequestedEventHandler_Impl, ICoreWebView2NewWindowRequestedEventArgs => NewWindowRequestedArgs);
event_handler!(PermissionRequested / PermissionRequested_Impl, ICoreWebView2PermissionRequestedEventHandler / ICoreWebView2PermissionRequestedEventHandler_Impl, ICoreWebView2PermissionRequestedEventArgs => PermissionRequestedArgs);
event_handler!(DownloadStarting / DownloadStarting_Impl, ICoreWebView2DownloadStartingEventHandler / ICoreWebView2DownloadStartingEventHandler_Impl, ICoreWebView2DownloadStartingEventArgs => DownloadStartingArgs);
event_handler!(DownloadStateChanged / DownloadStateChanged_Impl, ICoreWebView2StateChangedEventHandler / ICoreWebView2StateChangedEventHandler_Impl, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(BytesReceivedChanged / BytesReceivedChanged_Impl, ICoreWebView2BytesReceivedChangedEventHandler / ICoreWebView2BytesReceivedChangedEventHandler_Impl, sender ICoreWebView2DownloadOperation => DownloadOperation);
event_handler!(ProcessFailed / ProcessFailed_Impl, ICoreWebView2ProcessFailedEventHandler / ICoreWebView2ProcessFailedEventHandler_Impl, ICoreWebView2ProcessFailedEventArgs => ProcessFailedArgs);
event_handler!(DevToolsProtocolEventReceived / DevToolsProtocolEventReceived_Impl, ICoreWebView2DevToolsProtocolEventReceivedEventHandler / ICoreWebView2DevToolsProtocolEventReceivedEventHandler_Impl, ICoreWebView2DevToolsProtocolEventReceivedEventArgs => DevToolsProtocolEventReceivedArgs);
event_handler!(MoveFocusRequested / MoveFocusRequested_Impl, ICoreWebView2MoveFocusRequestedEventHandler / ICoreWebView2MoveFocusRequestedEventHandler_Impl, ICoreWebView2Controller, ICoreWebView2MoveFocusRequestedEventArgs => MoveFocusRequestedArgs);
event_handler!(AcceleratorKeyPressed / AcceleratorKeyPressed_Impl, ICoreWebView2AcceleratorKeyPressedEventHandler / ICoreWebView2AcceleratorKeyPressedEventHandler_Impl, ICoreWebView2Controller, ICoreWebView2AcceleratorKeyPressedEventArgs => AcceleratorKeyPressedArgs);

#[implement(ICoreWebView2FocusChangedEventHandler, Agile = false)]
pub(crate) struct FocusChanged(OwnerThread<RefCell<Box<dyn FnMut()>>>);

impl FocusChanged {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2FocusChangedEventHandler> {
        let handler: Box<dyn FnMut()> = Box::new(handler);
        Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
    }
}

impl ICoreWebView2FocusChangedEventHandler_Impl for FocusChanged_Impl {
    fn Invoke(&self, _sender: Ref<ICoreWebView2Controller>, _args: Ref<IUnknown>) -> HRESULT {
        self.0.invoke(|callback| {
            (*callback.borrow_mut())();
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2DocumentTitleChangedEventHandler, Agile = false)]
pub(crate) struct DocumentTitleChanged(OwnerThread<RefCell<Box<dyn FnMut(String)>>>);

impl DocumentTitleChanged {
    pub(crate) fn create<F: FnMut(String) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2DocumentTitleChangedEventHandler> {
        let handler: Box<dyn FnMut(String)> = Box::new(handler);
        Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
    }
}

impl ICoreWebView2DocumentTitleChangedEventHandler_Impl for DocumentTitleChanged_Impl {
    fn Invoke(&self, sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> HRESULT {
        self.0.invoke(|callback| {
            let title = unsafe {
                string::take_result(sender.ok().and_then(|sender| sender.DocumentTitle()))
            };
            (*callback.borrow_mut())(title);
            HRESULT(0)
        })
    }
}

#[implement(
    ICoreWebView2ContainsFullScreenElementChangedEventHandler,
    Agile = false
)]
pub(crate) struct ContainsFullScreenElementChanged(OwnerThread<RefCell<Box<dyn FnMut(bool)>>>);

impl ContainsFullScreenElementChanged {
    pub(crate) fn create<F: FnMut(bool) + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2ContainsFullScreenElementChangedEventHandler> {
        let handler: Box<dyn FnMut(bool)> = Box::new(handler);
        Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
    }
}

impl ICoreWebView2ContainsFullScreenElementChangedEventHandler_Impl
    for ContainsFullScreenElementChanged_Impl
{
    fn Invoke(&self, sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> HRESULT {
        self.0.invoke(|callback| {
            let contains = sender
                .ok()
                .and_then(|sender| unsafe { sender.ContainsFullScreenElement() })
                .is_ok_and(|value| value.as_bool());
            (*callback.borrow_mut())(contains);
            HRESULT(0)
        })
    }
}

#[implement(ICoreWebView2WindowCloseRequestedEventHandler, Agile = false)]
pub(crate) struct WindowCloseRequested(OwnerThread<RefCell<Box<dyn FnMut()>>>);

impl WindowCloseRequested {
    pub(crate) fn create<F: FnMut() + 'static>(
        handler: F,
    ) -> Result<ICoreWebView2WindowCloseRequestedEventHandler> {
        let handler: Box<dyn FnMut()> = Box::new(handler);
        Ok(Self(OwnerThread::new(RefCell::new(handler))?).into())
    }
}

impl ICoreWebView2WindowCloseRequestedEventHandler_Impl for WindowCloseRequested_Impl {
    fn Invoke(&self, _sender: Ref<ICoreWebView2>, _args: Ref<IUnknown>) -> HRESULT {
        self.0.invoke(|callback| {
            (*callback.borrow_mut())();
            HRESULT(0)
        })
    }
}

/// Defines a subscription method that returns an RAII removal guard.
macro_rules! subscription {
    (
        $(#[$doc:meta])*
        $method:ident($arg:ty) => $handler:ident, $add:ident / $remove:ident
    ) => {
        $(#[$doc])*
        pub fn $method<F: FnMut($arg) + 'static>(&self, handler: F) -> Result<EventRegistration> {
            let handler = crate::handler::$handler::create(handler)?;
            let mut token = Default::default();
            unsafe { self.0.$add(&handler, &mut token).ok()? };
            let source = self.0.clone();
            Ok(EventRegistration::new(move || {
                let _ = unsafe { source.$remove(token) };
            }))
        }
    };
}

pub(crate) use subscription;

#[cfg(all(test, windows, feature = "system", not(miri)))]
mod tests {
    use super::*;
    use std::{
        panic::{AssertUnwindSafe, catch_unwind},
        rc::Rc,
        sync::{Arc, Mutex, mpsc},
        thread::{self, ThreadId},
        time::{Duration, Instant},
    };

    struct Capture {
        drops: Arc<Mutex<Vec<ThreadId>>>,
        _local: Rc<()>,
    }

    impl Capture {
        fn invoke(&self) {}
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            self.drops.lock().unwrap().push(thread::current().id());
        }
    }

    fn release_from_another_apartment(create: impl FnOnce(Capture, &StaApartment) -> IUnknown) {
        let apartment = init_sta().unwrap();
        let owner = thread::current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let callback = create(
            Capture {
                drops: drops.clone(),
                _local: Rc::new(()),
            },
            &apartment,
        );
        let reference = AgileReference::new(&callback).unwrap();
        drop(callback);
        let (send, receive) = mpsc::channel();
        let worker = thread::spawn(move || {
            let _apartment = init_sta().unwrap();
            let callback = reference.resolve().unwrap();
            drop(reference);
            drop(callback);
            send.send(()).unwrap();
        });
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            assert!(
                Instant::now() < deadline,
                "cross-apartment release timed out"
            );
            assert!(windows_window::pump());
            match receive.try_recv() {
                Ok(()) => break,
                Err(mpsc::TryRecvError::Empty) => thread::yield_now(),
                Err(error) => panic!("{error}"),
            }
        }
        worker.join().unwrap();
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }

    #[test]
    fn callback_capture_release_stays_on_owner() {
        release_from_another_apartment(|capture, _| {
            FocusChanged::create(move || capture.invoke())
                .unwrap()
                .cast()
                .unwrap()
        });
    }

    #[test]
    fn pending_completion_capture_release_stays_on_owner() {
        release_from_another_apartment(|capture, _| {
            ExecuteScriptCompleted::create(move |_| capture.invoke())
                .unwrap()
                .cast()
                .unwrap()
        });
    }

    #[test]
    fn callback_creation_requires_sta() {
        thread::spawn(|| {
            let _: HRESULT = Error::new(E_ABORT, "pending creation error").into();
            let error = FocusChanged::create(|| panic!("unexpected callback")).unwrap_err();
            assert!([imp::CO_E_NOTINITIALIZED, RPC_E_WRONG_THREAD].contains(&error.code()));
            assert_eq!(Error::from(E_ABORT).message(), "pending creation error");
            unsafe {
                imp::CoInitializeEx(std::ptr::null_mut(), 0).ok().unwrap();
            }
            let error =
                EnvironmentCompleted::create(|_| panic!("unexpected callback")).unwrap_err();
            assert_eq!(error.code(), RPC_E_WRONG_THREAD);
            unsafe { imp::CoUninitialize() };
            let _apartment = init_sta().unwrap();
            FocusChanged::create(|| {}).unwrap();
        })
        .join()
        .unwrap();
    }

    #[test]
    fn foreign_com_calls_fail_fast() {
        const CHILD: &str = "WINDOWS_WEBVIEW_OWNER_FAILURE";
        if let Ok(operation) = std::env::var(CHILD) {
            let _apartment = init_sta().unwrap();
            let callback = FocusChanged::create(|| panic!("unexpected callback")).unwrap();
            let object = callback.cast_object::<FocusChanged>().unwrap();
            drop(callback);
            match operation.as_str() {
                "invoke" => thread::scope(|scope| {
                    let _ = scope
                        .spawn(|| unsafe {
                            let callback =
                                object.as_interface::<ICoreWebView2FocusChangedEventHandler>();
                            (callback.vtable().Invoke)(
                                callback.as_raw(),
                                std::ptr::null_mut(),
                                std::ptr::null_mut(),
                            )
                        })
                        .join()
                        .unwrap();
                }),
                "drop" => thread::spawn(move || {
                    let callback: ICoreWebView2FocusChangedEventHandler = object.into_interface();
                    drop(callback);
                })
                .join()
                .unwrap(),
                _ => panic!("unknown operation"),
            }
            return;
        }

        for (operation, message) in [
            (
                "invoke",
                "WebView callback accessed outside its owner thread",
            ),
            ("drop", "WebView callback dropped outside its owner thread"),
        ] {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "handler::tests::foreign_com_calls_fail_fast",
                    "--nocapture",
                ])
                .env(CHILD, operation)
                .env("RUST_BACKTRACE", "0")
                .output()
                .unwrap();
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(!output.status.success(), "{operation}: {stderr}");
            assert!(stderr.contains(message), "{operation}: {stderr}");
            assert!(
                stderr.contains("panic in a function that cannot unwind"),
                "{operation}: {stderr}"
            );
        }
    }

    #[test]
    fn foreign_completion_preserves_capture_and_error_info() {
        let _apartment = init_sta().unwrap();
        let values = Rc::new(RefCell::new(Vec::new()));
        let sink = values.clone();
        let callback = ExecuteScriptCompleted::create(move |result| {
            sink.borrow_mut().push(result.unwrap());
        })
        .unwrap();
        let object = callback
            .cast_object_ref::<ExecuteScriptCompleted>()
            .unwrap();
        thread::scope(|scope| {
            scope
                .spawn(|| {
                    let _apartment = init_sta().unwrap();
                    let _: HRESULT = Error::new(E_ABORT, "pending native error").into();
                    assert!(
                        catch_unwind(AssertUnwindSafe(|| {
                            object.Invoke(E_ABORT, &PCWSTR::null())
                        }))
                        .is_err()
                    );
                    assert_eq!(Error::from(E_ABORT).message(), "pending native error");
                })
                .join()
                .unwrap();
        });
        assert!(values.borrow().is_empty());
        unsafe {
            (callback.vtable().Invoke)(callback.as_raw(), HRESULT(0), w!("first"))
                .ok()
                .unwrap();
            (callback.vtable().Invoke)(callback.as_raw(), HRESULT(0), w!("second"))
                .ok()
                .unwrap();
        }
        assert_eq!(*values.borrow(), ["first"]);
    }

    #[test]
    fn foreign_events_assert_before_reading_arguments() {
        let _apartment = init_sta().unwrap();
        let navigation = NavigationCompleted::create(|_| panic!("unexpected callback")).unwrap();
        let download = DownloadStateChanged::create(|_| panic!("unexpected callback")).unwrap();
        let title = DocumentTitleChanged::create(|_| panic!("unexpected callback")).unwrap();
        let navigation = navigation.cast_object_ref::<NavigationCompleted>().unwrap();
        let download = download.cast_object_ref::<DownloadStateChanged>().unwrap();
        let title = title.cast_object_ref::<DocumentTitleChanged>().unwrap();
        thread::scope(|scope| {
            scope
                .spawn(|| {
                    assert!(
                        catch_unwind(AssertUnwindSafe(|| {
                            navigation.Invoke(Default::default(), Default::default())
                        }))
                        .is_err()
                    );
                    assert!(
                        catch_unwind(AssertUnwindSafe(|| {
                            download.Invoke(Default::default(), Default::default())
                        }))
                        .is_err()
                    );
                    assert!(
                        catch_unwind(AssertUnwindSafe(|| {
                            title.Invoke(Default::default(), Default::default())
                        }))
                        .is_err()
                    );
                })
                .join()
                .unwrap();
        });
    }

    #[test]
    fn completion_is_taken_before_reentrant_invocation() {
        let _apartment = init_sta().unwrap();
        let slot = Rc::new(RefCell::new(
            None::<ICoreWebView2ExecuteScriptCompletedHandler>,
        ));
        let calls = Rc::new(Cell::new(0));
        let sink = calls.clone();
        let reentrant = slot.clone();
        let callback = ExecuteScriptCompleted::create(move |result| {
            assert_eq!(result.unwrap(), "outer");
            sink.set(sink.get() + 1);
            unsafe {
                let callback = reentrant.borrow();
                let callback = callback.as_ref().unwrap();
                (callback.vtable().Invoke)(callback.as_raw(), HRESULT(0), w!("inner"))
                    .ok()
                    .unwrap();
            }
        })
        .unwrap();
        *slot.borrow_mut() = Some(callback.clone());
        unsafe {
            (callback.vtable().Invoke)(callback.as_raw(), HRESULT(0), w!("outer"))
                .ok()
                .unwrap();
        };
        assert_eq!(calls.get(), 1);
        slot.borrow_mut().take();
    }

    #[test]
    fn callback_capture_is_released_at_apartment_shutdown() {
        let apartment = init_sta().unwrap();
        let owner = thread::current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let capture = Capture {
            drops: drops.clone(),
            _local: Rc::new(()),
        };
        let callback = FocusChanged::create(move || capture.invoke()).unwrap();
        let unknown = callback.cast::<IUnknown>().unwrap();
        let reference = AgileReference::new(&unknown).unwrap();
        drop(unknown);
        drop(callback);
        assert!(drops.lock().unwrap().is_empty());
        drop(apartment);
        assert_eq!(*drops.lock().unwrap(), [owner]);
        thread::spawn(move || {
            let _apartment = init_sta().unwrap();
            let result = reference.resolve();
            drop(result);
            drop(reference);
        })
        .join()
        .unwrap();
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }

    #[test]
    fn callback_shutdown_disconnects_a_live_proxy() {
        let apartment = init_sta().unwrap();
        let owner = thread::current().id();
        let drops = Arc::new(Mutex::new(Vec::new()));
        let capture = Capture {
            drops: drops.clone(),
            _local: Rc::new(()),
        };
        let callback = FocusChanged::create(move || capture.invoke()).unwrap();
        let unknown = callback.cast::<IUnknown>().unwrap();
        let reference = AgileReference::new(&unknown).unwrap();
        drop(unknown);
        drop(callback);
        let (ready, receive_ready) = mpsc::channel();
        let (release, receive_release) = mpsc::channel();
        let worker = thread::spawn(move || {
            let _apartment = init_sta().unwrap();
            let callback = reference.resolve().unwrap();
            ready.send(()).unwrap();
            receive_release
                .recv_timeout(Duration::from_secs(5))
                .unwrap();
            drop(callback);
            drop(reference);
        });
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            assert!(
                Instant::now() < deadline,
                "cross-apartment resolve timed out"
            );
            assert!(windows_window::pump());
            match receive_ready.try_recv() {
                Ok(()) => break,
                Err(mpsc::TryRecvError::Empty) => thread::yield_now(),
                Err(error) => panic!("{error}"),
            }
        }
        assert!(drops.lock().unwrap().is_empty());
        drop(apartment);
        assert_eq!(*drops.lock().unwrap(), [owner]);
        release.send(()).unwrap();
        worker.join().unwrap();
        assert_eq!(*drops.lock().unwrap(), [owner]);
    }
}
