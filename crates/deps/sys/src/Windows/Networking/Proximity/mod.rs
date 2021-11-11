#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ConnectionRequestedEventArgs();
    fn DeviceArrivedEventHandler();
    fn DeviceDepartedEventHandler();
    fn IConnectionRequestedEventArgs();
    fn IPeerFinderStatics();
    fn IPeerFinderStatics2();
    fn IPeerInformation();
    fn IPeerInformation3();
    fn IPeerInformationWithHostAndService();
    fn IPeerWatcher();
    fn IProximityDevice();
    fn IProximityDeviceStatics();
    fn IProximityMessage();
    fn ITriggeredConnectionStateChangedEventArgs();
    fn MessageReceivedHandler();
    fn MessageTransmittedHandler();
    fn PeerDiscoveryTypes();
    fn PeerFinder();
    fn PeerInformation();
    fn PeerRole();
    fn PeerWatcher();
    fn PeerWatcherStatus();
    fn ProximityDevice();
    fn ProximityMessage();
    fn TriggeredConnectState();
    fn TriggeredConnectionStateChangedEventArgs();
}
