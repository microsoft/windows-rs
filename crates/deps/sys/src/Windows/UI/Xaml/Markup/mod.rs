#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IComponentConnector();
    fn IComponentConnector2();
    fn IDataTemplateComponent();
    fn IMarkupExtension();
    fn IMarkupExtensionFactory();
    fn IMarkupExtensionOverrides();
    fn IXamlBinaryWriter();
    fn IXamlBinaryWriterStatics();
    fn IXamlBindScopeDiagnostics();
    fn IXamlBindingHelper();
    fn IXamlBindingHelperStatics();
    fn IXamlMarkupHelper();
    fn IXamlMarkupHelperStatics();
    fn IXamlMember();
    fn IXamlMetadataProvider();
    fn IXamlReader();
    fn IXamlReaderStatics();
    fn IXamlType();
    fn IXamlType2();
    fn MarkupExtension();
    fn XamlBinaryWriter();
    fn XamlBinaryWriterErrorInformation();
    fn XamlBindingHelper();
    fn XamlMarkupHelper();
    fn XamlReader();
    fn XmlnsDefinition();
}
