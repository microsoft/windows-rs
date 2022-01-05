#[cfg(feature = "implement_exclusive")]
pub trait IWindowManagementPreviewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowManagementPreviewStaticsImpl: Sized {
    fn SetPreferredMinSize(&self, window: &::core::option::Option<super::AppWindow>, preferredframeminsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
