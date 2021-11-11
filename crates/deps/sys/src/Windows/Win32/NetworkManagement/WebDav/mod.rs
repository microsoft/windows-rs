#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AUTHNEXTSTEP();
    fn DAV_AUTHN_SCHEME_BASIC();
    fn DAV_AUTHN_SCHEME_CERT();
    fn DAV_AUTHN_SCHEME_DIGEST();
    fn DAV_AUTHN_SCHEME_FBA();
    fn DAV_AUTHN_SCHEME_NEGOTIATE();
    fn DAV_AUTHN_SCHEME_NTLM();
    fn DAV_AUTHN_SCHEME_PASSPORT();
    fn DAV_CALLBACK_AUTH_BLOB();
    fn DAV_CALLBACK_AUTH_UNP();
    fn DAV_CALLBACK_CRED();
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
    fn PFNDAVAUTHCALLBACK();
    fn PFNDAVAUTHCALLBACK_FREECRED();
}
