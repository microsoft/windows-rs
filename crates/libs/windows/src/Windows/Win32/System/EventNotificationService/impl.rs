#[cfg(feature = "Win32_System_Com")]
pub trait ISensLogonImpl: Sized + IDispatchImpl {
    fn Logon();
    fn Logoff();
    fn StartShell();
    fn DisplayLock();
    fn DisplayUnlock();
    fn StartScreenSaver();
    fn StopScreenSaver();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensLogon2Impl: Sized + IDispatchImpl {
    fn Logon();
    fn Logoff();
    fn SessionDisconnect();
    fn SessionReconnect();
    fn PostShell();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensNetworkImpl: Sized + IDispatchImpl {
    fn ConnectionMade();
    fn ConnectionMadeNoQOCInfo();
    fn ConnectionLost();
    fn DestinationReachable();
    fn DestinationReachableNoQOCInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISensOnNowImpl: Sized + IDispatchImpl {
    fn OnACPower();
    fn OnBatteryPower();
    fn BatteryLow();
}
