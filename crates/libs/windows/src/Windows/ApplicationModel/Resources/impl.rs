#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderImpl: Sized {
    fn GetString(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoader2Impl: Sized {
    fn GetStringForUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderFactoryImpl: Sized {
    fn CreateResourceLoaderByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStaticsImpl: Sized {
    fn GetStringForReference(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForCurrentViewWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUse(&self) -> ::windows::core::Result<ResourceLoader>;
    fn GetForViewIndependentUseWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics3Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::super::UI::UIContext>) -> ::windows::core::Result<ResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceLoaderStatics4Impl: Sized {
    fn GetDefaultPriPath(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
