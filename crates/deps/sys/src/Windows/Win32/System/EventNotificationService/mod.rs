#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CONNECTION_AOL();
    fn ISensLogon();
    fn ISensLogon2();
    fn ISensNetwork();
    fn ISensOnNow();
    fn IsDestinationReachableA();
    fn IsDestinationReachableW();
    fn IsNetworkAlive();
    fn NETWORK_ALIVE_AOL();
    fn NETWORK_ALIVE_INTERNET();
    fn NETWORK_ALIVE_LAN();
    fn NETWORK_ALIVE_WAN();
    fn QOCINFO();
    fn SENS();
    fn SENSGUID_EVENTCLASS_LOGON();
    fn SENSGUID_EVENTCLASS_LOGON2();
    fn SENSGUID_EVENTCLASS_NETWORK();
    fn SENSGUID_EVENTCLASS_ONNOW();
    fn SENSGUID_PUBLISHER();
    fn SENSGUID_SUBSCRIBER_LCE();
    fn SENSGUID_SUBSCRIBER_WININET();
    fn SENS_CONNECTION_TYPE();
    fn SENS_QOCINFO();
}
