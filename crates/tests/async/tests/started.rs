// The stock `ready` implementations are never in the `Started` state as they're either `Completed` and `Error`.
// This tests the `spawn` implementations to confirm that we can observe the `Started` state.
// The `GetResults` method may not be called in this state.

use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    let (send, recv) = std::sync::mpsc::channel::<()>();

    let a = IAsyncAction::spawn(move || {
        recv.recv().unwrap();
        Ok(())
    });

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    send.send(()).unwrap();
    a.get()?;

    Ok(())
}
