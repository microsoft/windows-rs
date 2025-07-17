#[test]
fn collections() {
    _ = windows_collections::IIterable::<i32>::from(vec![1, 2, 3]);
}

#[test]
fn core() {
    _ = windows_core::Event::<windows_core::IUnknown>::new();
}

#[test]
fn future() {
    windows_future::IAsyncAction::ready(Ok(()));
}

#[test]
fn numerics() {
    windows_numerics::Vector2::zero().length();
}
