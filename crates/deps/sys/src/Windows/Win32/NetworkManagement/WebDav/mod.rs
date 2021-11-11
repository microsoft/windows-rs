#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DavAddConnection();
    fn DavCancelConnectionsToServer();
    fn DavDeleteConnection();
    fn DavFlushFile();
    fn DavGetExtendedError();
    fn DavGetHTTPFromUNCPath();
    fn DavGetTheLockOwnerOfTheFile();
    fn DavGetUNCFromHTTPPath();
    fn DavInvalidateCache();
    fn DavRegisterAuthCallback();
    fn DavUnregisterAuthCallback();
}
