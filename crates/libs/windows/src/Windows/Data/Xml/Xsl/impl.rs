#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessorImpl: Sized {
    fn TransformToString(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessor2Impl: Sized {
    fn TransformToDocument(&self, inputnode: &::core::option::Option<super::Dom::IXmlNode>) -> ::windows::core::Result<super::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IXsltProcessorFactoryImpl: Sized {
    fn CreateInstance(&self, document: &::core::option::Option<super::Dom::XmlDocument>) -> ::windows::core::Result<XsltProcessor>;
}
