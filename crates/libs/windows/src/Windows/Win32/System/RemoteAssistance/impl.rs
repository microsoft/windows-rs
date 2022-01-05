#[cfg(feature = "Win32_System_Com")]
pub trait DRendezvousSessionEventsImpl: Sized + IDispatchImpl {}
pub trait IRendezvousApplicationImpl: Sized {
    fn SetRendezvousSession();
}
pub trait IRendezvousSessionImpl: Sized {
    fn State();
    fn RemoteUser();
    fn Flags();
    fn SendContextData();
    fn Terminate();
}
