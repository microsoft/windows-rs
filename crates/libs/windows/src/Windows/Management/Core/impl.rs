#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationDataManagerStaticsImpl: Sized {
    fn CreateForPackageFamily(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::ApplicationData>;
}
