use windows::{Foundation::*, Win32::System::WinRT::*};

fn send<T: Send>(_: Option<T>) {}
fn sync<T: Sync>(_: Option<T>) {}

#[test]
fn test() {
    send(Option::<Uri>::None);
    sync(Option::<Uri>::None);

    send(Option::<IRestrictedErrorInfo>::None);
    sync(Option::<IRestrictedErrorInfo>::None);
}
