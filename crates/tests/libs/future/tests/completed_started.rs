// `spawn` may receive `Completed` while still in the `Started` state.
// and must hold on to the handler and call it when execution completes.

use std::sync::mpsc::channel;
use std::thread;
use windows_core::*;
use windows_future::*;

#[cfg(windows)]
#[expect(non_snake_case)]
mod apartment {
    use super::*;
    use std::{
        ffi::c_void,
        os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle},
        sync::{Arc, Mutex},
        thread::ThreadId,
    };
    use test_future::*;

    fn isolated(test: &str) -> bool {
        const CHILD: &str = "WINDOWS_FUTURE_APARTMENT_CASE";
        if std::env::var(CHILD).as_deref() == Ok(test) {
            return true;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", test, "--nocapture"])
            .env(CHILD, test)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "stdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        false
    }

    struct Com;

    impl Com {
        fn new(mode: u32) -> Result<Self> {
            unsafe { CoInitializeEx(None, mode).ok()? };
            Ok(Self)
        }
    }

    impl Drop for Com {
        fn drop(&mut self) {
            unsafe { CoUninitialize() };
        }
    }

    struct Observation {
        calls: Mutex<Vec<(ThreadId, AsyncStatus)>>,
        reentrant: Mutex<Vec<(HRESULT, HRESULT)>>,
        dropped: Mutex<Option<ThreadId>>,
        done: OwnedHandle,
    }

    impl Observation {
        fn new() -> Result<Arc<Self>> {
            let handle = unsafe { CreateEventW(None, true, false, None) };
            if handle.is_null() {
                return Err(Error::from_thread());
            }
            Ok(Arc::new(Self {
                calls: Mutex::new(Vec::new()),
                reentrant: Mutex::new(Vec::new()),
                dropped: Mutex::new(None),
                done: unsafe { OwnedHandle::from_raw_handle(handle) },
            }))
        }

        fn wait(&self) -> Result<()> {
            let mut handle = self.done.as_raw_handle();
            let result = unsafe {
                CoWaitForMultipleHandles(
                    COWAIT_DISPATCH_CALLS as u32,
                    5_000,
                    1,
                    &mut handle,
                    &mut 0,
                )
                .ok()
            };
            if result.is_err() {
                eprintln!(
                    "wait failed: calls={:?}, dropped={:?}",
                    self.calls.lock().unwrap(),
                    self.dropped.lock().unwrap(),
                );
            }
            result
        }

        fn assert_owner(&self, owner: ThreadId) {
            assert_eq!(
                (
                    self.calls.lock().unwrap().clone(),
                    *self.dropped.lock().unwrap()
                ),
                (vec![(owner, AsyncStatus::Completed)], Some(owner)),
            );
            assert_eq!(
                *self.reentrant.lock().unwrap(),
                [(HRESULT(0), HRESULT(0x80000018u32 as i32))]
            );
        }
    }

    // Use the system proxy for AsyncActionCompletedHandler.
    #[interface("a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7")]
    unsafe trait IApartmentCompletion: IUnknown {
        unsafe fn Invoke(&self, sender: *mut c_void, status: AsyncStatus) -> HRESULT;
    }

    #[implement(IApartmentCompletion, Agile = false)]
    struct Completion(Arc<Observation>);

    impl IApartmentCompletion_Impl for Completion_Impl {
        unsafe fn Invoke(&self, sender: *mut c_void, status: AsyncStatus) -> HRESULT {
            self.0
                .calls
                .lock()
                .unwrap()
                .push((thread::current().id(), status));
            let sender = unsafe { IAsyncAction::from_raw_borrowed(&sender) }.unwrap();
            let results = sender
                .GetResults()
                .map_or_else(|error| error.code(), |_| HRESULT(0));
            let assignment = sender
                .SetCompleted(None)
                .map_or_else(|error| error.code(), |_| HRESULT(0));
            self.0.reentrant.lock().unwrap().push((results, assignment));
            HRESULT(0)
        }
    }

    impl Drop for Completion {
        fn drop(&mut self) {
            *self.0.dropped.lock().unwrap() = Some(thread::current().id());
            assert!(unsafe { SetEvent(self.0.done.as_raw_handle()) }.as_bool());
        }
    }

    fn handler(observation: &Arc<Observation>) -> Result<AsyncActionCompletedHandler> {
        let handler: IApartmentCompletion = Completion(observation.clone()).into();
        assert!(handler.cast::<IAgileObject>().is_err());
        assert!(handler.cast::<IMarshal>().is_err());
        handler.cast()
    }

    #[implement(IApartmentCompletion, INoMarshal, Agile = false)]
    struct NonMarshaled;

    impl INoMarshal_Impl for NonMarshaled_Impl {}

    impl IApartmentCompletion_Impl for NonMarshaled_Impl {
        unsafe fn Invoke(&self, _: *mut c_void, _: AsyncStatus) -> HRESULT {
            HRESULT(0)
        }
    }

    #[test]
    fn agile_registration_does_not_initialize_com() -> Result<()> {
        if !isolated("apartment::agile_registration_does_not_initialize_com") {
            return Ok(());
        }
        let (start, started) = channel();
        let (sent, received) = channel();
        let operation = IAsyncAction::spawn(move || {
            started
                .recv()
                .map_err(|_| Error::from_hresult(HRESULT(0x80004004u32 as i32)))?;
            Ok(())
        });
        operation.SetCompleted(&AsyncActionCompletedHandler::new(move |_, status| {
            sent.send(status).unwrap();
            Ok(())
        }))?;
        assert_eq!(
            unsafe { CoGetContextToken(&mut 0) },
            HRESULT(0x800401F0u32 as i32)
        );
        start.send(()).unwrap();
        assert_eq!(
            received
                .recv_timeout(std::time::Duration::from_secs(5))
                .unwrap(),
            AsyncStatus::Completed
        );
        operation.GetResults()
    }

    #[test]
    fn non_agile_registration_does_not_initialize_mta() -> Result<()> {
        if !isolated("apartment::non_agile_registration_does_not_initialize_mta") {
            return Ok(());
        }
        thread::spawn(|| {
            let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
            let observation = Observation::new()?;
            let handler = handler(&observation)?;
            let (start, started) = channel();
            let (sent, received) = channel();
            let operation = IAsyncAction::spawn(move || {
                started.recv().unwrap();
                sent.send(unsafe { CoGetContextToken(&mut 0) }).unwrap();
                Ok(())
            });
            operation.SetCompleted(&handler)?;
            start.send(()).unwrap();
            assert_eq!(
                received
                    .recv_timeout(std::time::Duration::from_secs(5))
                    .unwrap(),
                HRESULT(0x800401F0u32 as i32)
            );
            Ok(())
        })
        .join()
        .unwrap()
    }

    #[test]
    fn marshaling_failure_does_not_assign_completion() -> Result<()> {
        init_mta()?;
        thread::spawn(|| {
            let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
            let owner = thread::current().id();
            let blocked: IApartmentCompletion = NonMarshaled.into();
            let blocked: AsyncActionCompletedHandler = blocked.cast()?;
            let error = AgileReference::new(&blocked).unwrap_err().code();
            let (start, started) = channel();
            let operation = IAsyncAction::spawn(move || {
                started.recv().unwrap();
                Ok(())
            });
            assert_eq!(operation.SetCompleted(&blocked).unwrap_err().code(), error);
            let observation = Observation::new()?;
            let handler = handler(&observation)?;
            operation.SetCompleted(&handler)?;
            drop(handler);
            start.send(()).unwrap();
            observation.wait()?;
            observation.assert_owner(owner);
            Ok(())
        })
        .join()
        .unwrap()
    }

    #[test]
    fn spawned_non_agile_handler_stays_in_apartment() -> Result<()> {
        if !isolated("apartment::spawned_non_agile_handler_stays_in_apartment") {
            return Ok(());
        }
        init_mta()?;
        thread::spawn(|| {
            let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
            let owner = thread::current().id();
            let observation = Observation::new()?;
            let handler = handler(&observation)?;
            let (start, started) = channel();
            let operation = IAsyncAction::spawn(move || {
                started.recv().unwrap();
                Ok(())
            });
            operation.SetCompleted(&handler)?;
            drop(handler);
            start.send(()).unwrap();
            observation.wait()?;
            observation.assert_owner(owner);
            operation.GetResults()
        })
        .join()
        .unwrap()
    }

    #[test]
    fn agile_reference_control_stays_in_apartment() -> Result<()> {
        thread::spawn(|| {
            let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
            let owner = thread::current().id();
            let observation = Observation::new()?;
            let handler = handler(&observation)?;
            let reference = AgileReference::new(&handler)?;
            drop(handler);
            let operation = IAsyncAction::ready(Ok(()));
            operation.SetCompleted(&AsyncActionCompletedHandler::new(|_, _| Ok(())))?;
            let worker = thread::spawn(move || {
                let _com = Com::new(COINIT_MULTITHREADED as u32)?;
                let resolved = reference.resolve()?;
                resolved.Invoke(&operation, AsyncStatus::Completed)?;
                drop(resolved);
                drop(reference);
                Ok::<_, Error>(())
            });
            observation.wait()?;
            worker.join().unwrap()?;
            observation.assert_owner(owner);
            Ok(())
        })
        .join()
        .unwrap()
    }

    #[test]
    fn uninitialized_pool_rejects_proxy_invocation() -> Result<()> {
        if !isolated("apartment::uninitialized_pool_rejects_proxy_invocation") {
            return Ok(());
        }
        assert_eq!(
            pool_control().unwrap_err().code(),
            HRESULT(0x800401F0u32 as i32)
        );
        Ok(())
    }

    fn pool_control() -> Result<()> {
        thread::spawn(|| {
            let _com = Com::new(COINIT_APARTMENTTHREADED as u32)?;
            let owner = thread::current().id();
            let observation = Observation::new()?;
            let handler = handler(&observation)?;
            let reference = AgileReference::new(&handler)?;
            drop(handler);
            let operation = IAsyncAction::ready(Ok(()));
            operation.SetCompleted(&AsyncActionCompletedHandler::new(|_, _| Ok(())))?;
            let (sent, received) = channel();
            windows_threading::submit(move || {
                let (resolved, result) = match reference.resolve() {
                    Ok(handler) => (true, handler.Invoke(&operation, AsyncStatus::Completed)),
                    Err(error) => (false, Err(error)),
                };
                drop(reference);
                sent.send((resolved, result)).unwrap();
            });
            let waited = observation.wait();
            let (resolved, result) = received
                .recv_timeout(std::time::Duration::from_secs(5))
                .unwrap();
            assert!(resolved, "proxy resolution failed: {result:?}");
            if result.is_err() {
                assert!(observation.calls.lock().unwrap().is_empty());
            }
            result?;
            waited?;
            observation.assert_owner(owner);
            Ok(())
        })
        .join()
        .unwrap()
    }

    #[test]
    fn agile_reference_pool_with_initialized_mta() -> Result<()> {
        init_mta()?;
        pool_control()
    }
}

#[test]
fn action() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncAction::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        let completed_thread = thread::current().id();
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        completed_send.send(completed_thread).unwrap();
        Ok(())
    }))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncOperation::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn action_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}
