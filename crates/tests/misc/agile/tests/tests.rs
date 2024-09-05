use windows::{Foundation::*, Win32::System::WinRT::*};

fn send<T: Send>(_: Option<T>) {}
fn sync<T: Sync>(_: Option<T>) {}

#[test]
fn test() {
    send(Option::<Uri>::None);
    sync(Option::<Uri>::None);

    send(Option::<IRestrictedErrorInfo>::None);
    sync(Option::<IRestrictedErrorInfo>::None);

    send(Option::<IAsyncAction>::None);
    sync(Option::<IAsyncAction>::None);

    send(Option::<IAsyncActionWithProgress<i32>>::None);
    sync(Option::<IAsyncActionWithProgress<i32>>::None);

    send(Option::<IAsyncOperation<i32>>::None);
    sync(Option::<IAsyncOperation<i32>>::None);

    send(Option::<IAsyncOperationWithProgress<i32, i32>>::None);
    sync(Option::<IAsyncOperationWithProgress<i32, i32>>::None);
}
