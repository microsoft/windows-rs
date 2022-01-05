#[cfg(feature = "implement_exclusive")]
pub trait IInputActivationListenerPreviewStaticsImpl: Sized {
    fn CreateForApplicationWindow(&self, window: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<super::InputActivationListener>;
}
