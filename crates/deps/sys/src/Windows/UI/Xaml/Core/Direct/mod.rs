#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IXamlDirect();
    fn IXamlDirectObject();
    fn IXamlDirectStatics();
    fn XamlDirect();
    fn XamlDirectContract();
    fn XamlEventIndex();
    fn XamlPropertyIndex();
    fn XamlTypeIndex();
}
