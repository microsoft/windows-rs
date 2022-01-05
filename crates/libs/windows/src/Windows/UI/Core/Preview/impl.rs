#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAppWindowPreviewStaticsImpl: Sized {
    fn GetIdFromWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationCloseRequestedPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerPreviewImpl: Sized {
    fn CloseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<SystemNavigationCloseRequestedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerPreviewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemNavigationManagerPreview>;
}
