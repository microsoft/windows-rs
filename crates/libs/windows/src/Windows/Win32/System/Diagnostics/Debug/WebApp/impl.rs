pub trait IWebApplicationActivationImpl: Sized {
    fn CancelPendingActivation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebApplicationAuthoringModeImpl: Sized + IServiceProviderImpl {
    fn AuthoringClientBinary();
}
pub trait IWebApplicationHostImpl: Sized {
    fn HWND();
    fn Document();
    fn Refresh();
    fn Advise();
    fn Unadvise();
}
pub trait IWebApplicationNavigationEventsImpl: Sized {
    fn BeforeNavigate();
    fn NavigateComplete();
    fn NavigateError();
    fn DocumentComplete();
    fn DownloadBegin();
    fn DownloadComplete();
}
pub trait IWebApplicationScriptEventsImpl: Sized {
    fn BeforeScriptExecute();
    fn ScriptError();
}
pub trait IWebApplicationUIEventsImpl: Sized {
    fn SecurityProblem();
}
pub trait IWebApplicationUpdateEventsImpl: Sized {
    fn OnPaint();
    fn OnCssChanged();
}
