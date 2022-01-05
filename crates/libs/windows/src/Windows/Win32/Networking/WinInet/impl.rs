pub trait IDialBrandingImpl: Sized {
    fn Initialize();
    fn GetBitmap();
}
pub trait IDialEngineImpl: Sized {
    fn Initialize();
    fn GetProperty();
    fn SetProperty();
    fn Dial();
    fn HangUp();
    fn GetConnectedState();
    fn GetConnectHandle();
}
pub trait IDialEventSinkImpl: Sized {
    fn OnEvent();
}
pub trait IProofOfPossessionCookieInfoManagerImpl: Sized {
    fn GetCookieInfoForUri();
}
pub trait IProofOfPossessionCookieInfoManager2Impl: Sized {
    fn GetCookieInfoWithUriForAccount();
}
