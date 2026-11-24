use std::cell::Cell;
use std::future::IntoFuture;
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

#[derive(Clone, Default)]
struct Value(Cell<i32>);

impl imp::TypeKind for Value {
    type TypeKind = imp::CloneType;
}

impl RuntimeType for Value {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"struct(Test.Value;i4)");
}

#[test]
fn spawned_result_needs_send_but_not_sync() -> Result<()> {
    let operation = IAsyncOperation::spawn(|| Ok(Value(Cell::new(42))));
    assert_eq!(operation.join()?.0.get(), 42);
    let operation =
        IAsyncOperationWithProgress::<Value, IInspectable>::spawn(|| Ok(Value(Cell::new(43))));
    assert_eq!(operation.join()?.0.get(), 43);
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
