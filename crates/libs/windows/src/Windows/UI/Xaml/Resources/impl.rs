#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CustomXamlResourceLoader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderOverridesImpl: Sized {
    fn GetResource(&self, resourceid: &::windows::core::HSTRING, objecttype: &::windows::core::HSTRING, propertyname: &::windows::core::HSTRING, propertytype: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomXamlResourceLoaderStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<CustomXamlResourceLoader>;
    fn SetCurrent(&self, value: &::core::option::Option<CustomXamlResourceLoader>) -> ::windows::core::Result<()>;
}
