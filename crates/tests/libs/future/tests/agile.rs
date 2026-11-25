use std::cell::Cell;
use std::future::IntoFuture;
use std::sync::{Arc, Barrier, mpsc};
use windows_core::*;
use windows_future::*;

fn send<T: Send>(_: Option<T>) {}
fn sync<T: Sync>(_: Option<T>) {}

#[test]
fn test() {
    send(Option::<IAsyncAction>::None);
    sync(Option::<IAsyncAction>::None);

    send(Option::<IAsyncActionWithProgress<i32>>::None);
    sync(Option::<IAsyncActionWithProgress<i32>>::None);

    send(Option::<IAsyncOperation<i32>>::None);
    sync(Option::<IAsyncOperation<i32>>::None);

    send(Option::<IAsyncOperationWithProgress<i32, i32>>::None);
    sync(Option::<IAsyncOperationWithProgress<i32, i32>>::None);
}

#[repr(C)]
#[derive(Debug, Default)]
struct Value {
    value: i32,
    clones: Cell<u32>,
}

impl Value {
    fn new(value: i32) -> Self {
        Self {
            value,
            clones: Cell::new(0),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        self.clones.set(self.clones.get() + 1);
        Self {
            value: self.value,
            clones: Cell::new(self.clones.get()),
        }
    }
}

impl imp::TypeKind for Value {
    type TypeKind = imp::CloneType;
}

impl RuntimeType for Value {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Test.Value;i4;u4)");
}

#[test]
fn results_transfer_once_without_cloning() -> Result<()> {
    macro_rules! check {
        ($operation:expr) => {{
            let operation = $operation;
            let value = operation.join()?;
            assert_eq!(value.value, 42);
            assert_eq!(value.clones.get(), 0);
            assert_eq!(
                operation.GetResults().unwrap_err().code(),
                HRESULT(0x8000000E_u32 as _),
            );
            assert_eq!(operation.Status()?, AsyncStatus::Completed);
            assert_eq!(operation.cast::<IAsyncInfo>()?.ErrorCode()?, HRESULT(0));
        }};
    }

    check!(IAsyncOperation::ready(Ok(Value::new(42))));
    check!(IAsyncOperation::spawn(|| Ok(Value::new(42))));
    check!(IAsyncOperationWithProgress::<Value, IInspectable>::ready(
        Ok(Value::new(42)),
    ));
    check!(IAsyncOperationWithProgress::<Value, IInspectable>::spawn(
        || Ok(Value::new(42)),
    ));
    Ok(())
}

#[test]
fn concurrent_result_transfer_has_one_winner() -> Result<()> {
    macro_rules! check {
        ($operation:expr, $handler:ident) => {{
            let operation = $operation;
            let (sent, received) = mpsc::channel();
            operation.SetCompleted(&$handler::new(move |_, _| {
                sent.send(()).unwrap();
                Ok(())
            }))?;
            received.recv().unwrap();
            let barrier = Arc::new(Barrier::new(4));
            let values = std::thread::scope(|scope| {
                let workers: Vec<_> = (0..4)
                    .map(|_| {
                        let barrier = barrier.clone();
                        let operation = &operation;
                        scope.spawn(move || {
                            barrier.wait();
                            operation.GetResults()
                        })
                    })
                    .collect();
                workers
                    .into_iter()
                    .map(|worker| worker.join().unwrap())
                    .collect::<Vec<_>>()
            });
            let mut winners = 0;
            for value in values {
                match value {
                    Ok(value) => {
                        assert_eq!(value.value, 42);
                        assert_eq!(value.clones.get(), 0);
                        winners += 1;
                    }
                    Err(error) => assert_eq!(error.code(), HRESULT(0x8000000E_u32 as _)),
                }
            }
            assert_eq!(winners, 1);
        }};
    }

    check!(
        IAsyncOperation::ready(Ok(Value::new(42))),
        AsyncOperationCompletedHandler
    );
    check!(
        IAsyncOperation::spawn(|| Ok(Value::new(42))),
        AsyncOperationCompletedHandler
    );
    check!(
        IAsyncOperationWithProgress::<Value, IInspectable>::ready(Ok(Value::new(42))),
        AsyncOperationWithProgressCompletedHandler
    );
    check!(
        IAsyncOperationWithProgress::<Value, IInspectable>::spawn(|| Ok(Value::new(42))),
        AsyncOperationWithProgressCompletedHandler
    );
    Ok(())
}

#[test]
fn consumers_and_progress_do_not_require_send_results() -> Result<()> {
    fn consume(operation: IAsyncOperation<IInspectable>) {
        let future = operation.into_future();
        fn send_sync<T: Send + Sync>(_: &T) {}
        send_sync(&future);
    }
    let _: fn(IAsyncOperation<IInspectable>) = consume;
    IAsyncActionWithProgress::<IInspectable>::ready(Ok(())).join()?;
    IAsyncActionWithProgress::<IInspectable>::spawn(|| Ok(())).join()?;
    Ok(())
}
