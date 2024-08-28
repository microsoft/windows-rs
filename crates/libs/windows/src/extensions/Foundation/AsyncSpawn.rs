use super::Async::Async;
use crate::{core::*, Foundation::*};
use core::mem::transmute;
use std::sync::Mutex;

windows_targets::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(callback: PTP_SIMPLE_CALLBACK, context: *const core::ffi::c_void, environment: *const core::ffi::c_void) -> i32);
type PTP_SIMPLE_CALLBACK = unsafe extern "system" fn(instance: *const core::ffi::c_void, context: *const core::ffi::c_void);

struct State<T: Async> {
    result: Option<Result<T::Output>>,
    completed: Option<T::CompletedHandler>,
    completed_assigned: bool,
}

impl<T: Async> State<T> {
    fn status(&self) -> AsyncStatus {
        match &self.result {
            None => AsyncStatus::Started,
            Some(Ok(_)) => AsyncStatus::Completed,
            Some(Err(_)) => AsyncStatus::Error,
        }
    }

    fn error_code(&self) -> HRESULT {
        match &self.result {
            Some(Err(error)) => error.code(),
            _ => HRESULT(0),
        }
    }

    fn get_results(&self) -> Result<T::Output> {
        match &self.result {
            Some(result) => result.clone(),
            None => Err(Error::from_hresult(HRESULT(0x8000000Eu32 as i32))), // E_ILLEGAL_METHOD_CALL
        }
    }
}

struct SyncState<T: Async>(Mutex<State<T>>);

impl<T: Async> SyncState<T> {
    fn new() -> Self {
        Self(Mutex::new(State { result: None, completed: None, completed_assigned: false }))
    }

    fn status(&self) -> AsyncStatus {
        self.0.lock().unwrap().status()
    }

    fn error_code(&self) -> HRESULT {
        self.0.lock().unwrap().error_code()
    }

    fn get_results(&self) -> Result<T::Output> {
        self.0.lock().unwrap().get_results()
    }

    fn set_completed(&self, sender: &T, handler: Option<&T::CompletedHandler>) -> Result<()> {
        let mut guard = self.0.lock().unwrap();

        if guard.completed_assigned {
            Err(Error::from_hresult(HRESULT(0x80000018u32 as i32))) // E_ILLEGAL_DELEGATE_ASSIGNMENT
        } else {
            guard.completed_assigned = true;

            let status = guard.status();

            if status == AsyncStatus::Started {
                guard.completed = handler.cloned();
            } else {
                drop(guard);
                sender.invoke_completed(&handler.unwrap(), status);
            }

            Ok(())
        }
    }

    fn spawn<F>(&self, sender: &T, f: F)
    where
        F: FnOnce() -> Result<T::Output> + Send + 'static,
    {
        let result = f();
        let mut guard = self.0.lock().unwrap();
        debug_assert!(guard.result.is_none());
        guard.result = Some(result);
        let status = self.status();
        let completed = guard.completed.take();

        drop(guard);

        if let Some(completed) = completed {
            sender.invoke_completed(&completed, status);
        }
    }
}

unsafe impl<T: Async> Send for SyncState<T> {}

#[implement(IAsyncAction, IAsyncInfo)]
struct Action(SyncState<IAsyncAction>);

impl IAsyncInfo_Impl for Action_Impl {
    fn Id(&self) -> Result<u32> {
        Ok(1)
    }
    fn Status(&self) -> Result<AsyncStatus> {
        Ok(self.0.status())
    }
    fn ErrorCode(&self) -> Result<HRESULT> {
        Ok(self.0.error_code())
    }
    fn Cancel(&self) -> Result<()> {
        Ok(())
    }
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

impl IAsyncAction_Impl for Action_Impl {
    fn SetCompleted(&self, handler: Option<&AsyncActionCompletedHandler>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.get_results()
    }
}

impl IAsyncAction {
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let object = ComObject::new(Action(SyncState::new()));
        let interface = object.to_interface();

        threadpool(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

// impl<T: RuntimeType> IAsyncOperation<T> {
//     pub fn spawn(result: Result<T>) -> Self {

//     }
// }

// impl<P: RuntimeType> IAsyncActionWithProgress<P> {
//     pub fn spawn(result: Result<()>) -> Self {

//     }
// }

// impl<T: RuntimeType, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
//     pub fn spawn(result: Result<T>) -> Self {

//     }
// }

fn threadpool<F: FnOnce() + Send + 'static>(f: F) {
    unsafe {
        if TrySubmitThreadpoolCallback(callback::<F>, transmute(Box::new(f)), core::ptr::null()) == 0 {
            panic!("allocation failed");
        }

        unsafe extern "system" fn callback<F: FnOnce() + Send + 'static>(_: *const core::ffi::c_void, callback: *const core::ffi::c_void) {
            Box::from_raw(callback as *mut F)();
        }
    }
}
