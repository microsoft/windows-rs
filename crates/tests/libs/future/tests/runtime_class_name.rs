use windows_core::*;
use windows_future::*;

#[test]
fn async_operation_i32() {
    let a = IAsyncOperation::<i32>::ready(Ok(42));
    let inspectable: IInspectable = a.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.IAsyncOperation`1<Int32>");
}

#[test]
fn async_operation_string() {
    let a = IAsyncOperation::<HSTRING>::ready(Ok(HSTRING::from("hello")));
    let inspectable: IInspectable = a.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(&name, "Windows.Foundation.IAsyncOperation`1<String>");
}

#[test]
fn async_operation_with_progress_i32_i32() {
    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(42));
    let inspectable: IInspectable = a.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.IAsyncOperationWithProgress`2<Int32, Int32>"
    );
}

#[test]
fn async_action_with_progress_i32() {
    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    let inspectable: IInspectable = a.cast().unwrap();
    let name = inspectable.GetRuntimeClassName().unwrap();
    assert_eq!(
        &name,
        "Windows.Foundation.IAsyncActionWithProgress`1<Int32>"
    );
}
