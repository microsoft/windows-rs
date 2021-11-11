#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BaseValueSource();
    fn BitmapDescription();
    fn CollectionElementValue();
    fn E_UNKNOWNTYPE();
    fn EnumType();
    fn IBitmapData();
    fn IVisualTreeService();
    fn IVisualTreeService2();
    fn IVisualTreeService3();
    fn IVisualTreeServiceCallback();
    fn IVisualTreeServiceCallback2();
    fn IXamlDiagnostics();
    fn InitializeXamlDiagnostic();
    fn InitializeXamlDiagnosticsEx();
    fn MetadataBit();
    fn ParentChildRelation();
    fn PropertyChainSource();
    fn PropertyChainValue();
    fn RenderTargetBitmapOptions();
    fn ResourceType();
    fn SourceInfo();
    fn VisualElement();
    fn VisualElementState();
    fn VisualMutationType();
}
