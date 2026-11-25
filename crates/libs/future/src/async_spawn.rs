use super::*;
use std::sync::{Mutex, OnceLock};

struct State<T: Async> {
    completed: Option<imp::AgileDelegate<T::CompletedHandler>>,
    completed_assigned: bool,
}

struct SyncState<T: Async> {
    result: OnceLock<AsyncResult<T::Output>>,
    state: Mutex<State<T>>,
}

impl<T: Async> SyncState<T>
where
    T::Output: Send,
{
    fn new() -> Self {
        Self {
            result: OnceLock::new(),
            state: Mutex::new(State {
                completed: None,
                completed_assigned: false,
            }),
        }
    }

    fn status(&self) -> AsyncStatus {
        self.result
            .get()
            .map_or(AsyncStatus::Started, AsyncResult::status)
    }

    fn error_code(&self) -> HRESULT {
        self.result
            .get()
            .map_or(HRESULT(0), AsyncResult::error_code)
    }

    fn get_results(&self) -> Result<T::Output> {
        self.result()?.take()
    }

    fn check_results(&self) -> Result<()> {
        self.result()?.check()
    }

    fn result(&self) -> Result<&AsyncResult<T::Output>> {
        self.result
            .get()
            .ok_or_else(|| Error::from_hresult(HRESULT(0x8000000Eu32 as i32)))
    }

    fn set_completed(&self, sender: &T, handler: Ref<T::CompletedHandler>) -> Result<()> {
        {
            let mut guard = self.state.lock().unwrap();
            if guard.completed_assigned {
                return Err(Error::from_hresult(HRESULT(0x80000018u32 as i32)));
            }
            let status = self.status();
            let handler = handler.ok()?;
            if status != AsyncStatus::Started {
                guard.completed_assigned = true;
                drop(guard);
                sender.invoke_completed(handler, status);
                return Ok(());
            }
        }

        let handler = handler.ok()?;
        let reference = imp::AgileDelegate::new(handler)?;
        let mut guard = self.state.lock().unwrap();
        if guard.completed_assigned {
            return Err(Error::from_hresult(HRESULT(0x80000018u32 as i32)));
        }
        guard.completed_assigned = true;
        let status = self.status();
        if status == AsyncStatus::Started {
            guard.completed = Some(reference);
        } else {
            drop(guard);
            sender.invoke_completed(handler, status);
        }
        Ok(())
    }

    fn spawn<F>(&self, sender: &T, f: F)
    where
        F: FnOnce() -> Result<T::Output> + Send + 'static,
    {
        let result = AsyncResult::new(f());
        let mut guard = self.state.lock().unwrap();
        assert!(self.result.set(result).is_ok());
        let status = self.status();
        let completed = guard.completed.take();

        drop(guard);

        if let Some(completed) = completed {
            _ = completed.call(|handler| {
                sender.invoke_completed(handler, status);
                Ok(())
            });
        }
    }
}

struct Action(SyncState<IAsyncAction>);

implement_decl! {
    impl Action as Action_Impl: [IAsyncAction, IAsyncInfo]
}

struct Operation<T>(SyncState<IAsyncOperation<T>>)
where
    T: RuntimeType + Send + 'static;

implement_decl! {
    impl<T> Operation as Operation_Impl: [
        IAsyncOperation<T>,
        IAsyncInfo,
    ]
    where T: RuntimeType + Send + 'static
}

struct ActionWithProgress<P>(SyncState<IAsyncActionWithProgress<P>>)
where
    P: RuntimeType + 'static;

implement_decl! {
    impl<P> ActionWithProgress as ActionWithProgress_Impl: [
        IAsyncActionWithProgress<P>,
        IAsyncInfo,
    ]
    where P: RuntimeType + 'static
}

struct OperationWithProgress<T, P>(SyncState<IAsyncOperationWithProgress<T, P>>)
where
    T: RuntimeType + Send + 'static,
    P: RuntimeType + 'static;

implement_decl! {
    impl<T, P> OperationWithProgress as OperationWithProgress_Impl: [
        IAsyncOperationWithProgress<T, P>,
        IAsyncInfo,
    ]
    where T: RuntimeType + Send + 'static, P: RuntimeType + 'static
}

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

impl<T: RuntimeType + Send> IAsyncInfo_Impl for Operation_Impl<T> {
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

impl<P: RuntimeType> IAsyncInfo_Impl for ActionWithProgress_Impl<P> {
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

impl<T: RuntimeType + Send, P: RuntimeType> IAsyncInfo_Impl for OperationWithProgress_Impl<T, P> {
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
    fn SetCompleted(&self, handler: Ref<AsyncActionCompletedHandler>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionCompletedHandler> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.check_results()
    }
}

impl<T: RuntimeType + Send> IAsyncOperation_Impl<T> for Operation_Impl<T> {
    fn SetCompleted(&self, handler: Ref<AsyncOperationCompletedHandler<T>>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationCompletedHandler<T>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.get_results()
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress_Impl<P> for ActionWithProgress_Impl<P> {
    fn SetCompleted(&self, handler: Ref<AsyncActionWithProgressCompletedHandler<P>>) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncActionWithProgressCompletedHandler<P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<()> {
        self.0.check_results()
    }
    fn SetProgress(&self, _: Ref<AsyncActionProgressHandler<P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncActionProgressHandler<P>> {
        Err(Error::empty())
    }
}

impl<T: RuntimeType + Send, P: RuntimeType> IAsyncOperationWithProgress_Impl<T, P>
    for OperationWithProgress_Impl<T, P>
{
    fn SetCompleted(
        &self,
        handler: Ref<AsyncOperationWithProgressCompletedHandler<T, P>>,
    ) -> Result<()> {
        self.0.set_completed(&self.as_interface(), handler)
    }
    fn Completed(&self) -> Result<AsyncOperationWithProgressCompletedHandler<T, P>> {
        Err(Error::empty())
    }
    fn GetResults(&self) -> Result<T> {
        self.0.get_results()
    }
    fn SetProgress(&self, _: Ref<AsyncOperationProgressHandler<T, P>>) -> Result<()> {
        Ok(())
    }
    fn Progress(&self) -> Result<AsyncOperationProgressHandler<T, P>> {
        Err(Error::empty())
    }
}

impl IAsyncAction {
    /// Creates an `IAsyncAction` that waits for the closure to execute on the Windows thread pool.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let object = ComObject::new(Action(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<T: RuntimeType + Send> IAsyncOperation<T> {
    /// Creates an `IAsyncOperation<T>` that runs the closure on the Windows thread pool.
    ///
    /// The result is transferred once from the worker to the caller of `GetResults`.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let object = ComObject::new(Operation(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<P: RuntimeType> IAsyncActionWithProgress<P> {
    /// Runs the closure on the Windows thread pool with progress reporting.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        let object = ComObject::new(ActionWithProgress(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}

impl<T: RuntimeType + Send, P: RuntimeType> IAsyncOperationWithProgress<T, P> {
    /// Runs the value-producing closure on the Windows thread pool with progress reporting.
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T> + Send + 'static,
    {
        let object = ComObject::new(OperationWithProgress(SyncState::new()));
        let interface = object.to_interface();

        windows_threading::submit(move || {
            object.0.spawn(&object.as_interface(), f);
        });

        interface
    }
}
